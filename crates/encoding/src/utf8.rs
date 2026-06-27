use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js;
use js_core::utils::StringArg;

use crate::error::EncodingError;

#[js::function]
pub fn encode<'js>(
    ctx: js::Ctx<'js>,
    input: StringArg,
) -> js::Result<js::Class<'js, ByteBuffer<'js>>> {
    let s = input.as_str();
    js::Class::instance(ctx.clone(), ByteBuffer::new(ctx, s.as_bytes().to_vec())?)
}

#[js::function]
pub fn decode<'js>(
    ctx: js::Ctx<'js>,
    input: js::Class<'js, ByteBuffer<'js>>,
) -> js::Result<js::String<'js>> {
    let bytes = input.borrow().as_slice().to_vec();
    let s = String::from_utf8(bytes)
        .map_err(|e| EncodingError::InvalidUtf8(e.to_string()).into_exception(&ctx))?;
    js::String::from_str(ctx, &s)
}
