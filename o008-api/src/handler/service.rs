use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use o008_common::{DispatchCommand, ServiceRequest};
use o008_common::AppCommand;
use o008_message_bus::{RequestMessage};
use crate::handler::{message_into_response};


/// Get Service item by service name, application name and tenant name
///
/// Get Service item by name, application and tenant. Return status 200 on success or 404 if Service is not found.
#[utoipa::path(
    get,
    path = "/service/{service}/app/{app}/tenant/{tenant}",
    responses(
        (status = 200, description = "Get service done successfully", body = Service),
        (status = 404, description = "Service not found")
    ),
    params(
        ("service" = String, Path, description = "Service name"),
        ("app" = String, Path, description = "Service application name"),
        ("tenant" = String, Path, description = "Service tenant name"),
    )
)]
pub async fn service_get(Path((name, application, tenant)): Path<(String, String, String)>) -> impl IntoResponse {
    let req = ServiceRequest::build_get_request(name, application, tenant);
    let msg = RequestMessage::new(DispatchCommand::from(AppCommand::GetService { request: req }));
    message_into_response(msg, StatusCode::OK).await
}

/// Get Service (with its versions) item by service name, application name and tenant name
///
/// Get Service (with its versions) item by name, application and tenant. Return status 200 on success or 404 if Service is not found.
#[utoipa::path(
get,
path = "/service/{service}/app/{app}/tenant/{tenant}/versions",
responses(
(status = 200, description = "Get service done successfully", body = Service),
(status = 404, description = "Service not found")
),
params(
("service" = String, Path, description = "Service name"),
("app" = String, Path, description = "Service application name"),
("tenant" = String, Path, description = "Service tenant name"),
)
)]
pub async fn service_versions_get(Path((name, application, tenant)): Path<(String, String, String)>) -> impl IntoResponse {
    let req = ServiceRequest::build_get_request(name, application, tenant);
    let msg = RequestMessage::new(DispatchCommand::from(AppCommand::GetServiceVersions { request: req }));
    message_into_response(msg, StatusCode::OK).await
}


/// Create or Update Service item by service name, application name and tenant name
///
/// Create or Update Service item by name, application and tenant. Return status 200 on success or 404 if Service is not found.
#[utoipa::path(
    put,
    path = "/service/{service}/app/{app}/tenant/{tenant}",
    request_body = ServiceRequest,
    responses(
        (status = 200, description = "create/update service done successfully", body = Service),
        (status = 404, description = "Service not found")
    ),
    params(
        ("service" = String, Path, description = "Service name"),
        ("app" = String, Path, description = "Service application name"),
        ("tenant" = String, Path, description = "Service tenant name"),
    )
)]
pub async fn service_put(Path((name, application, tenant)): Path<(String, String, String)>,
                         Json(payload) : Json<ServiceRequest>) -> impl IntoResponse {
    let source = ServiceRequest::build_get_request(name, application, tenant);
    let msg = RequestMessage::new(DispatchCommand::from(AppCommand::PersistService { source, request: payload }));
    message_into_response(msg, StatusCode::ACCEPTED).await
}