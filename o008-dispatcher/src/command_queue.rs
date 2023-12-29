use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::Value;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use tracing::info;

use o008_common::InternalCommand::Quit;

use crate::{AsyncDispatcher, cmd_dispatch_channel, DispatchCommand, DispatcherError, DispatchPublisher, DispatchResult, InternalCommandError};

const COMMAND_WAIT_MILLIS: u64 = 64;
const JOIN_WAIT_MILLIS: u64 = 24;


pub struct CommandQueue {
    halt: Arc<AtomicBool>,
    handles_queue: Arc<Mutex<VecDeque<JoinHandle<()>>>>,
}

impl CommandQueue {
    pub async fn poll(poll_once: bool) {
        let cq: Self = Default::default();
        if poll_once {
            cmd_dispatch_channel().send(Box::new(DispatchCommand::from(Quit)));
        }
        tokio::join!(
            cq.command_task(),
            cq.queue_join()
        );
    }

    async fn command_dispatch(cmd: Box<DispatchCommand>) -> DispatchResult<Value> {
        match *cmd {
            DispatchCommand::App(app) => app.dispatch().await,
            DispatchCommand::Internal(i) => i.dispatch().await,
        }
    }

    async fn command_task(&self) {
        info!("start command_task");
        let aqr = Arc::clone(&self.handles_queue);
        let halt = Arc::clone(&self.halt);
        task::spawn(async move {
            while let Some(cmd) = cmd_dispatch_channel().recv().await {
                let mut qlock = aqr.lock().await;
                qlock.push_back(task::spawn(async move {
                    let result = CommandQueue::command_dispatch(cmd).await;
                    if let Err(DispatcherError::InternalCommand(i)) = result {
                        match i {
                            InternalCommandError::Terminate(_) => {
                                cmd_dispatch_channel().terminate();
                            }
                        }
                    } else {
                        result.publish();
                    }
                }));
                tokio::time::sleep(Duration::from_millis(COMMAND_WAIT_MILLIS)).await;
            }
            Self::terminate(halt.as_ref()).await;
        }).await.expect("command queue task panics!!");
    }

    async fn queue_join(&self) {
        let aqr = Arc::clone(&self.handles_queue);
        let halt = Arc::clone(&self.halt);
        task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(JOIN_WAIT_MILLIS)).await;
                let mut qr = aqr.lock().await;
                while let Some(handle) = qr.pop_front() {
                    handle.await.expect("command handle panics!!");
                }
                if halt.load(Ordering::Relaxed) {
                    break;
                }
            }
        }).await.expect("command queue join panics!!!");
    }

    async fn terminate(halt: &AtomicBool) {
        halt.store(true, Ordering::Relaxed)
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self {
            halt: Arc::new(AtomicBool::new(false)),
            handles_queue: Arc::new(Mutex::new(VecDeque::<JoinHandle<()>>::new())),
        }
    }
}
