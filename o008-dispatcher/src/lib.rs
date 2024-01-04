use std::sync::{OnceLock};
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use crate::dispatch_channel::DispatchChannel;
use o008_common::AppCommand;

pub use error::{AppCommandError, DispatcherError, InternalCommandError};
pub use dispatch_command::DispatchCommand;
pub use command_queue::{MessageQueue, MessagePoll};
pub use message::DispatchMessage;

mod dispatcher;
mod action;
mod dispatch_channel;
mod dispatch_command;
mod error;
mod command_queue;
mod message;

static ST_DISPATCHER_CHANNEL : OnceLock<DispatchChannel> = OnceLock::new();

static ST_MESSAGE_QUEUE : OnceLock<MessageQueue> = OnceLock::new();

 fn cmd_dispatch_channel<'a>() -> &'a DispatchChannel {
    ST_DISPATCHER_CHANNEL.get_or_init(Default::default)
}

fn message_queue<'a>() -> &'a MessageQueue {
    ST_MESSAGE_QUEUE.get_or_init(Default::default)
}

pub fn send_message(cmd: &AppCommand) -> DispatchMessage {
    let msg = DispatchMessage::new(DispatchCommand::from(cmd.clone()));
    ST_DISPATCHER_CHANNEL.get_or_init(Default::default).send(msg.clone());
    msg
}

pub async fn poll_message(msg_id: Uuid) -> DispatchResult<Value> {
    message_queue().poll(msg_id).await
}

pub type DispatchResult<T> = Result<T, DispatcherError>;

#[async_trait]
pub trait AsyncDispatcher<T> {
    async fn dispatch(&self) -> DispatchResult<T>;
}
