use js_core::error::SystemError;

use crate::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    System(SystemError),

    #[error("{0}")]
    Encoding(String),

    #[error("{0}")]
    Argument(String),
}

impl Error {
    pub fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        match self {
            Error::System(sys) => sys.into_js(ctx),
            Error::Encoding(msg) | Error::Argument(msg) => {
                let ctor: js::Constructor = ctx.globals().get("TypeError")?;
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
