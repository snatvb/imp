use thiserror::Error;

#[derive(Error, Debug, js_core::JsError)]
pub enum Error {
    #[error("{0}")]
    #[js(error)]
    Parse(String),

    #[error("{0}")]
    #[js(range_error)]
    OutOfRange(String),

    #[error("{0}")]
    #[js(type_error)]
    InvalidArg(String),

    #[error("{0}")]
    #[js(type_error)]
    InvalidUnit(String),

    #[error("{0}")]
    #[js(error)]
    Format(String),
}
