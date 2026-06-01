use std::fmt;

use rquickjs::{Constructor, Ctx, Object, Type, Value};

#[derive(Debug, Clone, Copy)]
pub struct CoerceError;

pub fn js_type_of<'js>(val: &Value<'js>) -> String {
    match val.type_of() {
        Type::Null => "null".into(),
        Type::Undefined => "undefined".into(),
        Type::Bool => format!("type boolean ({})", val.as_bool().unwrap()),
        Type::Int => format!("type number ({})", val.as_int().unwrap()),
        Type::Float => format!("type number ({})", val.as_float().unwrap()),
        Type::String => "string".into(),
        Type::Symbol => "symbol".into(),
        Type::BigInt => "type bigint".into(),
        Type::Function => "function".into(),
        Type::Array => "an instance of Array".into(),
        Type::Object | Type::Constructor | Type::Promise | Type::Proxy => {
            let name: Option<String> = val.as_object().and_then(|obj| {
                obj.get::<_, Object>("constructor")
                    .ok()
                    .and_then(|c| c.get::<_, String>("name").ok())
            });
            match name {
                Some(n) if !n.is_empty() => format!("an instance of {n}"),
                _ => "an instance of Object".into(),
            }
        }
        Type::Exception => "exception".into(),
        Type::Module => "module".into(),
        _ => "unknown".into(),
    }
}

pub fn throw_type_error<'js>(ctx: &Ctx<'js>, code: &str, msg: &str) -> rquickjs::Error {
    let make = || -> rquickjs::Result<rquickjs::Error> {
        let ctor: Constructor = ctx.globals().get("TypeError")?;
        let err: Object = ctor.construct((msg,))?;
        err.set("code", code)?;
        Ok(ctx.throw(err.into()))
    };
    make().unwrap_or_else(|e| e)
}

pub trait JsCoerce<'js>: Sized {
    fn js_type() -> &'static str;

    fn coerce(val: &Value<'js>) -> Result<Self, CoerceError>;

    fn coerce_js(ctx: &Ctx<'js>, val: &Value<'js>, name: impl fmt::Display) -> rquickjs::Result<Self> {
        Self::coerce(val).map_err(|_| {
            let received = js_type_of(val);
            let msg = format!(
                "The \"{name}\" argument must be of type {}. Received {received}",
                Self::js_type()
            );
            throw_type_error(ctx, "ERR_INVALID_ARG_TYPE", &msg)
        })
    }
}

impl<'js> JsCoerce<'js> for String {
    fn js_type() -> &'static str {
        "string"
    }

    fn coerce(val: &Value<'js>) -> Result<Self, CoerceError> {
        val.as_string()
            .and_then(|s| s.to_string().ok())
            .ok_or(CoerceError)
    }
}

impl<'js> JsCoerce<'js> for i32 {
    fn js_type() -> &'static str {
        "number"
    }

    fn coerce(val: &Value<'js>) -> Result<Self, CoerceError> {
        val.as_int().ok_or(CoerceError)
    }
}

impl<'js> JsCoerce<'js> for f64 {
    fn js_type() -> &'static str {
        "number"
    }

    fn coerce(val: &Value<'js>) -> Result<Self, CoerceError> {
        val.as_float().ok_or(CoerceError)
    }
}

impl<'js> JsCoerce<'js> for bool {
    fn js_type() -> &'static str {
        "boolean"
    }

    fn coerce(val: &Value<'js>) -> Result<Self, CoerceError> {
        val.as_bool().ok_or(CoerceError)
    }
}

impl<'js> JsCoerce<'js> for Object<'js> {
    fn js_type() -> &'static str {
        "object"
    }

    fn coerce(val: &Value<'js>) -> Result<Self, CoerceError> {
        val.as_object().cloned().ok_or(CoerceError)
    }
}
