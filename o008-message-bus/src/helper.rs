use std::time::Duration;
use serde_json::Value;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{error, info};
use uuid::Uuid;
use o008_common::{DispatchCommand, DispatchResponse, DispatchResult, InternalCommand};
use o008_setting::app_config;
use crate::{AppRequestMessage, AppResponseMessage, request_bus, response_bus};
use crate::dispatch::app_command;

pub fn send_request(msg: AppRequestMessage) -> bool {
    match request_bus().send(msg) {
        Ok(_) => true,
        Err(e) => {
            error!("could not send request message: {}", e);
            false
        }
    }
}

pub fn send_response(msg: AppResponseMessage) -> bool {
    match response_bus().send(msg) {
        Ok(_) => true,
        Err(e) =>  {
            error!("could not send response message: {}", e);
            false
        }
    }
}

pub async fn bus_processor(msg: AppRequestMessage) -> Option<DispatchResult<Value>> {
    let trq= launch_request_poll(msg.id());
    let trs= launch_response_poll(msg.id());
    if send_request(msg) {
        trq.await.unwrap();
        trs.await.unwrap()
    } else {
        None
    }
}

pub fn launch_response_poll(target: Uuid) -> JoinHandle<Option<DispatchResult<Value>>> {
    let mut res= response_bus().subscribe();
    tokio::spawn(async move {
        loop {
            match res.try_recv() {
                Ok(msg) => match msg.response() {
                    DispatchResponse::App(app) =>
                        if msg.from() == target {
                            info!("target {} response message received", target);
                            break Some(*app)
                        } else {
                            sleep(Duration::from_millis(app_config().bus().response_wait())).await
                        },
                    DispatchResponse::Internal(e) => match e {
                        InternalCommand::Quit => break None
                    }
                },
                Err(e)  => match e {
                    TryRecvError::Closed => break None,
                    _ => sleep(Duration::from_millis(app_config().bus().response_wait())).await
                }
            }
        }
    })
}

pub fn launch_request_poll(target: Uuid)-> JoinHandle<()>  {
    let req_bus = request_bus();
    let mut rx = req_bus.subscribe();
    tokio::spawn(async move {
        loop {
            match rx.try_recv() {
                Ok(msg) =>
                    match msg.request() {
                        DispatchCommand::App(cmd) =>
                            if msg.id() == target {
                                info!("target {} request message received", target);
                                if !app_command::dispatcher(msg.id(), cmd).await {
                                    error!("could not dispatch message: {:?}", msg)
                                }
                                break;
                            } else {
                                sleep(Duration::from_millis(app_config().bus().request_wait())).await
                            },
                        DispatchCommand::Internal(e) => match e {
                            InternalCommand::Quit => break,
                        }
                    },
                Err(e) => match e {
                    TryRecvError::Closed => break,
                    _ => sleep(Duration::from_millis(app_config().bus().request_wait())).await
                }
            }
        }
    })
}
