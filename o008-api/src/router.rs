use axum::Router;
use axum::routing::{get, put};
use crate::handler;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use o008_entity::{Application, Builder, Service, Tenant};


#[derive(OpenApi)]
#[openapi(
    paths(
        handler::get_service,
        handler::create_or_update
    ),
    components(
        schemas(
            Application,
            Builder,
            Service,
            Tenant,
            o008_common::TenantRequest,
            o008_common::ApplicationRequest,
            o008_common::ServiceRequest
        ),
    )
)]
struct ApiDocV1;

pub fn router_o008_v1() -> Router {
    Router::new()
        .route("/service/:service/app/:app/tenant/:tenant", get(handler::get_service))
        .route("/service/:service/app/:app/tenant/:tenant", put(handler::create_or_update))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDocV1::openapi()))
}