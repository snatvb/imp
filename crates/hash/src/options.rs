use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js::{Class, Ctx, Value};
use js_core::utils::{JsStringArg, StringArg};

use crate::error::HashError;

pub fn extract_input_bytes<'js>(
    ctx: &Ctx<'js>,
    value: &Value<'js>,
) -> js_core::js::Result<Vec<u8>> {
    if value.is_string() {
        let s = <StringArg as JsStringArg>::coerce_string(ctx, value, "input")?;
        return Ok(s.into_bytes());
    }
    if let Ok(cls) = Class::<ByteBuffer>::from_value(value) {
        return Ok(cls.borrow().as_slice().to_vec());
    }
    let received = js_core::coerce::js_type_of(value);
    Err(type_error(ctx, "input", "string | ByteBuffer", &received))
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    #[default]
    Hex,
    Base64,
    Bytes,
}

impl Encoding {
    pub fn parse(s: &str) -> Self {
        match s {
            "base64" => Encoding::Base64,
            "bytes" => Encoding::Bytes,
            _ => Encoding::Hex,
        }
    }
}

pub fn type_error<'js>(
    ctx: &Ctx<'js>,
    name: &str,
    expected: &str,
    received: &str,
) -> js_core::js::Error {
    HashError::invalid_arg_type(name, expected, received).into_exception(ctx)
}
