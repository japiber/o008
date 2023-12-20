use std::fmt::{Display, Formatter};
use o008_common::BoxDynError;

#[derive(Debug)]
pub enum AppCommandError {
    Create(BoxDynError),
    NotFound(String),
    Destroy(BoxDynError),
}

#[derive(Debug)]
pub enum InternalCommandError {
    Quit(Option<String>)
}

#[derive(Debug)]
pub enum DispatcherError {
    AppCommand(AppCommandError),
    InternalCommand(InternalCommandError),
}

impl Display for AppCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppCommandError::Create(e) => write!(f, "create error: {}", e),
            AppCommandError::NotFound(s) => write!(f, "not found error: {}", s),
            AppCommandError::Destroy(e) => write!(f, "destroy error: {}", e),
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
            DispatcherError::AppCommand(e) => write!(f, "{}", e),
            DispatcherError::InternalCommand(e) => write!(f, "{}", e),
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
