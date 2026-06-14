use js_core::JsError;
use thiserror::Error;

#[derive(Error, Debug, JsError)]
pub enum Error {
    #[error("Invalid URL: {0}")]
    #[js(type_error)]
    InvalidUrl(String),

    #[error("Invalid base URL: {0}")]
    #[js(type_error)]
    InvalidBaseUrl(String),
}
