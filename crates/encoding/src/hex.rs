use hex::FromHex;
use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js;
use js_core::js::function::Opt;
use js_core::utils::StringArg;

use crate::error::EncodingError;
use crate::options::{HexOptions, extract_input_bytes};

#[js::function]
pub fn encode<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    options: Opt<HexOptions>,
) -> js::Result<js::String<'js>> {
    let bytes = extract_input_bytes(&ctx, &input)?;
    let opts = options.0.unwrap_or_default();
    let encoded = if opts.uppercase {
        hex::encode_upper(&bytes)
    } else {
        hex::encode(&bytes)
    };
    js::String::from_str(ctx, &encoded)
}

#[js::function]
pub fn decode<'js>(ctx: js::Ctx<'js>, input: StringArg) -> js::Result<js::Class<'js, ByteBuffer>> {
    let s = input.as_str();
    let bytes = Vec::from_hex(s)
        .map_err(|e| EncodingError::InvalidHex(e.to_string()).into_exception(&ctx))?;
    js::Class::instance(ctx, ByteBuffer::new(bytes))
}

pub fn make_module<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Object<'js>> {
    let obj = js::Object::new(ctx.clone())?;
    obj.set("encode", js_decode)?;
    obj.set("decode", js_decode)?;
    Ok(obj)
}
