use std::sync::Arc;
use lazy_static::lazy_static;
use serde_json::Value;
use o008_common::{DispatchCommand, DispatchResponse};
use crate::bus::Bus;


mod bus;
mod message;
mod action;
mod dispatch;
mod helper;

pub use message::request::RequestMessage;
pub use message::response::ResponseMessage;
pub use helper::{bus_processor, send_request, send_response, launch_request_poll, launch_response_poll};
use o008_setting::app_config;

type AppRequestMessage = RequestMessage<DispatchCommand>;
type AppResponseMessage = ResponseMessage<DispatchResponse<Value>>;

pub type RequestMessageBus = Bus<AppRequestMessage>;
pub type ResponseMessageBus = Bus<AppResponseMessage>;


lazy_static! {
    static ref ST_REQUEST_BUS: Arc<RequestMessageBus> = {
        Arc::new(RequestMessageBus::new(app_config().bus().request_capacity()))
    };

    static ref ST_RESPONSE_BUS: Arc<ResponseMessageBus> = {
        Arc::new(ResponseMessageBus::new(app_config().bus().response_capacity()))
    };
}

pub fn request_bus() -> Arc<RequestMessageBus> {
    Arc::clone(&ST_REQUEST_BUS)
}

pub fn response_bus() -> Arc<ResponseMessageBus> {
    Arc::clone(&ST_RESPONSE_BUS)
}

