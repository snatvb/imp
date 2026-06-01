use std::io;

use crate::js;
use js::{Constructor, Ctx, Object, Value};

#[derive(Debug)]
pub struct SystemError {
    pub code: &'static str,
    pub syscall: &'static str,
    pub message: String,
    pub path: Option<String>,
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl SystemError {
    pub fn from_io(e: io::Error, syscall: &'static str, path: Option<String>) -> Self {
        let code = match e.kind() {
            io::ErrorKind::NotFound => "ENOENT",
            io::ErrorKind::PermissionDenied => "EACCES",
            io::ErrorKind::AlreadyExists => "EEXIST",
            io::ErrorKind::InvalidInput => "EINVAL",
            _ => "UNKNOWN",
        };
        SystemError {
            code,
            syscall,
            message: e.to_string(),
            path,
        }
    }

    pub fn into_js<'js>(self, ctx: &Ctx<'js>) -> js::Result<Value<'js>> {
        let ctor: Constructor = ctx.globals().get("Error")?;
        let err: Object = ctor.construct((self.message,))?;
        err.set("code", self.code)?;
        err.set("syscall", self.syscall)?;
        if let Some(ref p) = self.path {
            err.set("path", p)?;
        }
        Ok(err.into_value())
    }

    pub fn into_exception<'js>(self, ctx: &Ctx<'js>) -> js::Error {
        match self.into_js(ctx) {
            Ok(val) => ctx.throw(val),
            Err(e) => e,
        }
    }
}
