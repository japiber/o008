use async_trait::async_trait;
use uuid::Uuid;
use o008_common::{AppCommand, CommandDispatcher, DispatchCommand, InternalCommand, ResultDispatcher};
use o008_message_bus::{handler, RequestMessage};
use crate::action::{application, builder, service, service_version, tenant};

pub struct RequestMessageCommand(RequestMessage<DispatchCommand>);

impl RequestMessageCommand {

    async fn dispatch_app_command(&self, acmd: AppCommand) -> ResultDispatcher {
        let from = self.0.id();
        let r = match acmd {
            AppCommand::CreateBuilder { request } =>
                handler::request(from, request, builder::create).await,
            AppCommand::GetBuilder { request } =>
                handler::request(from, request, builder::get).await,
            AppCommand::DeleteBuilder { request } =>
                handler::request(from, request, builder::delete).await,
            AppCommand::CreateTenant { request } =>
                handler::request(from, request, tenant::create).await,
            AppCommand::GetTenant { request } =>
                handler::request(from, request, tenant::get).await,
            AppCommand::CreateApplication { request } =>
                handler::request(from, request, application::create).await,
            AppCommand::GetApplication { request } =>
                handler::request(from, request, application::get).await,
            AppCommand::PersistService { source, request } =>
                handler::request_with_source(from, source, request, service::persist).await,
            AppCommand::GetService { request } =>
                handler::request(from, request, service::get).await,
            AppCommand::GetServiceVersions { request } =>
                handler::request(from, request, service::get_with_versions).await,
            AppCommand::PersistServiceVersion { source, request } =>
                handler::request_with_source(from, source, request, service_version::persist).await,
        };
        ResultDispatcher::Done(r)
    }
}

#[async_trait]
impl CommandDispatcher for RequestMessageCommand {

    async fn dispatch(&self, target: Uuid) -> ResultDispatcher {
        if self.0.id() == target {
            match self.0.request().clone() {
                DispatchCommand::App(ac) => self.dispatch_app_command(*ac).await,
                DispatchCommand::Internal(i) => match i {
                    InternalCommand::Quit => ResultDispatcher::Abort
                },
            }
        } else {
            ResultDispatcher::Pending
        }
    }
}

impl From<RequestMessage<DispatchCommand>> for RequestMessageCommand {
    fn from(value: RequestMessage<DispatchCommand>) -> Self {
        Self(value)
    }
}