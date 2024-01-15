use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::{to_value};
use o008_common::{DispatchCommand, ServiceRequest};
use o008_common::AppCommand;
use o008_entity::{QueryEntity, Service};
use o008_message_bus::{launch_response_poll, RequestMessage, send_request};
use crate::handler::dispatch_error_into_response;


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

    let msg = RequestMessage::new(DispatchCommand::from(AppCommand::GetService { value: req }));
    let tr = launch_response_poll(msg.id());
    send_request(msg);
    match tr.await.unwrap() {
        None => (StatusCode::NO_CONTENT, "").into_response(),
        Some(msg) => match msg.response() {
            Ok(srv) => (StatusCode::OK, Json(srv)).into_response(),
            Err(e) => dispatch_error_into_response(e)
        }
    }
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
    let req = ServiceRequest::build_get_request(name, application, tenant);
    let msg = if Service::persisted(to_value(req.clone()).unwrap()).await {
        RequestMessage::new(DispatchCommand::from(AppCommand::UpdateService { source: req.clone(), value: payload }))
    } else {
        RequestMessage::new(DispatchCommand::from(AppCommand::CreateService { value: payload }))
    };
    let tr = launch_response_poll(msg.id());
    send_request(msg);
    match tr.await.unwrap() {
        None => (StatusCode::NO_CONTENT, "").into_response(),
        Some(msg) => match msg.response() {
            Ok(srv) => (StatusCode::ACCEPTED, Json(srv)).into_response(),
            Err(e) => dispatch_error_into_response(e)
        }
    }
}