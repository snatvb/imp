use js_core::error::SystemError;
use thiserror::Error;

use rquickjs::{Constructor, Ctx, Object, Value};

#[derive(Error, Debug)]
pub enum PathError {
    #[error("{0}")]
    System(SystemError),

    #[error("{0}")]
    InvalidPath(String),
}

impl PathError {
    pub fn from_io(e: std::io::Error, syscall: &'static str) -> Self {
        PathError::System(SystemError::from_io(e, syscall, None))
    }

    pub fn invalid_path(msg: impl Into<String>) -> Self {
        PathError::InvalidPath(msg.into())
    }

    pub fn into_js<'js>(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        match self {
            PathError::System(sys) => sys.into_js(ctx),
            PathError::InvalidPath(msg) => {
                let ctor: Constructor = ctx.globals().get("Error")?;
                let err: Object = ctor.construct((msg,))?;
                Ok(err.into_value())
            }
        }
    }

    pub fn into_exception<'js>(self, ctx: &Ctx<'js>) -> rquickjs::Error {
        match self.into_js(ctx) {
            Ok(val) => ctx.throw(val),
            Err(e) => e,
        }
    }
}
