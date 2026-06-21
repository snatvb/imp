use std::str::FromStr;

use js_core::byte_buffer::ByteBuffer;
use js_core::error::JsError;
use js_core::js::{Class, Ctx, FromJs, Value};
use js_core::utils::{JsStringArg, StringArg};

use crate::error::EncodingError;

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
    let received = js_type_name(value);
    Err(type_error(ctx, "input", "string | ByteBuffer", &received))
}

pub fn extract_input_string<'js>(
    ctx: &Ctx<'js>,
    value: &Value<'js>,
) -> js_core::js::Result<String> {
    if value.is_string() {
        return <StringArg as JsStringArg>::coerce_string(ctx, value, "input");
    }
    if let Ok(cls) = Class::<ByteBuffer>::from_value(value) {
        let bytes = cls.borrow().as_slice().to_vec();
        return String::from_utf8(bytes)
            .map_err(|e| EncodingError::InvalidUtf8(e.to_string()).into_exception(ctx));
    }
    let received = js_type_name(value);
    Err(type_error(ctx, "input", "string | ByteBuffer", &received))
}

fn js_type_name<'js>(value: &Value<'js>) -> String {
    if value.is_null() {
        return "null".to_string();
    }
    if value.is_undefined() {
        return "undefined".to_string();
    }
    if value.is_bool() {
        return "boolean".to_string();
    }
    if value.is_int() || value.is_float() {
        return "number".to_string();
    }
    if value.is_symbol() {
        return "symbol".to_string();
    }
    if value.is_array() {
        return "array".to_string();
    }
    if value.is_object() {
        return "object".to_string();
    }
    if value.is_function() {
        return "function".to_string();
    }
    "value".to_string()
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum B64Variant {
    #[default]
    Standard,
    Url,
}

impl FromStr for B64Variant {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "url" => B64Variant::Url,
            _ => B64Variant::Standard,
        })
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum B64DecodeMode {
    #[default]
    Base64,
    Utf8,
}

impl FromStr for B64DecodeMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "utf8" => B64DecodeMode::Utf8,
            _ => B64DecodeMode::Base64,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Base64Options {
    pub variant: B64Variant,
    pub pad: bool,
}

impl Default for Base64Options {
    fn default() -> Self {
        Base64Options {
            variant: B64Variant::default(),
            pad: true,
        }
    }
}

impl<'js> FromJs<'js> for Base64Options {
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> js_core::js::Result<Self> {
        let mut result = Self::default();
        let Some(opts) = value.as_object() else {
            return Ok(result);
        };

        if let Some(v) = opts.get::<_, Option<Value>>("variant")?
            && !v.is_null()
            && !v.is_undefined()
        {
            let s = <StringArg as JsStringArg>::coerce_string(ctx, &v, "variant")?;
            result.variant = s.parse().unwrap();
        }

        if let Some(b) = opts.get::<_, Option<bool>>("pad")? {
            result.pad = b;
        }

        Ok(result)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Base64DecodeOptions {
    pub mode: B64DecodeMode,
}

impl<'js> FromJs<'js> for Base64DecodeOptions {
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> js_core::js::Result<Self> {
        let mut result = Self::default();
        let Some(opts) = value.as_object() else {
            return Ok(result);
        };

        if let Some(v) = opts.get::<_, Option<Value>>("mode")?
            && !v.is_null()
            && !v.is_undefined()
        {
            let s = <StringArg as JsStringArg>::coerce_string(ctx, &v, "mode")?;
            result.mode = s.parse().unwrap();
        }

        Ok(result)
    }
}

#[derive(Clone, Copy, Default)]
pub struct HexOptions {
    pub uppercase: bool,
}

impl<'js> FromJs<'js> for HexOptions {
    fn from_js(_ctx: &Ctx<'js>, value: Value<'js>) -> js_core::js::Result<Self> {
        let mut result = Self::default();
        let Some(opts) = value.as_object() else {
            return Ok(result);
        };

        if let Some(b) = opts.get::<_, Option<bool>>("uppercase")? {
            result.uppercase = b;
        }

        Ok(result)
    }
}

pub fn type_error<'js>(
    ctx: &Ctx<'js>,
    name: &str,
    expected: &str,
    received: &str,
) -> js_core::js::Error {
    EncodingError::invalid_arg_type(name, expected, received).into_exception(ctx)
}
