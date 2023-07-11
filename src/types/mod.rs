pub mod commands;
pub mod error;

pub type Result<T> = core::result::Result<T, error::ProxylError>;
