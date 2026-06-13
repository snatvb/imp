use js_core::rs_string::RsString;

use crate::prelude::*;

use crate::convert::{js_to_value, value_to_js};
use crate::error::Error;

#[js::function]
pub fn parse<'js>(ctx: Ctx<'js>, input: StringArg) -> js::Result<Value<'js>> {
    let s = input.as_str();
    let val: serde_json::Value =
        ron::from_str(s).map_err(|e| Error::Parse(e.to_string()).into_exception(&ctx))?;
    value_to_js(&ctx, val)
}

#[js::function]
pub fn stringify<'js>(ctx: Ctx<'js>, value: Value<'js>) -> js::Result<js::Class<'js, RsString>> {
    let val = js_to_value(&ctx, value).map_err(|e| e.into_exception(&ctx))?;
    let s =
        ron::to_string(&val).map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?;
    js::Class::instance(ctx, RsString::owned(s))
}
