use std::fmt::{Display, Formatter};

pub(crate) mod tenant;
pub(crate) mod builder;
pub(crate) mod application;
pub(crate) mod service;

pub enum RequestValidatorError {
    MissingAttribute(String),
    InvalidFormat(String),
    InvalidType(String),
    AtLeastOneRequired,
}

impl Display for RequestValidatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestValidatorError::MissingAttribute(s) => write!(f, "missing attribute: {}", s),
            RequestValidatorError::InvalidFormat(s) => write!(f, "invalid format attribute: {}", s),
            RequestValidatorError::InvalidType(s) => write!(f, "invalid type attribute: {}", s),
            RequestValidatorError::AtLeastOneRequired => write!(f, "at least one attribute is required"),
        }
    }
}

pub type RequestValidatorResult = Result<(), RequestValidatorError>;

pub trait RequestValidator {
    fn is_valid_create(&self) -> RequestValidatorResult;
    fn is_valid_get(&self) -> RequestValidatorResult;
    fn is_valid_update(&self) -> RequestValidatorResult;
}