use crate::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, JsError)]
pub enum Error {
    #[error("{0}")]
    #[js(error)]
    Parse(String),

    #[error("{0}")]
    #[js(error)]
    Serialize(String),

    #[error("{0}")]
    #[js(type_error)]
    Unsupported(String),
}
