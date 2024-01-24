use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use o008_common::{AppCommand, DispatchCommand, ServiceVersionRequest};
use o008_message_bus::{RequestMessage};
use crate::handler::{message_into_response};

/// Create or Update Service item by service name, application name and tenant name
///
/// Create or Update Service item by name, application and tenant. Return status 200 on success or 404 if Service is not found.
#[utoipa::path(
put,
path = "/service/{service}/app/{app}/tenant/{tenant}/version/{version}",
request_body = ServiceVersionRequest,
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
                                 Json(payload) : Json<ServiceVersionRequest>) -> impl IntoResponse {
    let source = ServiceVersionRequest::build_get_request(version, name, application, tenant);
    let msg = RequestMessage::new(
        DispatchCommand::from(AppCommand::PersistServiceVersion { source, request: payload })
    );
    message_into_response(msg, StatusCode::ACCEPTED).await
}
