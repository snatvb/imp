use js_core::byte_buffer::ByteBuffer;

use crate::prelude::*;

use crate::convert::{js_to_value, value_to_js};
use crate::error::Error;

#[js::function]
pub fn parse<'js>(ctx: Ctx<'js>, input: js::Class<'js, ByteBuffer>) -> js::Result<Value<'js>> {
    let borrowed = input.borrow();
    let bytes = borrowed.as_slice();
    let val: serde_json::Value = rmp_serde::from_slice(bytes)
        .map_err(|e| Error::Parse(e.to_string()).into_exception(&ctx))?;
    value_to_js(&ctx, val)
}

#[js::function]
pub fn stringify<'js>(ctx: Ctx<'js>, value: Value<'js>) -> js::Result<js::Class<'js, ByteBuffer>> {
    let val = js_to_value(&ctx, value).map_err(|e| e.into_exception(&ctx))?;
    let bytes = rmp_serde::to_vec(&val)
        .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?;
    js::Class::instance(ctx, ByteBuffer::new(bytes))
}
