use std::io;

use crate::js;
use js::{Constructor, Object};

pub trait JsError: std::error::Error {
    fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>>;

    fn into_exception<'js>(self, ctx: &js::Ctx<'js>) -> js::Error
    where
        Self: Sized,
    {
        match self.into_js(ctx) {
            Ok(val) => ctx.throw(val),
            Err(e) => e,
        }
    }
}

pub fn throw_abort_error<'js>(ctx: &js::Ctx<'js>, reason: &str) -> js::Error {
    unsafe {
        let name = std::ffi::CString::new("AbortError").unwrap();
        let msg = std::ffi::CString::new(reason).unwrap();
        let fmt = c"%s";
        js::qjs::JS_ThrowDOMException(
            ctx.as_raw().as_ptr(),
            name.as_ptr(),
            fmt.as_ptr(),
            msg.as_ptr(),
        );
    }
    js::Error::Exception
}

pub fn throw_timeout_error<'js>(ctx: &js::Ctx<'js>, reason: &str) -> js::Error {
    unsafe {
        let name = std::ffi::CString::new("TimeoutError").unwrap();
        let msg = std::ffi::CString::new(reason).unwrap();
        let fmt = c"%s";
        js::qjs::JS_ThrowDOMException(
            ctx.as_raw().as_ptr(),
            name.as_ptr(),
            fmt.as_ptr(),
            msg.as_ptr(),
        );
    }
    js::Error::Exception
}

pub fn make_type_error<'js>(ctx: &js::Ctx<'js>, msg: String) -> js::Result<js::Value<'js>> {
    let ctor: Constructor = ctx.globals().get("TypeError")?;
    let err: Object = ctor.construct((msg,))?;
    Ok(err.into_value())
}

pub fn make_error<'js>(ctx: &js::Ctx<'js>, msg: String) -> js::Result<js::Value<'js>> {
    let ctor: Constructor = ctx.globals().get("Error")?;
    let err: Object = ctor.construct((msg,))?;
    Ok(err.into_value())
}

pub fn make_range_error<'js>(ctx: &js::Ctx<'js>, msg: String) -> js::Result<js::Value<'js>> {
    let ctor: Constructor = ctx.globals().get("RangeError")?;
    let err: Object = ctor.construct((msg,))?;
    Ok(err.into_value())
}

pub trait IntoJsResult<T> {
    fn into_js(self, ctx: &js::Ctx<'_>) -> js::Result<T>;
}

#[macro_export]
macro_rules! declare_into_js_result {
    () => {
        pub trait IntoJsResult<T> {
            fn into_js(self, ctx: &$crate::js::Ctx<'_>) -> $crate::js::Result<T>;
        }
    };
}

#[macro_export]
macro_rules! impl_into_js_result {
    ($trait:ident, $target:ty $(, $external:ty)* $(,)?) => {
        impl<T> $trait<T> for Result<T, $target> {
            fn into_js(self, ctx: &$crate::js::Ctx<'_>) -> $crate::js::Result<T> {
                self.map_err(|e| <$target as $crate::error::JsError>::into_exception(e, ctx))
            }
        }
        $(
            impl<T> $trait<T> for Result<T, $external> {
                fn into_js(self, ctx: &$crate::js::Ctx<'_>) -> $crate::js::Result<T> {
                    self.map_err(|e| <$target as $crate::error::JsError>::into_exception(<$target>::from(e), ctx))
                }
            }
        )*
    };
}

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

impl std::error::Error for SystemError {}

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
}

impl JsError for SystemError {
    fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        let ctor: Constructor = ctx.globals().get("Error")?;
        let err: Object = ctor.construct((self.message,))?;
        err.set("code", self.code)?;
        err.set("syscall", self.syscall)?;
        if let Some(ref p) = self.path {
            err.set("path", p)?;
        }
        Ok(err.into_value())
    }
}

#[derive(Debug)]
pub struct Exception(pub String);

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Exception {}

impl JsError for Exception {
    fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        make_error(ctx, self.0)
    }
}
