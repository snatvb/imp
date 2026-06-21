use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js;
use js_core::utils::StringArg;

use crate::error::EncodingError;

#[js::function]
pub fn encode<'js>(ctx: js::Ctx<'js>, input: StringArg) -> js::Result<js::Class<'js, ByteBuffer>> {
    let s = input.as_str();
    js::Class::instance(ctx, ByteBuffer::new(s.as_bytes().to_vec()))
}

#[js::function]
pub fn decode<'js>(
    ctx: js::Ctx<'js>,
    input: js::Class<'js, ByteBuffer>,
) -> js::Result<js::String<'js>> {
    let bytes = input.borrow().as_slice().to_vec();
    let s = String::from_utf8(bytes)
        .map_err(|e| EncodingError::InvalidUtf8(e.to_string()).into_exception(&ctx))?;
    js::String::from_str(ctx, &s)
}

pub fn make_module<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Object<'js>> {
    let obj = js::Object::new(ctx.clone())?;
    obj.set("encode", js_encode)?;
    obj.set("decode", js_decode)?;
    Ok(obj)
}
