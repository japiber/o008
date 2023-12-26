use std::fmt::{Display, Formatter};


#[derive(Debug, Clone)]
pub enum AppCommandError {
    Create(String),
    NotFound(String),
    Destroy(String),
    InvalidRequest(String),
}

#[derive(Debug, Clone)]
pub enum InternalCommandError {
    Quit(Option<String>)
}

#[derive(Debug, Clone)]
pub enum DispatcherError {
    AppCommand(AppCommandError),
    InternalCommand(InternalCommandError),
}

impl Display for AppCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppCommandError::Create(s) => write!(f, "create: {}", s),
            AppCommandError::NotFound(s) => write!(f, "not found: {}", s),
            AppCommandError::Destroy(s) => write!(f, "destroy: {}", s),
            AppCommandError::InvalidRequest(s) => write!(f, "invalid request: {}", s),
        }
    }
}

impl std::error::Error for AppCommandError {}

impl Display for InternalCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalCommandError::Quit(reason) => match reason {
                None => write!(f, "application terminates"),
                Some(s) => write!(f, "application terminates: {}", s)
            }
        }
    }
}

impl std::error::Error for InternalCommandError {}

impl Display for DispatcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DispatcherError::AppCommand(e) => write!(f, "app command: {}", e),
            DispatcherError::InternalCommand(e) => write!(f, "internal command: {}", e),
        }
    }
}

impl std::error::Error for DispatcherError {}

impl From<AppCommandError> for DispatcherError {
    fn from(value: AppCommandError) -> Self {
        DispatcherError::AppCommand(value)
    }
}

impl From<InternalCommandError> for DispatcherError {
    fn from(value: InternalCommandError) -> Self {
        DispatcherError::InternalCommand(value)
    }
}
