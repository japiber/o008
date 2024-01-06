use std::error::Error as StdError;

mod macros;
mod command;
mod request;
pub mod error;

pub use command::AppCommand;
pub use command::InternalCommand;
pub use macros::ScopeCall;
pub use request::application::ApplicationRequest;
pub use request::builder::BuilderRequest;
pub use request::service::ServiceRequest;
pub use request::tenant::TenantRequest;
pub use request::RequestValidator;
pub use error::{AppCommandError, DispatcherError, InternalCommandError};

pub type BoxDynError = Box<dyn StdError + Send + Sync + 'static>;

#[async_trait::async_trait]
pub trait AsyncFrom<T>  where T: Send + Unpin + Sized {
    async fn from(value: T) -> Self;
}

pub type DispatchResult<T> = Result<T, DispatcherError>;


