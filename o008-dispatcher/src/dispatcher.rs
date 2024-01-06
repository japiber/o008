use async_trait::async_trait;
use o008_common::{AppCommand, InternalCommand};
use crate::{action, AsyncDispatcher, DispatchCommand, DispatchMessage, DispatchResult};
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
            AppCommand::UpdateService { source, value} => action::service::update(source, value).await,
            AppCommand::GetService { value } => action::service::get(value).await,
        }
    }
}

pub async fn dispatch(cmd: Box<DispatchMessage>) -> DispatchResult<serde_json::Value> {
    match (*cmd).request() {
        DispatchCommand::App(app) => app.dispatch().await,
        DispatchCommand::Internal(i) => i.dispatch().await,
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
