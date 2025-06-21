use std::time::Duration;

use axum::{Router, body::Bytes, debug_handler, extract::State};
use axum_extra::{
    TypedHeader,
    headers::{CacheControl, ContentType},
};
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
use utoipa::openapi::{OpenApi, PathItem, RefOr, Tag, path::Operation};
use utoipa_axum::{router::OpenApiRouter, routes};

const PUBLIC_TAG: &str = "public";
const SERVICE_TAG: &str = "service";

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Cannot serialize OpenApiDocument"))]
    Serialize { source: serde_json::Error },
}

#[debug_handler(state=&'static str)]
#[utoipa::path(get, path = "/name",
    responses((status = StatusCode::OK, description = "Name of the service: `{name}`", content_type = "text/plain",))
)]
/// Name of the service
///
/// Returns the name of the service: `{name}`
async fn name(State(name): State<&'static str>) -> (TypedHeader<CacheControl>, &'static str) {
    (
        TypedHeader(
            // This endpoint mainly serve as a check that the right service is
            // enabled, so caching it would lead to unexpected behavior
            CacheControl::new().with_no_cache(),
        ),
        name,
    )
}
#[debug_handler(state=Bytes)]
#[utoipa::path(get, path = "/openapi.json", 
    request_body(),
    responses((status = StatusCode::OK, description = "OpenApi document", content_type = "application/json",))
)]
/// API documentation
///
/// Returns the api exposed by the service
async fn openapi(
    State(openapi): State<Bytes>,
) -> (TypedHeader<CacheControl>, TypedHeader<ContentType>, Bytes) {
    (
        TypedHeader(
            CacheControl::new()
                .with_immutable()
                .with_max_age(Duration::from_secs(31536000)),
        ),
        TypedHeader(ContentType::json()),
        openapi,
    )
}

pub fn add_service_endpoint(
    service_name: &'static str,
    app: OpenApiRouter,
) -> Result<Router, Error> {
    let mut service_endpoint = OpenApiRouter::new()
        .routes(routes!(name))
        .with_state(service_name)
        .routes(routes!(openapi));

    let name_op = service_endpoint
        .get_openapi_mut()
        .paths
        .paths
        .get_mut("/name")
        .unwrap()
        .get
        .as_mut()
        .unwrap();
    name_op
        .description
        .as_mut()
        .map(|d| *d = d.replace("{name}", service_name));
    let RefOr::T(name_resp) = name_op
        .responses
        .responses
        .get_mut(StatusCode::OK.as_str())
        .unwrap()
    else {
        panic!()
    };
    name_resp.description = name_resp.description.replace("{name}", service_name);
    name_resp
        .content
        .entry("text/plain".to_string())
        .or_default()
        .example = Some(service_name.into());

    tag_api(
        "",
        service_endpoint.get_openapi_mut(),
        Tag::builder()
            .name(SERVICE_TAG)
            .description(Some(
                "Endpoints dedicated to auxiliary infos for the service",
            ))
            .build(),
    );

    let (app, mut openapi) = app
        .with_state(())
        .nest("/service", service_endpoint)
        .split_for_parts();

    tag_api(
        "",
        &mut openapi,
        Tag::builder()
            .name(format!("service-{service_name}"))
            .description(Some(format!(
                "Endpoints served by the `{service_name}` service",
            )))
            .build(),
    );

    tag_api(
        "/public",
        &mut openapi,
        Tag::builder()
            .name(PUBLIC_TAG)
            .description(Some("Endpoints available to the end user"))
            .build(),
    );

    let openapi = openapi.to_json().context(SerializeSnafu)?.into();

    Ok(app.with_state(openapi))
}

fn tag_api(paths_prefix: &'static str, openapi: &mut OpenApi, tag: Tag) {
    openapi
        .paths
        .paths
        .iter_mut()
        .filter_map(|(path, path_item)| path.starts_with(paths_prefix).then_some(path_item))
        .flat_map(iter_operations)
        .for_each(|op| op.tags.get_or_insert_default().push(tag.name.clone()));

    openapi.tags.get_or_insert_default().push(tag);
}

fn iter_operations(
    PathItem {
        get,
        put,
        post,
        delete,
        options,
        head,
        patch,
        trace,
        ..
    }: &mut PathItem,
) -> impl Iterator<Item = &mut Operation> {
    [
        get.as_mut(),
        put.as_mut(),
        post.as_mut(),
        delete.as_mut(),
        options.as_mut(),
        head.as_mut(),
        patch.as_mut(),
        trace.as_mut(),
    ]
    .into_iter()
    .flatten()
}
