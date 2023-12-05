use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Failed parsing input: {}", .0)]
    ParserError(String),
}
