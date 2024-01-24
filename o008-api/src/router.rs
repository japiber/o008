use axum::Router;
use axum::routing::{get, put};
use crate::handler;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(
    paths(
        handler::service_get,
        handler::service_put,
        handler::service_version_put,
        handler::service_versions_get,
    ),
    components(
        schemas(
            o008_entity::Application,
            o008_entity::Builder,
            o008_entity::Service,
            o008_entity::ServiceVersion,
            o008_entity::ServiceVersionItem,
            o008_entity::RepoReference,
            o008_entity::Tenant,
            o008_common::BuilderRequest,
            o008_common::TenantRequest,
            o008_common::RepoReferenceKind,
            o008_common::ApplicationRequest,
            o008_common::RepoReferenceRequest,
            o008_common::ServiceRequest,
            o008_common::ServiceVersionRequest,
        ),
    )
)]
struct ApiDocV1;

pub fn router_o008_v1() -> Router {
    Router::new()
        .route("/service/:service/app/:app/tenant/:tenant", get(handler::service_get))
        .route("/service/:service/app/:app/tenant/:tenant", put(handler::service_put))
        .route("/service/:service/app/:app/tenant/:tenant/version/:version", put(handler::service_version_put))
        .route("/service/:service/app/:app/tenant/:tenant/versions", get(handler::service_versions_get))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDocV1::openapi()))
}