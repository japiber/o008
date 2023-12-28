use std::sync::{OnceLock};
use async_trait::async_trait;
use crate::dispatch_channel::DispatchChannel;

pub use error::{AppCommandError, DispatcherError, InternalCommandError};
pub use dispatch_command::DispatchCommand;

mod dispatcher;
mod action;
mod dispatch_channel;
mod dispatch_command;
mod error;
mod command_queue;

pub use command_queue::CommandQueue;


pub type BoxDispatchCommand = Box<DispatchCommand>;

static ST_DISPATCHER_CHANNEL : OnceLock<DispatchChannel<BoxDispatchCommand>> = OnceLock::new();

pub fn cmd_dispatch_channel<'a>() -> &'a DispatchChannel<BoxDispatchCommand> {
    ST_DISPATCHER_CHANNEL.get_or_init(Default::default)
}

pub type DispatchResult<T> = Result<T, DispatcherError>;

#[async_trait]
pub trait AsyncDispatcher<T> {
    async fn dispatch(&self) -> DispatchResult<T>;
}

pub trait DispatchPublisher<S> {
    fn publish(&self) -> S;
}
