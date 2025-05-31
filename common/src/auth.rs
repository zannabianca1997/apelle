use std::convert::Infallible;

use axum::{
    extract::FromRequestParts,
    http::{HeaderName, HeaderValue, StatusCode, header::InvalidHeaderValue, request::Parts},
    response::{IntoResponse, IntoResponseParts},
};
use snafu::{OptionExt, ResultExt, Snafu};
use uuid::Uuid;

use crate::Reporter;

pub const ID_HEADER: HeaderName = HeaderName::from_static("x-apelle-user-id");
pub const NAME_HEADER: HeaderName = HeaderName::from_static("x-apelle-user-name");

#[derive(Debug, Snafu)]
pub struct InvalidName {
    source: InvalidHeaderValue,
}

#[derive(Debug, Clone)]
pub struct AuthHeaders {
    // Uuid are lightweight and so validated as soon as possible
    id: Uuid,
    // Instead we try to avoid validating the name until it is needed
    name: HeaderValue,
}

impl AuthHeaders {
    pub fn new(id: Uuid, name: &str) -> Result<Self, InvalidName> {
        Ok(Self {
            id,
            name: HeaderValue::from_bytes(name.as_bytes()).context(InvalidNameSnafu)?,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum AuthHeadersRejection {
    IdMissing,
    NameMissing,
    NameNotUtf8 { source: std::str::Utf8Error },
    InvalidUuid { source: uuid::Error },
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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let id = parts.headers.get(ID_HEADER).context(NameMissingSnafu)?;
        let name = parts.headers.get(NAME_HEADER).context(IdMissingSnafu)?;

        str::from_utf8(name.as_bytes()).context(NameNotUtf8Snafu)?;

        Ok(Self {
            id: Uuid::try_parse_ascii(id.as_bytes()).context(InvalidUuidSnafu)?,
            name: name.clone(),
        })
    }
}

impl IntoResponseParts for AuthHeaders {
    type Error = Infallible;

    fn into_response_parts(
        self,
        res: axum::response::ResponseParts,
    ) -> Result<axum::response::ResponseParts, Self::Error> {
        [
            (
                ID_HEADER,
                HeaderValue::from_str(&self.id.to_string()).unwrap(),
            ),
            (NAME_HEADER, self.name),
        ]
        .into_response_parts(res)
        .map_err(|_| unreachable!())
    }
}
