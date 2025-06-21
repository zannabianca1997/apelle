use std::{collections::HashMap, sync::Arc, time::Duration};

use apelle_common::{PUBLIC_TAG, SERVICE_TAG, TracingClient, iter_operations};
use axum::{
    Json, Router,
    body::Body,
    debug_handler,
    extract::{FromRef, Path, State},
    http::StatusCode,
    response::{IntoResponse as _, Response},
};
use axum_extra::{TypedHeader, headers::CacheControl};
use config::Config;
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt, stream::FuturesUnordered};
use snafu::Snafu;
use utoipa::{OpenApi, openapi};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::{Config as SwaggerConfig, SwaggerUi};

use crate::config::Service;

pub mod config;

/// Main fatal error
#[derive(Debug, Snafu)]
pub enum MainError {}

#[derive(Clone, FromRef)]
struct App {
    client: reqwest::Client,
    services: Arc<HashMap<String, Service>>,
}

#[derive(OpenApi)]
struct AppApi;

#[debug_handler(state=App)]
#[utoipa::path(get, path = "/api-docs/{name}/openapi.json",
    request_body(),
    responses(
        (status = StatusCode::OK, description = "OpenApi document", content_type = "application/json"),
        (status = StatusCode::NOT_FOUND, description = "Service not found")
    )
)]
/// Service api docs
///
/// Returns the api exposed by a specific service
async fn service(
    State(services): State<Arc<HashMap<String, Service>>>,
    client: TracingClient,
    Path(name): Path<String>,
) -> Response {
    // Get the service url
    let Some(Service { url, .. }) = services.get(&name.to_lowercase().replace('_', "-")) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let response = match client
        .get(url.join("service/openapi.json").unwrap())
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(service = name, "Error in connecting to the service: {err}");

            return (
                StatusCode::BAD_GATEWAY,
                "Error in connecting to the service",
            )
                .into_response();
        }
    };

    let mut response_builder = Response::builder().status(response.status());
    *response_builder.headers_mut().unwrap() = response.headers().clone();
    response_builder
        .body(Body::from_stream(response.bytes_stream()))
        .unwrap()
}

#[derive(OpenApi)]
#[openapi(info(
    title = "Apelle",
    version = "2.0.0-alpha",
    description = "
> A communist music queue

`apelle` is a service for handling a shared music queue. Users can insert songs
in the queues, and upvote them to push them upward. `apelle` will track the
position of each song in the queue, and the position of the currently playing
song.

It also fetch the song data from the sources (for now, only Youtube is
supported). Users provides only the minimal necessary to identify the song (e.g.
the youtube video ID).
        ",
    license(name = "MIT")
))]
struct PublicApiDocs;

#[debug_handler(state=App)]
#[utoipa::path(get, path = "/public/openapi.json",
    request_body(),
    responses(
        (status = StatusCode::OK, description = "OpenApi document", content_type = "application/json")
    )
)]
/// Service api docs
///
/// Returns the api exposed by a specific service
async fn public(
    State(services): State<Arc<HashMap<String, Service>>>,
    client: TracingClient,
) -> (TypedHeader<CacheControl>, Json<openapi::OpenApi>) {
    // Fetch all the docs for every service that's exposed
    let mut openapi = services
        .iter()
        .filter_map(|(name, Service { url, public })| {
            let public = public.as_deref()?;

            Some(
                client
                    .get(url.join("service/openapi.json").unwrap())
                    .send()
                    .map(|r| r.and_then(|r| r.error_for_status()))
                    .and_then(|r| r.json::<openapi::OpenApi>())
                    .map_ok(move |docs| (name.as_str(), public, docs))
                    .map_err(move |err| (name.as_str(), err)),
            )
        })
        .collect::<FuturesUnordered<_>>()
        .filter_map(async |r| match r {
            Ok(r) => Some(r),
            Err((service, err)) => {
                // Don't fail if a service is down, show only the ones that are up
                tracing::error!(service, "Error in connecting to the service: {err}");
                None
            }
        })
        .fold(
            PublicApiDocs::openapi(),
            async |docs, (service, public, mut service_docs)| {
                tracing::debug!(service, public, "Unnesting api docs");

                service_docs.paths.paths = service_docs
                    .paths
                    .paths
                    .into_iter()
                    .filter_map(|(path, item)| {
                        // Taking only the public enpoints, and replacing the prefix
                        // with the public path
                        let path = path.strip_prefix("/public")?;
                        Some((format!("{public}{path}"), item))
                    })
                    .collect();

                docs.merge_from(service_docs)
            },
        )
        .await;

    // Remove the public tag, and all tags that are not used
    openapi.tags.as_mut().map(|tags| {
        tags.retain(|t| {
            t.name != PUBLIC_TAG
                && openapi
                    .paths
                    .paths
                    .values()
                    .flat_map(iter_operations)
                    .any(|op| {
                        op.tags
                            .as_ref()
                            .map(Vec::as_slice)
                            .unwrap_or_default()
                            .contains(&t.name)
                    })
        });
    });

    (
        TypedHeader(
            CacheControl::new()
                .with_immutable()
                .with_max_age(Duration::from_secs(31536000)),
        ),
        Json(openapi),
    )
}

pub async fn app(Config { services }: Config) -> Result<OpenApiRouter, MainError> {
    let client = reqwest::Client::new();

    let services: HashMap<String, Service> = services
        .into_iter()
        .map(|(name, service)| (name.to_lowercase().replace('_', "-"), service))
        .collect();

    let ui = SwaggerUi::new("/swagger-ui").config(
        SwaggerConfig::new(
            services
                .keys()
                .map(|s| format!("/api-docs/{s}/openapi.json")),
        )
        // Disable the try it out button, as the services does not support CORS
        .supported_submit_methods(Vec::<String>::new()),
    );
    let external_ui = SwaggerUi::new("/external-swagger-ui/")
        .config(SwaggerConfig::new(vec!["/api-docs/openapi.json"]));

    let swagger_router = Router::new().merge(ui).merge(external_ui);

    Ok(OpenApiRouter::with_openapi(AppApi::openapi())
        .routes(routes!(service))
        .routes(routes!(public))
        .with_state(App {
            client,
            services: Arc::new(services),
        })
        .merge(swagger_router.into()))
}
