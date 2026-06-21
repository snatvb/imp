use base64::Engine as _;
use base64::engine::DecodePaddingMode;
use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};
use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js;
use js_core::js::function::Opt;
use js_core::utils::StringArg;

use crate::error::EncodingError;
use crate::options::{
    B64DecodeMode, B64Variant, Base64DecodeOptions, Base64Options, extract_input_bytes,
};

fn engine_standard(pad: bool) -> base64::engine::general_purpose::GeneralPurpose {
    if pad { STANDARD } else { STANDARD_NO_PAD }.clone()
}

fn engine_url(pad: bool) -> base64::engine::general_purpose::GeneralPurpose {
    if pad { URL_SAFE } else { URL_SAFE_NO_PAD }.clone()
}

fn decode_engine(variant: B64Variant) -> base64::engine::general_purpose::GeneralPurpose {
    let cfg = base64::engine::GeneralPurposeConfig::new()
        .with_decode_padding_mode(DecodePaddingMode::Indifferent);
    let alphabet = match variant {
        B64Variant::Standard => base64::alphabet::STANDARD,
        B64Variant::Url => base64::alphabet::URL_SAFE,
    };
    base64::engine::general_purpose::GeneralPurpose::new(&alphabet, cfg)
}

#[js::function]
pub fn encode<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    options: Opt<Base64Options>,
) -> js::Result<js::String<'js>> {
    let bytes = extract_input_bytes(&ctx, &input)?;
    let opts = options.0.unwrap_or_default();
    let encoded = match opts.variant {
        B64Variant::Standard => engine_standard(opts.pad).encode(&bytes),
        B64Variant::Url => engine_url(opts.pad).encode(&bytes),
    };
    js::String::from_str(ctx, &encoded)
}

#[js::function]
pub fn decode<'js>(
    ctx: js::Ctx<'js>,
    input: StringArg,
    options: Opt<Base64DecodeOptions>,
) -> js::Result<js::Value<'js>> {
    let s = input.as_str();
    let opts = options.0.unwrap_or_default();
    let bytes = decode_engine(B64Variant::Standard)
        .decode(s)
        .map_err(|e| EncodingError::InvalidBase64(e.to_string()).into_exception(&ctx))?;
    match opts.mode {
        B64DecodeMode::Base64 => {
            let buf = js::Class::instance(ctx.clone(), ByteBuffer::new(bytes))?;
            Ok(buf.into_value())
        }
        B64DecodeMode::Utf8 => {
            let s = String::from_utf8(bytes)
                .map_err(|e| EncodingError::InvalidUtf8(e.to_string()).into_exception(&ctx))?;
            Ok(js::String::from_str(ctx, &s)?.into_value())
        }
    }
}

pub fn make_module<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Object<'js>> {
    let obj = js::Object::new(ctx.clone())?;
    obj.set("encode", js_encode)?;
    obj.set("decode", js_decode)?;
    Ok(obj)
}
