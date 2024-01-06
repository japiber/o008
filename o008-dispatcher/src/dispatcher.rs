use async_trait::async_trait;
use o008_common::{AppCommand, DispatcherError, InternalCommand};
use o008_common::InternalCommandError::Terminate;
use o008_action::{application, builder, service, tenant};
use crate::{AsyncDispatcher, DispatchCommand, DispatchMessage, DispatchResult};


#[async_trait]
impl AsyncDispatcher<serde_json::Value> for AppCommand {
    #[tracing::instrument]
    async fn dispatch(&self) -> DispatchResult<serde_json::Value> {
        match self {
            AppCommand::CreateBuilder { value } => builder::create(value).await,
            AppCommand::GetBuilder { value} => builder::get(value).await,
            AppCommand::DeleteBuilder { value } => builder::delete(value).await,
            AppCommand::CreateTenant { value} => tenant::create(value).await,
            AppCommand::GetTenant { value } => tenant::get(value).await,
            AppCommand::CreateApplication { value} => application::create(value).await,
            AppCommand::GetApplication { value } => application::get(value).await,
            AppCommand::CreateService { value } => service::create(value).await,
            AppCommand::UpdateService { source, value} => service::update(source, value).await,
            AppCommand::GetService { value } => service::get(value).await,
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
