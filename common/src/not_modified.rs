use std::collections::BTreeMap;

use axum::response::IntoResponse;
use derive_more::Display;
use utoipa::IntoResponses;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Default)]
pub struct NotModified;

impl IntoResponse for NotModified {
    fn into_response(self) -> axum::response::Response {
        axum::http::StatusCode::NOT_MODIFIED.into_response()
    }
}

impl IntoResponses for NotModified {
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        BTreeMap::from([(
            axum::http::StatusCode::NOT_MODIFIED.as_str().to_string(),
            utoipa::openapi::response::Response::new("Not modified").into(),
        )])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
pub enum ResponseOrNotModified<T> {
    Response(T),
    NotModified,
}

impl<T> IntoResponse for ResponseOrNotModified<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            ResponseOrNotModified::Response(response) => response.into_response(),
            ResponseOrNotModified::NotModified => NotModified.into_response(),
        }
    }
}

impl<T> IntoResponses for ResponseOrNotModified<T>
where
    T: IntoResponses,
{
    fn responses() -> std::collections::BTreeMap<
        String,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    > {
        T::responses()
            .into_iter()
            .chain(NotModified::responses())
            .collect()
    }
}
