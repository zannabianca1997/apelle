use std::convert::Infallible;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::HeaderValue,
};
use reqwest::{IntoUrl, Method, Request, RequestBuilder, Response};

use crate::main_wrapper::TRACE_ID_HEADER;

/// A wrapper around reqwest::Client that adds tracing headers
#[derive(Debug, Clone)]
pub struct TracingClient {
    client: reqwest::Client,
    trace_id: HeaderValue,
}

impl<S> FromRequestParts<S> for TracingClient
where
    reqwest::Client: FromRef<S>,
    S: Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(TracingClient {
            client: reqwest::Client::from_ref(state),
            trace_id: parts
                .headers
                .get(TRACE_ID_HEADER)
                .cloned()
                .unwrap_or_else(generate_trace_id),
        })
    }
}

fn generate_trace_id() -> HeaderValue {
    uuid::Uuid::new_v4().to_string().parse().unwrap()
}

macro_rules! convenience {
    ($($method:ident)*) => {
      $(
        pub fn $method<U: IntoUrl>(&self, url: U) -> RequestBuilder {
            self.with_trace_id(self.client.$method(url))
        }
        )*
    };
}

impl TracingClient {
    pub fn new() -> Self {
        TracingClient {
            client: reqwest::Client::new(),
            trace_id: generate_trace_id(),
        }
    }

    convenience! {
        get
        post
        put
        delete
        head
        patch
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.with_trace_id(self.client.request(method, url))
    }

    pub fn execute(
        &self,
        mut request: Request,
    ) -> impl Future<Output = Result<Response, reqwest::Error>> {
        request
            .headers_mut()
            .append(TRACE_ID_HEADER, self.trace_id.clone());

        self.client.execute(request)
    }

    fn with_trace_id(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.header(TRACE_ID_HEADER, self.trace_id.clone())
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn trace_id(&self) -> &HeaderValue {
        &self.trace_id
    }
}
