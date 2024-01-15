use std::future::Future;
use serde_json::Value;
use uuid::Uuid;
use o008_common::{AppCommand, DispatchResult};
use crate::action::{application, builder, service, service_version, tenant};
use crate::{ResponseMessage, send_response};

pub(crate) async fn dispatcher(from : Uuid, cmd: Box<AppCommand>) -> bool {
    match *cmd {
        AppCommand::CreateBuilder { value } => response_handler(from, value, builder::create).await,
        AppCommand::GetBuilder { value } => response_handler(from, value, builder::get).await,
        AppCommand::DeleteBuilder { value } => response_handler(from, value, builder::delete).await,
        AppCommand::CreateTenant { value } => response_handler(from, value, tenant::create).await,
        AppCommand::GetTenant { value } => response_handler(from, value, tenant::get).await,
        AppCommand::CreateApplication { value } => response_handler(from, value, application::create).await,
        AppCommand::GetApplication { value } => response_handler(from, value, application::get).await,
        AppCommand::CreateService { value } => response_handler(from, value, service::create).await,
        AppCommand::UpdateService {source, value } => response_handler_with_source(from, source, value, service::update).await,
        AppCommand::GetService { value } => response_handler(from, value, service::get).await,
        AppCommand::CreateServiceVersion { value } => response_handler(from, value, service_version::create).await,
    }
}

async fn response_handler<F, T, R>(from: Uuid, req: R, f: F) -> bool
    where
        F: FnOnce(R) -> T,
        T: Future<Output = DispatchResult<Value>> + Send,
        R: Send
{
    let result = f(req).await;
    let msg = ResponseMessage::new(from, result);
    send_response(msg)
}

async fn response_handler_with_source<F, T, S, R>(from: Uuid, src: S, req: R, f: F) -> bool
    where
        F: FnOnce(S, R) -> T,
        T: Future<Output = DispatchResult<Value>> + Send,
        R: Send,
        S: Send
{
    let result = f(src, req).await;
    let msg = ResponseMessage::new(from, result);
    send_response(msg)
}

