use async_trait::async_trait;
use tracing::error;
use o008_common::{AppCommand, InternalCommand};
use crate::{action, AsyncDispatcher, DispatchPublisher, DispatchResult};
use crate::error::DispatcherError;
use crate::error::InternalCommandError::Terminate;


#[async_trait]
impl AsyncDispatcher<serde_json::Value> for AppCommand {
    #[tracing::instrument]
    async fn dispatch(&self) -> DispatchResult<serde_json::Value> {
        match self {
            AppCommand::CreateBuilder { value } => action::builder::create(value).await,
            AppCommand::GetBuilder { value} => action::builder::get(value).await,
            AppCommand::DeleteBuilder { value } => action::builder::delete(value).await,
            AppCommand::CreateTenant { value} => action::tenant::create(value).await,
            AppCommand::GetTenant { value } => action::tenant::get(value).await,
            AppCommand::CreateApplication { value} => action::application::create(value).await,
            AppCommand::GetApplication { value } => action::application::get(value).await,
            AppCommand::CreateService { value } => action::service::create(value).await,
            AppCommand::GetService { value } => action::service::get(value).await,
        }
    }
}

impl DispatchPublisher<((), Option<DispatcherError>)> for DispatchResult<serde_json::Value> {
    #[tracing::instrument]
    fn publish(&self) -> ((), Option<DispatcherError>) {
        match self {
            Ok(v) => (println!("{}", serde_json::to_string_pretty(&v).unwrap()), None),
            Err(e) => (error!("{}", e.to_string()), None),
        }
    }
}

impl DispatchPublisher<((), Option<DispatcherError>)> for DispatchResult<()> {
    #[tracing::instrument]
    fn publish(&self) -> ((), Option<DispatcherError>) {
        self.clone().map_or_else(|e| ((), Some(e.clone())), |v| (v, None))
    }
}

#[async_trait]
impl AsyncDispatcher<serde_json::Value> for InternalCommand {
    async fn dispatch(&self) -> DispatchResult<serde_json::Value> {
        match self {
            InternalCommand::Quit => Err(DispatcherError::InternalCommand(Terminate(None)))
        }
    }
}