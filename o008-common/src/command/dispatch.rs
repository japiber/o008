use crate::{AppCommand, DispatchResult, InternalCommand};

#[derive(Debug, Clone)]
pub enum DispatchCommand {
    App(Box<AppCommand>),
    Internal(InternalCommand)
}

impl From<AppCommand> for DispatchCommand {
    fn from(value: AppCommand) -> Self {
        Self::App(Box::new(value))
    }
}

impl From<InternalCommand> for DispatchCommand {
    fn from(value: InternalCommand) -> Self {
        Self::Internal(value)
    }
}

#[derive(Debug, Clone)]
pub enum DispatchResponse<T> {
    App(Box<DispatchResult<T>>),
    Internal(InternalCommand)
}

impl<T> From<DispatchResult<T>> for DispatchResponse<T> {
    fn from(value: DispatchResult<T>) -> Self {
        Self::App(Box::new(value))
    }
}

impl<T> From<InternalCommand> for DispatchResponse<T> {
    fn from(value: InternalCommand) -> Self {
        Self::Internal(value)
    }
}
