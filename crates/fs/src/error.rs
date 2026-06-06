use js_core::error::SystemError;

use crate::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    System(SystemError),

    #[error("{0}")]
    GlobPattern(String),

    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    Encoding(String),

    #[error("{0}")]
    Argument(String),
}

impl From<globset::Error> for Error {
    fn from(e: globset::Error) -> Self {
        Error::GlobPattern(e.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::Internal(e.to_string())
    }
}

pub trait IntoJsResult<T> {
    fn into_js(self, ctx: &js::Ctx<'_>) -> js::Result<T>;
}

impl<T, E: Into<Error>> IntoJsResult<T> for Result<T, E> {
    fn into_js(self, ctx: &js::Ctx<'_>) -> js::Result<T> {
        self.map_err(|e| e.into().into_exception(ctx))
    }
}

impl Error {
    pub fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        match self {
            Error::System(sys) => sys.into_js(ctx),
            Error::GlobPattern(msg) | Error::Argument(msg) => {
                let ctor: js::Constructor = ctx.globals().get("TypeError")?;
                let err: js::Object = ctor.construct((msg,))?;
                Ok(err.into_value())
            }
            Error::Encoding(msg) | Error::Internal(msg) => {
                let ctor: js::Constructor = ctx.globals().get("Error")?;
                let err: js::Object = ctor.construct((msg,))?;
                Ok(err.into_value())
            }
        }
    }

    pub fn into_exception<'js>(self, ctx: &js::Ctx<'js>) -> js::Error {
        match self.into_js(ctx) {
            Ok(val) => ctx.throw(val),
            Err(e) => e,
        }
    }
}
