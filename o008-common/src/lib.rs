use std::error::Error as StdError;

mod macros;
mod command;
mod request;

pub use command::AppCommand;
pub use command::InternalCommand;
pub use macros::ScopeCall;
pub use request::application::Application as ApplicationRequest;
pub use request::builder::Builder as BuilderRequest;
pub use request::tenant::Tenant as TenantRequest;
pub use request::RequestValidator;

pub type BoxDynError = Box<dyn StdError + Send + Sync + 'static>;


