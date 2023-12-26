pub(crate) mod tenant;
pub(crate) mod builder;
pub(crate) mod application;

pub trait RequestValidator {
    fn is_valid_create(&self) -> bool;
    fn is_valid_get(&self) -> bool;
}