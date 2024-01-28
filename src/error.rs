use thiserror::Error;

use std::result;

/// This is a type that encapsulated the `std::result` with the enum `SQLRiteError`
/// and makes function signatures easier to read.
pub type Result<T> = result::Result<T, RUSQLError>;

/// SQLRiteError is an enum with all the standardized errors available for returning
///
#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum RUSQLError {
    #[error("Not Implemented error: {0}")] NotImplemented(String),
    #[error("General error: {0}")] General(String),
    #[error("Internal error: {0}")] Internal(String),
    #[error("Unknown command error: {0}")] UnknownCommand(String),
}
