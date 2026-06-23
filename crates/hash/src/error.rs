use js_core::JsError;
use thiserror::Error;

js_core::declare_into_js_result!();

#[derive(Error, Debug, JsError)]
pub enum HashError {
    #[error("{0}")]
    #[js(type_error)]
    InvalidArgType(String),
}

impl HashError {
    pub fn invalid_arg_type(arg_name: &str, expected: &str, received: &str) -> Self {
        HashError::InvalidArgType(format!(
            "the \"{arg_name}\" argument must be of type {expected}. Received {received}"
        ))
    }
}

js_core::impl_into_js_result!(IntoJsResult, HashError);
