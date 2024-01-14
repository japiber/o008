use o008_common::{AppCommand, InternalCommand};


#[derive(Debug, Clone)]
pub enum DispatchCommand {
    App(Box<AppCommand>),
    Internal(InternalCommand)
}

impl From<AppCommand> for DispatchCommand {
    fn from(value: AppCommand) -> Self {
        DispatchCommand::App(Box::new(value))
    }
}

impl From<InternalCommand> for DispatchCommand {
    fn from(value: InternalCommand) -> Self {
        DispatchCommand::Internal(value)
    }
}