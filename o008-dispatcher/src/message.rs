use serde_json::Value;
use uuid::Uuid;
use crate::{DispatchCommand, DispatchResult, ST_DISPATCHER_CHANNEL, ST_MESSAGE_QUEUE};
use crate::command_queue::MessagePoll;

#[derive(Debug, Clone)]
pub struct DispatchMessage {
    id: Uuid,
    request: DispatchCommand,
}

impl DispatchMessage {
    pub fn send (cmd: DispatchCommand) -> Self {
        let msg = Self {
            id: Uuid::new_v4(),
            request: cmd,
        };
        ST_DISPATCHER_CHANNEL.get_or_init(Default::default).send(msg.clone());
        msg
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn request(&self) -> DispatchCommand {
        self.request.clone()
    }

    pub async fn poll(&self) -> DispatchResult<Value> {
        ST_MESSAGE_QUEUE.get_or_init(Default::default).poll(self.id()).await
    }
}