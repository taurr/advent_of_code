use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
