use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use uuid::Uuid;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;

use crate::{cmd_dispatch_channel, DispatcherError, DispatchMessage, DispatchResult, InternalCommandError};
use crate::dispatcher::dispatch;

const COMMAND_WAIT_MILLIS: u64 = 64;
const JOIN_WAIT_MILLIS: u64 = 24;


type ResultItem<T> = JoinHandle<(DispatchMessage, Option<DispatchResult<T>>)>;

#[async_trait]
pub trait MessagePoll<T> {
    async fn poll(&self, msg_id: Uuid) -> DispatchResult<Value>;
}

#[derive(Debug, Clone)]
struct InnerMessageQueue {
    queue: Arc<Mutex<VecDeque<ResultItem<Value>>>>,
    response: Arc<Mutex<HashMap<Uuid, DispatchResult<Value>>>>,
}

pub struct MessageQueue {
    inner: Arc<InnerMessageQueue>,
}

#[async_trait]
impl MessagePoll<Value> for MessageQueue {
    async fn poll(&self, msg_id: Uuid) -> DispatchResult<Value> {
        let inner = Arc::clone(&self.inner);
        task::spawn(async move {
            while !inner.check_result(msg_id).await {
                inner.command_task().await;
                inner.handle_result().await;
            }
            inner.extract_result(msg_id).await.unwrap()
        }).await.expect("message queue poll panics")
    }
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Default::default())
        }
    }
}

impl InnerMessageQueue {
    async fn check_result(&self, msg_id: Uuid) -> bool {
        let arr = Arc::clone(&self.response);
        let lar = arr.lock().await;
        lar.contains_key(&msg_id)
    }

    async fn extract_result(&self, msg_id: Uuid) -> Option<DispatchResult<Value>> {
        let arr = Arc::clone(&self.response);
        let mut lar = arr.lock().await;
        lar.remove(&msg_id)
    }

    async fn command_task(&self) {
        let queue = Arc::clone(&self.queue);
        task::spawn(async move {
            if let Some(msg) = cmd_dispatch_channel().recv().await {
                let mut qlock = queue.lock().await;
                qlock.push_back(task::spawn(async move {
                    let result : DispatchResult<Value> = dispatch(Box::new(msg.clone())).await;
                    return if check_terminate(&result) {
                        cmd_dispatch_channel().terminate();
                        (msg, None)
                    } else {
                        (msg, Some(result))
                    };
                }));
                tokio::time::sleep(Duration::from_millis(COMMAND_WAIT_MILLIS)).await;
            }
        }).await.expect("message queue command task panics!!")
    }

    async fn handle_result(&self) {
        let queue = Arc::clone(&self.queue);
        let response = Arc::clone(&self.response);
        task::spawn(async move {
            tokio::time::sleep(Duration::from_millis(JOIN_WAIT_MILLIS)).await;
            let mut qlock = queue.lock().await;
            while let Some(handle) = qlock.pop_front() {
                if let (msg, Some(result)) = handle.await.expect("command handle panics!!") {
                    let mut rlock = response.lock().await;
                    rlock.insert(msg.id(), result);
                }
            }
        }).await.expect("message queue handle_result panics!!");
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self {
            inner: Arc::new(Default::default())
        }
    }
}

impl Default for InnerMessageQueue {
    fn default() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::<ResultItem<Value>>::new())),
            response: Arc::new(Mutex::new(HashMap::<Uuid, DispatchResult<Value>>::new()))
        }
    }
}

fn check_terminate<T>(dr: &DispatchResult<T>) -> bool {
    if let Err(DispatcherError::InternalCommand(i)) = dr {
        return match i {
            InternalCommandError::Terminate(_) => {
                true
            }
        };
    }
    false
}
