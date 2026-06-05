use std::fmt;

use crate::coerce::{js_type_of, throw_type_error};
use crate::js::{self, Class, Ctx, FromJs, Type, Value};
use crate::rs_string::RsString;

pub enum StringArg {
    JsString(String),
    RsString(RsString),
}

impl StringArg {
    pub fn as_str(&self) -> &str {
        match self {
            StringArg::JsString(s) => s,
            StringArg::RsString(rs) => rs.get_slice(),
        }
    }
}

pub trait JsStringArg<'js>: Sized {
    fn to_string_arg(self, ctx: &Ctx<'js>) -> js::Result<StringArg>;

    fn coerce_js(ctx: &Ctx<'js>, val: &Value<'js>, name: impl fmt::Display) -> js::Result<StringArg> {
        val.clone().to_string_arg(ctx).map_err(|_| {
            let received = js_type_of(val);
            let msg = format!(
                "The \"{name}\" argument must be of type string. Received {received}"
            );
            throw_type_error(ctx, "ERR_INVALID_ARG_TYPE", &msg)
        })
    }
}

impl<'js> JsStringArg<'js> for js::String<'js> {
    fn to_string_arg(self, _: &Ctx<'js>) -> js::Result<StringArg> {
        self.to_string().map(StringArg::JsString)
    }
}

impl<'js> JsStringArg<'js> for Class<'js, RsString> {
    fn to_string_arg(self, _: &Ctx<'js>) -> js::Result<StringArg> {
        let rs = self.borrow();
        Ok(StringArg::RsString(rs.clone()))
    }
}

impl<'js> JsStringArg<'js> for Value<'js> {
    fn to_string_arg(self, ctx: &Ctx<'js>) -> js::Result<StringArg> {
        match self.type_of() {
            Type::String => self.as_string().unwrap().to_string().map(StringArg::JsString),
            Type::Object | Type::Constructor | Type::Promise | Type::Proxy => {
                if let Ok(class) = Class::<RsString>::from_js(ctx, self) {
                    let rs = class.borrow();
                    Ok(StringArg::RsString(rs.clone()))
                } else {
                    Err(js::Error::new_from_js("object", "string"))
                }
            }
            _ => Err(js::Error::new_from_js("value", "string")),
        }
    }
}

impl<'js> JsStringArg<'js> for StringArg {
    fn to_string_arg(self, _: &Ctx<'js>) -> js::Result<StringArg> {
        Ok(self)
    }
}
