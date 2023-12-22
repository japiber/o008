use std::sync::OnceLock;
use async_trait::async_trait;
use tokio::task;
use tokio::task::JoinHandle;
use crate::dispatch_channel::DispatchChannel;

pub use error::{AppCommandError, DispatcherError, InternalCommandError};
pub use dispatch_command::{DispatchCommand, InternalCommand};

mod dispatcher;
mod action;
mod dispatch_channel;
mod dispatch_command;
mod error;


pub type BoxDispatchCommand = Box<DispatchCommand>;

static ST_DISPATCHER_CHANNEL : OnceLock<DispatchChannel<BoxDispatchCommand>> = OnceLock::new();

pub fn cmd_dispatch_channel<'a>() -> &'a DispatchChannel<BoxDispatchCommand> {
    ST_DISPATCHER_CHANNEL.get_or_init( DispatchChannel::new)
}

pub type DispatchResult<T> = Result<T, DispatcherError>;

#[async_trait]
pub trait AsyncDispatcher<T> {
    async fn dispatch(&self) -> DispatchResult<T>;
}

pub trait DispatchPublisher<S> {
    fn publish(&self) -> S;
}


pub fn command_poll() -> JoinHandle<()> {
    task::spawn(async {
        loop {
            let r = command_dispatch().await;
            if let Some(DispatcherError::InternalCommand(i)) = r.1  {
                match i {
                    InternalCommandError::Quit(_) => break
                }
            }
        }
    })
}

pub async fn command_dispatch() -> ((), Option<DispatcherError>) {
    let cmd = cmd_dispatch_channel().recv().await;
    match *cmd {
        DispatchCommand::App(app) => app.dispatch().await.publish(),
        DispatchCommand::Internal(i) => i.dispatch().await.publish()
    }
}



