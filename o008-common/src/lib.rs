use std::error::Error as StdError;
use async_trait::async_trait;
use sqlx::types::Uuid;

mod macros;
mod command;
mod request;
pub mod error;
pub use command::{AppCommand, InternalCommand, DispatchCommand, DispatchResponse};
pub use macros::ScopeCall;
pub use request::application::ApplicationRequest;
pub use request::builder::BuilderRequest;
pub use request::repo_reference::RepoReferenceRequest;
pub use request::repo_reference_kind::RepoReferenceKind;
pub use request::service::ServiceRequest;
pub use request::service_version::ServiceVersionRequest;
pub use request::tenant::TenantRequest;
pub use request::RequestValidator;
pub use error::{AppCommandError, DispatcherError, InternalCommandError};

pub type BoxDynError = Box<dyn StdError + Send + Sync + 'static>;

#[async_trait]
pub trait AsyncFrom<T>  where T: Send + Unpin + Sized {
    async fn from(value: T) -> Self;
}

pub trait TypeInfo {
    fn type_name() -> &'static str;
    fn type_of(&self) -> &'static str;
}

pub enum ResultDispatcher {
    Done(bool),
    Pending,
    Abort
}

#[async_trait]
pub trait CommandDispatcher {
    async fn dispatch(&self, target: Uuid) -> ResultDispatcher;
}

pub type DispatchResult<T> = Result<T, DispatcherError>;


