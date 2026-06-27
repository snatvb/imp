use js_core::JsError;
use thiserror::Error;

js_core::declare_into_js_result!();

#[derive(Error, Debug, JsError)]
pub enum CryptoError {
    #[error("{0}")]
    #[js(type_error)]
    InvalidArgType(String),

    #[error("{0}")]
    #[js(type_error)]
    InvalidValue(String),

    #[error("{0}")]
    #[js(range_error)]
    RangeError(String),
}

impl CryptoError {
    pub fn invalid_arg_type(arg_name: &str, expected: &str, received: &str) -> Self {
        CryptoError::InvalidArgType(format!(
            "the \"{arg_name}\" argument must be of type {expected}. Received {received}"
        ))
    }

    pub fn invalid_value(msg: impl Into<String>) -> Self {
        CryptoError::InvalidValue(msg.into())
    }

    pub fn range_error(msg: impl Into<String>) -> Self {
        CryptoError::RangeError(msg.into())
    }
}

js_core::impl_into_js_result!(IntoJsResult, CryptoError);
