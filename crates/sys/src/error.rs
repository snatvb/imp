use js_core::JsError;
use js_core::error::SystemError;
use thiserror::Error;

js_core::declare_into_js_result!();

#[derive(Error, Debug, JsError)]
pub enum Error {
    #[error("{0}")]
    #[js(system)]
    System(SystemError),

    #[error("{0}")]
    #[js(error)]
    Internal(String),

    #[error("Unknown key: {0}")]
    #[js(type_error)]
    UnknownKey(String),
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::Internal(e.to_string())
    }
}

js_core::impl_into_js_result!(
    IntoJsResult,
    Error,
    tokio::task::JoinError
);
