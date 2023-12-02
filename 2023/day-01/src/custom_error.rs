use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("{description}: {input}")]
    InvalidInput { input: String, description: String },
}

impl AocError {
    #[tracing::instrument(level = "trace", skip(input, description))]
    pub fn invalid_input(input: impl Into<String>, description: impl Into<String>) -> AocError {
        AocError::InvalidInput {
            input: input.into(),
            description: description.into(),
        }
    }
}
