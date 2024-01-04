use uuid::Uuid;
use crate::DispatchCommand;

#[derive(Debug, Clone)]
pub struct DispatchMessage {
    id: Uuid,
    request: DispatchCommand,
}

impl DispatchMessage {
    pub fn new (cmd: DispatchCommand) -> Self {
        Self {
            id: Uuid::new_v4(),
            request: cmd,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn request(&self) -> DispatchCommand {
        self.request.clone()
    }
}