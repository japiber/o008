use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use o008_dispatcher::{AppCommandError, DispatcherError, InternalCommandError};

mod service;

pub use service::get_service;
pub use service::__path_get_service;


fn dispatch_error_into_response(e: DispatcherError) -> Response {
    match e {
        DispatcherError::AppCommand(app_err) =>
            match app_err {
                AppCommandError::Create(s) => (StatusCode::BAD_REQUEST, s).into_response(),
                AppCommandError::NotFound(s) => (StatusCode::NOT_FOUND, s).into_response(),
                AppCommandError::Destroy(s) => (StatusCode::GONE, s).into_response(),
                AppCommandError::InvalidRequest(s) => (StatusCode::BAD_REQUEST, s).into_response(),
                AppCommandError::InvalidResponse(s) => (StatusCode::UNPROCESSABLE_ENTITY, s).into_response(),
            },
        DispatcherError::InternalCommand(int_error) =>
        match int_error {
            InternalCommandError::Terminate(_) => (StatusCode::BAD_REQUEST, "api server is shutting dorn").into_response()
        }

    }
}
