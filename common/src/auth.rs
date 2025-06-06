use std::convert::Infallible;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{HeaderName, HeaderValue, StatusCode, header::InvalidHeaderValue, request::Parts},
    response::{IntoResponse, IntoResponseParts},
};
use itertools::Itertools;
use snafu::{OptionExt, ResultExt, Snafu};
use uuid::Uuid;

use crate::Reporter;

pub const ID_HEADER: HeaderName = HeaderName::from_static("x-apelle-user-id");
pub const NAME_HEADER: HeaderName = HeaderName::from_static("x-apelle-user-name");
pub const GLOBAL_ROLES_HEADER: HeaderName = HeaderName::from_static("x-apelle-user-global-roles");

#[derive(Debug, Snafu)]
pub enum InvalidHeaders {
    InvalidName { source: InvalidHeaderValue },
    InvalidRoles { source: InvalidHeaderValue },
}

#[derive(Debug, Clone)]
pub struct AuthHeaders {
    // Uuid are lightweight and so validated as soon as possible
    id: Uuid,
    // Instead we try to avoid validating the name and roles until it is needed
    name: HeaderValue,
    roles: Option<HeaderValue>,
}

impl AuthHeaders {
    pub fn new<'a>(
        id: Uuid,
        name: &'a str,
        roles: impl IntoIterator<Item = &'a (impl AsRef<str> + 'a)>,
    ) -> Result<Self, InvalidHeaders> {
        let roles_joined: String =
            Itertools::intersperse(roles.into_iter().map(|r| r.as_ref()), ",").collect();
        Ok(Self {
            id,
            name: HeaderValue::from_str(name).context(InvalidNameSnafu)?,
            roles: (!roles_joined.is_empty())
                .then(|| HeaderValue::from_str(&roles_joined))
                .transpose()
                .context(InvalidRolesSnafu)?,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        str::from_utf8(self.name.as_bytes()).expect("All constructors checks this is valid")
    }

    pub fn roles(&self) -> impl IntoIterator<Item = &str> {
        self.roles
            .as_ref()
            .map(|r| r.to_str().expect("All constructors checks this is valid"))
            .unwrap_or("")
            .split(',')
    }
}

#[derive(Debug, Snafu)]
pub enum AuthHeadersRejection {
    IdMissing,
    NameMissing,
    RolesMissing,
    NameNotUtf8 { source: std::str::Utf8Error },
    InvalidUuid { source: uuid::Error },
    RolesNotUtf8 { source: std::str::Utf8Error },
}

impl IntoResponse for AuthHeadersRejection {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Failed to parse auth headers: {}", Reporter(self));
        // This is not a public API, so it is always a internal error
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<S: Sync> FromRequestParts<S> for AuthHeaders {
    type Rejection = AuthHeadersRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let id = parts.headers.get(ID_HEADER).context(IdMissingSnafu)?;
        let name = parts.headers.get(NAME_HEADER).context(NameMissingSnafu)?;
        let roles = parts.headers.get(GLOBAL_ROLES_HEADER);

        str::from_utf8(name.as_bytes()).context(NameNotUtf8Snafu)?;
        if let Some(roles) = roles {
            str::from_utf8(roles.as_bytes()).context(RolesNotUtf8Snafu)?;
        }

        Ok(Self {
            id: Uuid::try_parse_ascii(id.as_bytes()).context(InvalidUuidSnafu)?,
            name: name.clone(),
            roles: roles.cloned(),
        })
    }
}

impl<S: Sync> OptionalFromRequestParts<S> for AuthHeaders {
    type Rejection = AuthHeadersRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let id = parts.headers.get(ID_HEADER);
        let name = parts.headers.get(NAME_HEADER);
        let roles = parts.headers.get(GLOBAL_ROLES_HEADER);

        let (Some(id), Some(name)) = (id, name) else {
            // No auth headers
            return Ok(None);
        };

        str::from_utf8(name.as_bytes()).context(NameNotUtf8Snafu)?;
        if let Some(roles) = roles {
            str::from_utf8(roles.as_bytes()).context(RolesNotUtf8Snafu)?;
        }

        Ok(Some(Self {
            id: Uuid::try_parse_ascii(id.as_bytes()).context(InvalidUuidSnafu)?,
            name: name.clone(),
            roles: roles.cloned(),
        }))
    }
}

impl IntoResponseParts for AuthHeaders {
    type Error = Infallible;

    fn into_response_parts(
        self,
        res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        (
            [
                (
                    ID_HEADER,
                    HeaderValue::from_str(&self.id.to_string()).unwrap(),
                ),
                (NAME_HEADER, self.name),
            ],
            self.roles.map(|roles| [(GLOBAL_ROLES_HEADER, roles)]),
        )
            .into_response_parts(res)
            .map_err(|_| unreachable!())
    }
}
