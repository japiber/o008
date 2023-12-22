use std::fmt::{Display, Formatter};
use std::error::Error as StdError;


#[derive(Debug)]
pub enum DalError {
    DataCreation(sqlx::Error),
    DataNotFound(sqlx::Error),
    DataUpdate(sqlx::Error),
    DataDelete(sqlx::Error),
    DataGenericError(sqlx::Error),
}

impl Display for DalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DalError::DataCreation(e) => write!(f, "could not create: {}", e),
            DalError::DataNotFound(e) => write!(f, "not found: {}", e),
            DalError::DataUpdate(e) => write!(f, "could not update: {}", e),
            DalError::DataDelete(e) => write!(f, "could not delete: {}", e),
            DalError::DataGenericError(e) => write!(f, "generic error: {}", e),
        }
    }
}

impl StdError for DalError {
}
