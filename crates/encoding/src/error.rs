use js_core::JsError;
use thiserror::Error;

js_core::declare_into_js_result!();

#[derive(Error, Debug, JsError)]
pub enum EncodingError {
    #[error("invalid base64 input: {0}")]
    #[js(error)]
    InvalidBase64(String),

    #[error("invalid hex input: {0}")]
    #[js(error)]
    InvalidHex(String),

    #[error("invalid utf-8 input: {0}")]
    #[js(error)]
    InvalidUtf8(String),

    #[error("invalid uri input: {0}")]
    #[js(error)]
    InvalidUri(String),

    #[error("{0}")]
    #[js(type_error)]
    InvalidArgType(String),
}

impl EncodingError {
    pub fn invalid_arg_type(arg_name: &str, expected: &str, received: &str) -> Self {
        EncodingError::InvalidArgType(format!(
            "the \"{arg_name}\" argument must be of type {expected}. Received {received}"
        ))
    }
}

js_core::impl_into_js_result!(IntoJsResult, EncodingError);
