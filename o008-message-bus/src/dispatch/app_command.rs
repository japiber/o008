use uuid::Uuid;
use o008_common::{AppCommand};
use crate::action::{application, builder, service, service_version, tenant};
use crate::dispatch::handler;


pub(crate) async fn dispatcher(from : Uuid, cmd: Box<AppCommand>) -> bool {
    match *cmd {
        AppCommand::CreateBuilder { value } =>
            handler::request(from, value, builder::create).await,
        AppCommand::GetBuilder { value } =>
            handler::request(from, value, builder::get).await,
        AppCommand::DeleteBuilder { value } =>
            handler::request(from, value, builder::delete).await,
        AppCommand::CreateTenant { value } =>
            handler::request(from, value, tenant::create).await,
        AppCommand::GetTenant { value } =>
            handler::request(from, value, tenant::get).await,
        AppCommand::CreateApplication { value } =>
            handler::request(from, value, application::create).await,
        AppCommand::GetApplication { value } =>
            handler::request(from, value, application::get).await,
        AppCommand::CreateService { value } =>
            handler::request(from, value, service::create).await,
        AppCommand::UpdateService {source, value } =>
            handler::request_with_source(from, source, value, service::update).await,
        AppCommand::GetService { value } =>
            handler::request(from, value, service::get).await,
        AppCommand::CreateServiceVersion { value } =>
            handler::request(from, value, service_version::create).await,
    }
}
