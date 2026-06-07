use js_core::JsError;
use js_core::error::SystemError;
use thiserror::Error;

js_core::declare_into_js_result!();

#[derive(Error, Debug, JsError)]
pub enum PathError {
    #[error("{0}")]
    #[js(system)]
    System(SystemError),

    #[error("{0}")]
    #[js(error)]
    InvalidPath(String),
}

impl PathError {
    pub fn from_io(e: std::io::Error, syscall: &'static str) -> Self {
        PathError::System(SystemError::from_io(e, syscall, None))
    }

    pub fn invalid_path(msg: impl Into<String>) -> Self {
        PathError::InvalidPath(msg.into())
    }
}

js_core::impl_into_js_result!(IntoJsResult, PathError);
