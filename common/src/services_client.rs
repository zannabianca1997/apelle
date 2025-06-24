use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::HeaderValue,
};
use reqwest::{IntoUrl, Method, Request, RequestBuilder, Response};

use crate::{AuthHeaders, main_wrapper::TRACE_ID_HEADER};

/// Client for intra-microservice requests
///
/// A wrapper around reqwest::Client that propagates the trace id and
/// authentication headers
#[derive(Debug, Clone)]
pub struct ServicesClient {
    client: reqwest::Client,
    trace_id: HeaderValue,
    auth: Option<AuthHeaders>,
}

impl<S> FromRequestParts<S> for ServicesClient
where
    reqwest::Client: FromRef<S>,
    S: Sync + Send,
{
    type Rejection = <AuthHeaders as OptionalFromRequestParts<S>>::Rejection;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(ServicesClient {
            client: reqwest::Client::from_ref(state),
            trace_id: parts
                .headers
                .get(TRACE_ID_HEADER)
                .cloned()
                .unwrap_or_else(generate_trace_id),
            auth: Option::<AuthHeaders>::from_request_parts(parts, state).await?,
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
            self.wrap(self.client.$method(url))
        }
        )*
    };
}

impl Default for ServicesClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ServicesClient {
    pub fn new() -> Self {
        ServicesClient {
            client: reqwest::Client::new(),
            trace_id: generate_trace_id(),
            auth: None,
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
        self.wrap(self.client.request(method, url))
    }

    pub fn execute(
        &self,
        mut request: Request,
    ) -> impl Future<Output = Result<Response, reqwest::Error>> {
        request
            .headers_mut()
            .append(TRACE_ID_HEADER, self.trace_id.clone());

        if let Some(auth) = &self.auth {
            request.headers_mut().extend(auth.headers());
        }

        self.client.execute(request)
    }

    fn wrap(&self, builder: RequestBuilder) -> RequestBuilder {
        let mut builder = builder.header(TRACE_ID_HEADER, self.trace_id.clone());
        if let Some(auth) = &self.auth {
            for (header, value) in auth.headers() {
                builder = builder.header(header, value);
            }
        }
        builder
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn trace_id(&self) -> &HeaderValue {
        &self.trace_id
    }
}
