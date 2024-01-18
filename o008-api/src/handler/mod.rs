use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use o008_common::DispatchCommand;
use o008_common::error::{AppCommandError, DispatcherError, InternalCommandError};
use o008_message_bus::{bus_processor, RequestMessage};

mod service;
mod service_version;

pub use service::service_get;
pub use service::service_put;
pub use service_version::service_version_put;
pub use service::__path_service_get;
pub use service::__path_service_put;
pub use service_version::__path_service_version_put;


fn dispatch_error_into_response(e: DispatcherError) -> Response {
    match e {
        DispatcherError::AppCommand(app_err) =>
            match app_err {
                AppCommandError::Create(s) => (StatusCode::BAD_REQUEST, s).into_response(),
                AppCommandError::Update(s) => (StatusCode::BAD_REQUEST, s).into_response(),
                AppCommandError::NotFound(s) => (StatusCode::NOT_FOUND, s).into_response(),
                AppCommandError::Destroy(s) => (StatusCode::GONE, s).into_response(),
                AppCommandError::InvalidRequest(s) => (StatusCode::BAD_REQUEST, s).into_response(),
                AppCommandError::InvalidResponse(s) => (StatusCode::UNPROCESSABLE_ENTITY, s).into_response(),
            },
        DispatcherError::InternalCommand(int_error) =>
            match int_error {
                InternalCommandError::Terminate(_) => (StatusCode::BAD_REQUEST, "api server is shutting down").into_response()
            }
    }
}

async fn message_into_response(msg: RequestMessage<DispatchCommand>, ok_status: StatusCode) -> Response {
    match bus_processor(msg).await {
        None => (StatusCode::NO_CONTENT, "").into_response(),
        Some(result) => match result {
            Ok(srv) => (ok_status, Json(srv)).into_response(),
            Err(e) => dispatch_error_into_response(e)
        }
    }
}
