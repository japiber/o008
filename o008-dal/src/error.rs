use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    DataCreation(String),
    DataNotFound(String),
    DataUpdate(String),
    DataDelete(String),
    DataGenericError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DataCreation(e) => write!(f, "Could not create data: {}", e),
            Error::DataNotFound(e) => write!(f, "data not found: {}", e),
            Error::DataUpdate(e) => write!(f, "could not update data: {}", e),
            Error::DataDelete(e) => write!(f, "could not delete data: {}", e),
            Error::DataGenericError(e) => write!(f, "data error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
