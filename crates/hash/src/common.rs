use base64::Engine;
use digest::Digest;
use js_core::byte_buffer::ByteBuffer;
use js_core::js;
use js_core::js::Class;
use js_core::js::function::Opt;

use crate::options::{Encoding, extract_input_bytes};

pub(crate) fn encode_output<'js>(
    ctx: &js::Ctx<'js>,
    data: &[u8],
    encoding: Encoding,
) -> js::Result<js::Value<'js>> {
    match encoding {
        Encoding::Hex => Ok(js::String::from_str(ctx.clone(), &hex::encode(data))?.into_value()),
        Encoding::Base64 => {
            let b64 = base64::engine::general_purpose::STANDARD.encode(data);
            Ok(js::String::from_str(ctx.clone(), &b64)?.into_value())
        }
        Encoding::Bytes => {
            Ok(Class::instance(ctx.clone(), ByteBuffer::new(data.to_vec()))?.into_value())
        }
    }
}

pub(crate) fn hash_input<'js, D: Digest>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    let bytes = extract_input_bytes(&ctx, &input)?;
    let encoding = encoding
        .0
        .as_deref()
        .map(Encoding::parse)
        .unwrap_or_default();
    encode_output(&ctx, D::digest(&bytes).as_slice(), encoding)
}

pub(crate) fn blake3_input<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    let bytes = extract_input_bytes(&ctx, &input)?;
    let encoding = encoding
        .0
        .as_deref()
        .map(Encoding::parse)
        .unwrap_or_default();
    encode_output(&ctx, blake3::hash(&bytes).as_bytes(), encoding)
}
