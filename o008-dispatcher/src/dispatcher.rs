use async_trait::async_trait;
use tracing::error;
use o008_setting::AppCommand;
use crate::action::{create_application_action, create_builder_action, create_tenant_action, delete_builder_action, get_application_action, get_builder_action};
use crate::{AsyncDispatcher, DispatchPublisher, DispatchResult};
use crate::dispatch_command::InternalCommand;
use crate::error::DispatcherError;
use crate::error::InternalCommandError::Quit;


#[async_trait]
impl AsyncDispatcher<serde_json::Value> for AppCommand {
    #[tracing::instrument]
    async fn dispatch(&self) -> DispatchResult<serde_json::Value> {
        match self {
            AppCommand::CreateBuilder { name, active, cmd } => create_builder_action(name, *active, cmd).await,
            AppCommand::GetBuilder { name} => get_builder_action(name).await,
            AppCommand::DeleteBuilder { name } => delete_builder_action(name).await,
            AppCommand::CreateTenant { name, coexisting} => create_tenant_action(name, *coexisting).await,
            AppCommand::CreateApplication { name, tenant, class_unit} => create_application_action(name, tenant, class_unit).await,
            AppCommand::GetApplication { name } => get_application_action(name).await,
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
impl AsyncDispatcher<()> for InternalCommand {
    async fn dispatch(&self) -> DispatchResult<()> {
        match self {
            InternalCommand::Quit => Err(DispatcherError::InternalCommand(Quit(None)))
        }
    }
}