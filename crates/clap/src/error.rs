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

    #[error("TypeError: {0}")]
    #[js(type_error)]
    TypeError(String),

    #[error("Invalid Property: {0}")]
    #[js(type_error)]
    InvalidProperty(String),
}

js_core::impl_into_js_result!(IntoJsResult, Error);
