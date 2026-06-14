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
    Aborted(String),

    #[error("{0}")]
    #[js(error)]
    Exception(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Exception(e.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderName> for Error {
    fn from(e: reqwest::header::InvalidHeaderName) -> Self {
        Error::Exception(e.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::Exception(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::System(SystemError::from_io(e, "fetch", None))
    }
}

js_core::impl_into_js_result!(
    IntoJsResult,
    Error,
    reqwest::Error,
    reqwest::header::InvalidHeaderName,
    reqwest::header::InvalidHeaderValue,
    std::io::Error
);
