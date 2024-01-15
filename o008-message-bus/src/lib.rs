use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use serde_json::Value;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::error;
use uuid::Uuid;
use o008_common::{DispatchCommand, DispatchResult, InternalCommand};
use crate::bus::Bus;


mod bus;
mod message;
mod action;
mod dispatch;

pub use message::request::RequestMessage;
pub use message::response::ResponseMessage;
use crate::dispatch::app_command;

pub type RequestMessageBus = Bus<RequestMessage<DispatchCommand>>;
pub type ResponseMessageBus = Bus<ResponseMessage<DispatchResult<Value>>>;

lazy_static! {
    static ref ST_REQUEST_BUS: Arc<RequestMessageBus> = {
        Arc::new(RequestMessageBus::new(32))
    };

    static ref ST_RESPONSE_BUS: Arc<ResponseMessageBus> = {
        Arc::new(ResponseMessageBus::new(32))
    };
}

fn request_bus() -> Arc<RequestMessageBus> {
    Arc::clone(&ST_REQUEST_BUS)
}

fn response_bus() -> Arc<ResponseMessageBus> {
    Arc::clone(&ST_RESPONSE_BUS)
}

pub fn send_request(msg: RequestMessage<DispatchCommand>) -> bool {
    match request_bus().send(msg) {
        Ok(_) => true,
        Err(e) => {
            error!("could not send request message: {}", e);
            false
        }
    }
}

pub fn send_response(msg: ResponseMessage<DispatchResult<Value>>) -> bool {
    match response_bus().send(msg) {
        Ok(_) => true,
        Err(e) =>  {
            error!("could not send response message: {}", e);
            false
        }
    }
}

pub fn launch_response_poll(from: Uuid) -> JoinHandle<Option<ResponseMessage<DispatchResult<Value>>>> {
    let mut res = response_bus().subscribe();
    tokio::spawn(async move {
        loop {
            match res.try_recv() {
                Ok(msg) => if msg.from() == from {
                    break Some(msg)
                }
                Err(e) => match e {
                    TryRecvError::Closed => break None,
                    _ => sleep(Duration::from_millis(32)).await,
                }
            }
        }
    })
}

pub fn launch_request_poll()-> JoinHandle<()>  {
    let req_bus = request_bus();
    let mut rx = req_bus.subscribe();
    tokio::spawn(async move {
        loop {
            match rx.try_recv() {
                Ok(msg) => match msg.request() {
                    DispatchCommand::App(cmd) => if !app_command::dispatcher(msg.id(), cmd).await {
                        error!("could not dispatch message: {:?}", msg)
                    },
                    DispatchCommand::Internal(e) => match e {
                        InternalCommand::Quit => break,
                    }
                },
                Err(e) => match e {
                    TryRecvError::Closed => break,
                    _ => sleep(Duration::from_millis(32)).await
                }
            }
        }
    })
}

