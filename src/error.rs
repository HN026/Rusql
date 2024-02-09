use thiserror::Error;

use sqlparser::parser::ParserError;
use std::result;

pub type Result<T> = result::Result<T, RUSQLError>;

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum RUSQLError {
    #[error("Not Implemented error: {0}")]
    NotImplemented(String),
    #[error("General error: {0}")]
    General(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Unknown command error: {0}")]
    UnknownCommand(String),
    #[error("Parser error: {0:?}")]
    SqlError(#[from] ParserError),
}
