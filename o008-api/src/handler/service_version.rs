use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::to_value;
use o008_common::{AppCommand, DispatchCommand, ServiceVersionCreateRequest, ServiceVersionRequest};
use o008_entity::{QueryEntity, ServiceVersion};
use o008_message_bus::{RequestMessage};
use crate::handler::{message_into_response};

/// Create or Update Service item by service name, application name and tenant name
///
/// Create or Update Service item by name, application and tenant. Return status 200 on success or 404 if Service is not found.
#[utoipa::path(
put,
path = "/service/{service}/app/{app}/tenant/{tenant}/version/{version}",
request_body = ServiceVersionCreateRequest,
responses(
(status = 200, description = "create service version done successfully", body = ServiceVersion),
(status = 404, description = "Service version not found")
),
params(
("service" = String, Path, description = "Service name"),
("app" = String, Path, description = "Service application name"),
("tenant" = String, Path, description = "Service tenant name"),
("version" = String, Path, description = "Service version"),
)
)]
pub async fn service_version_put(Path((name, application, tenant, version)): Path<(String, String, String, String)>,
                                 Json(payload) : Json<ServiceVersionCreateRequest>) -> impl IntoResponse {
    let mut req = ServiceVersionRequest::build_get_request(version, name, application, tenant);
    if !ServiceVersion::persisted(to_value(&req).unwrap()).await {
        req.set_repo_ref(payload.repo_ref);
        req.set_builder(payload.builder);
        let msg = RequestMessage::new(DispatchCommand::from(AppCommand::CreateServiceVersion { value: req }));
        message_into_response(msg, StatusCode::ACCEPTED).await
    } else {
        (StatusCode::ALREADY_REPORTED, Json(req)).into_response()
    }
}
