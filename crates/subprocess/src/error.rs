use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubprocessError {
    #[error("subprocess not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
