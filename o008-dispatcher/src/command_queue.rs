use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use crate::{AsyncDispatcher, cmd_dispatch_channel, DispatchCommand, DispatcherError, DispatchPublisher, DispatchResult, InternalCommandError};


const COMMAND_WAIT_MILLIS : u64 = 16;
const JOIN_WAIT_MILLIS : u64 = 64;
const TERMINATE_WAIT_MILLIS : u64 = 256;

pub struct CommandQueue {
    halt: Arc<AtomicBool>,
    handles_queue: Arc<Mutex<VecDeque<JoinHandle<()>>>>,
}

impl CommandQueue {
    pub async fn poll(poll_once: bool) {
        let cq : Self = Default::default();
        task::spawn(async move {
            if poll_once {
                CommandQueue::terminate(&cq.halt).await
            }
            loop {
                cq.command_task().await;
                cq.queue_join().await;
                if cq.halt.load(Ordering::Relaxed) {
                    break
                }
            }
        }).await.expect("command queue poll panics");
    }

    async fn command_dispatch(cmd: Box<DispatchCommand>) -> DispatchResult<Value> {
        match *cmd {
            DispatchCommand::App(app) => app.dispatch().await,
            DispatchCommand::Internal(i) => i.dispatch().await,
        }
    }

    async fn command_task(&self) {
        let cmd = cmd_dispatch_channel().recv().await;
        let halt = self.halt.clone();
        let mut qlock = self.handles_queue.lock().await;
        qlock.push_back(task::spawn(async move {
            let result = CommandQueue::command_dispatch(cmd).await;
            if let Err(DispatcherError::InternalCommand(i)) = result   {
                match i {
                    InternalCommandError::Quit(_) => CommandQueue::terminate(halt.as_ref()).await
                }
            } else {
                result.publish();
            }
            tokio::time::sleep(Duration::from_millis(COMMAND_WAIT_MILLIS)).await
        }));
    }

    async fn queue_join(&self) {
        let aqr = Arc::clone(&self.handles_queue);
        let h = task::spawn(async move {
            let mut qr = aqr.lock().await;
            while let Some(handle) = qr.pop_front() {
                handle.await.expect("command queue error");
                tokio::time::sleep(Duration::from_millis(JOIN_WAIT_MILLIS)).await
            }
        });
        h.await.expect("command queue error");
    }

    async fn terminate(halt: &AtomicBool) {
        tokio::time::sleep(Duration::from_millis(TERMINATE_WAIT_MILLIS)).await;
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
