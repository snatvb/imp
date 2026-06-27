use base64::Engine;
use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js;
use js_core::js::Class;
use js_core::utils::{JsStringArg, StringArg};

use crate::error::CryptoError;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputEncoding {
    #[default]
    Hex,
    Bytes,
    Base64,
}

impl OutputEncoding {
    pub fn parse(s: &str) -> Self {
        match s {
            "hex" => OutputEncoding::Hex,
            "base64" => OutputEncoding::Base64,
            _ => OutputEncoding::Bytes,
        }
    }
}

pub fn parse_encoding(s: Option<&str>) -> OutputEncoding {
    s.map(OutputEncoding::parse).unwrap_or_default()
}

pub fn extract_bytes<'js>(
    ctx: &js::Ctx<'js>,
    value: &js::Value<'js>,
    arg_name: &str,
) -> js::Result<Vec<u8>> {
    if value.is_string() {
        let s = <StringArg as JsStringArg>::coerce_string(ctx, value, arg_name)?;
        return Ok(s.into_bytes());
    }
    if let Ok(cls) = Class::<ByteBuffer>::from_value(value) {
        return Ok(cls.borrow().as_slice().to_vec());
    }
    let received = js_core::coerce::js_type_of(value);
    Err(
        CryptoError::invalid_arg_type(arg_name, "string | ByteBuffer", &received)
            .into_exception(ctx),
    )
}

pub fn encode_output<'js>(
    ctx: &js::Ctx<'js>,
    data: &[u8],
    encoding: OutputEncoding,
) -> js::Result<js::Value<'js>> {
    match encoding {
        OutputEncoding::Hex => {
            Ok(js::String::from_str(ctx.clone(), &hex::encode(data))?.into_value())
        }
        OutputEncoding::Base64 => {
            let b64 = base64::engine::general_purpose::STANDARD.encode(data);
            Ok(js::String::from_str(ctx.clone(), &b64)?.into_value())
        }
        OutputEncoding::Bytes => Ok(Class::instance(
            ctx.clone(),
            ByteBuffer::new(ctx.clone(), data.to_vec())?,
        )?
        .into_value()),
    }
}
