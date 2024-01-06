use std::sync::{OnceLock};
use async_trait::async_trait;
use crate::dispatch_channel::DispatchChannel;


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

pub type DispatchResult<T> = Result<T, DispatcherError>;

#[async_trait]
pub trait AsyncDispatcher<T> {
    async fn dispatch(&self) -> DispatchResult<T>;
}
