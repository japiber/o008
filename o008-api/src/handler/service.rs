use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use o008_common::{ServiceRequest};
use o008_common::AppCommand;
use o008_dispatcher::{poll_message, send_message};
use crate::handler::dispatch_error_into_response;


/// Get Service item by service name, application name and tenant name
///
/// Get Service item by name, application and tenant. Return only status 200 on success or 404 if Service is not found.
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
pub async fn get_service(Path((name, application, tenant)): Path<(String, String, String)>) -> impl IntoResponse {

    let req = ServiceRequest::get_request(name, application, tenant);
    let msg = send_message(&AppCommand::GetService { value: req });
    match poll_message(msg.id()).await {
        Ok(srv) => (StatusCode::OK, Json(srv)).into_response(),
        Err(e) => dispatch_error_into_response(e)
    }
}
