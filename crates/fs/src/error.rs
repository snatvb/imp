use crate::prelude::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{code}: {message}")]
    System {
        code: &'static str,
        syscall: &'static str,
        message: String,
        path: Option<String>,
        source: Option<std::io::Error>,
    },

    #[error("{0}")]
    Encoding(String),

    #[error("{0}")]
    Argument(String),
}

impl Error {
    pub fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        match self {
            Error::System {
                code,
                syscall,
                message,
                path,
                ..
            } => {
                let ctor: js::Constructor = ctx.globals().get("Error")?;
                let err: js::Object = ctor.construct((message,))?;
                err.set("code", code)?;
                err.set("syscall", syscall)?;
                if let Some(ref p) = path {
                    err.set("path", p)?;
                }
                Ok(err.into_value())
            }
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

    pub fn from_io(e: std::io::Error, syscall: &'static str, path: Option<String>) -> Self {
        let code = match e.kind() {
            std::io::ErrorKind::NotFound => "ENOENT",
            std::io::ErrorKind::PermissionDenied => "EACCES",
            std::io::ErrorKind::AlreadyExists => "EEXIST",
            std::io::ErrorKind::InvalidInput => "EINVAL",
            _ => "UNKNOWN",
        };
        Error::System {
            code,
            syscall,
            message: e.to_string(),
            path,
            source: Some(e),
        }
    }
}
