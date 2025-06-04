//! Parsing of headers set by nginx to analyze the original request

use apelle_common::Reporter;
use axum::{
    extract::FromRequestParts,
    http::{
        HeaderName, Method, StatusCode, header::ToStrError, method::InvalidMethod, request::Parts,
    },
    response::IntoResponse,
};
use snafu::{OptionExt, ResultExt, Snafu};

pub const URI_HEADER: HeaderName = HeaderName::from_static("x-original-uri");
pub const METHOD_HEADER: HeaderName = HeaderName::from_static("x-original-method");

#[derive(Debug, Clone)]
pub struct OriginHeaders {
    pub uri: String,
    pub method: Method,
}

#[derive(Debug, Snafu)]
pub enum OriginHeadersRejection {
    UriMissing,
    MethodMissing,
    UriNotUtf8 { source: ToStrError },
    InvalidMethod { source: InvalidMethod },
}

impl IntoResponse for OriginHeadersRejection {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Failed to parse origin headers: {}", Reporter(self));
        // This is not a public API, so it is always a internal error
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<S: Sync> FromRequestParts<S> for OriginHeaders {
    type Rejection = OriginHeadersRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let uri = parts.headers.get(URI_HEADER).context(UriMissingSnafu)?;
        let method = parts
            .headers
            .get(METHOD_HEADER)
            .context(MethodMissingSnafu)?;

        let uri = uri.to_str().context(UriNotUtf8Snafu)?.to_owned();
        let method = Method::from_bytes(method.as_bytes()).context(InvalidMethodSnafu)?;

        Ok(Self { uri, method })
    }
}
