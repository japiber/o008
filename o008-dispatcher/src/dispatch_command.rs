use o008_setting::AppCommand;


#[derive(Debug, Clone)]
pub enum InternalCommand {
    Quit
}

#[derive(Debug, Clone)]
pub enum DispatchCommand {
    App(AppCommand),
    Internal(InternalCommand)
}

impl From<AppCommand> for DispatchCommand {
    fn from(value: AppCommand) -> Self {
        DispatchCommand::App(value)
    }
}

impl From<InternalCommand> for DispatchCommand {
    fn from(value: InternalCommand) -> Self {
        DispatchCommand::Internal(value)
    }
}