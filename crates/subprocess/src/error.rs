use js_core::JsError;
use js_core::error::SystemError;
use thiserror::Error;

js_core::declare_into_js_result!();

#[derive(Error, Debug, JsError)]
pub enum SubprocessError {
    #[error("subprocess not found: {0}")]
    #[js(error)]
    NotFound(String),

    #[error("{0}")]
    #[js(system)]
    Io(SystemError),

    #[error("subprocess timed out after {0}ms")]
    #[js(error)]
    Timeout(u64),
}

impl SubprocessError {
    pub fn from_io_spawn(e: std::io::Error, cmd: &str) -> Self {
        if e.kind() == std::io::ErrorKind::NotFound {
            SubprocessError::NotFound(cmd.to_string())
        } else {
            SubprocessError::Io(SystemError::from_io(e, "spawn", Some(cmd.to_string())))
        }
    }
}

js_core::impl_into_js_result!(IntoJsResult, SubprocessError);
