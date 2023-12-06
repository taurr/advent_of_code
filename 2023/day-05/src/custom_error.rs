use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Failed parsing input: {}", .0)]
    ParserError(String),

    #[error("{}", .0)]
    ProcessError(String),
}
