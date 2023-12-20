use std::fmt::{Debug, Display, Formatter};
use o008_dal::DalError;

#[derive(Debug)]
pub enum EntityError {
    Persist(DalError),
    Destroy(DalError),
    UnPersisted(String),
}

impl Display for EntityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityError::Persist(e) => write!(f, "could not persist entity: {}", e),
            EntityError::Destroy(e) => write!(f, "could not destroy entity: {}", e),
            EntityError::UnPersisted(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for EntityError {}

