use axum::Router;
use axum::routing::get;
use crate::handler;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use o008_entity::{Application, Builder, Service, Tenant};

#[derive(OpenApi)]
#[openapi(
    paths(handler::get_service),
    components(
        schemas(Application),
        schemas(Builder),
        schemas(Service),
        schemas(Tenant),
    )
)]
struct ApiDocV1;

pub fn router_o008_v1() -> Router {
    Router::new()
        .route("/service/:service/app/:app/tenant/:tenant", get(handler::get_service))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDocV1::openapi()))
}