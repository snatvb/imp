use std::collections::HashMap;

use js_core::abort::AbortSignal;
use js_core::byte_buffer::ByteBuffer;
use js_core::js::{Class, Ctx, FromJs, Value};
use js_core::utils::{DurationArg, JsStringArg, StringArg};

const DEFAULT_MAX_OUTPUT: usize = 10 * 1024 * 1024;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    #[default]
    Utf8,
    Binary,
}

impl Encoding {
    fn from_str(s: &str) -> Self {
        match s {
            "binary" => Encoding::Binary,
            _ => Encoding::Utf8,
        }
    }
}

#[derive(Default, Clone)]
pub struct RunOptions {
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub input: Option<Vec<u8>>,
    pub timeout: Option<DurationArg>,
    pub max_output: Option<usize>,
    pub signal: Option<AbortSignal>,
    pub encoding: Encoding,
}

impl RunOptions {
    #[inline(always)]
    pub fn max_output(&self) -> usize {
        self.max_output.unwrap_or(DEFAULT_MAX_OUTPUT)
    }
}

impl<'js> FromJs<'js> for RunOptions {
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> js_core::js::Result<Self> {
        let mut result = Self::default();
        let Some(opts) = value.as_object() else {
            return Ok(result);
        };

        if let Some(v) = opts.get::<_, Option<Value>>("cwd")?
            && !v.is_null()
            && !v.is_undefined()
        {
            result.cwd = Some(<StringArg as JsStringArg>::coerce_string(ctx, &v, "cwd")?);
        }

        if let Some(v) = opts.get::<_, Option<Value>>("input")?
            && !v.is_null()
            && !v.is_undefined()
        {
            if v.is_string() {
                let s = <StringArg as JsStringArg>::coerce_string(ctx, &v, "input")?;
                result.input = Some(s.into_bytes());
            } else if let Ok(cls) = Class::<ByteBuffer>::from_value(&v) {
                result.input = Some(cls.borrow().as_slice().to_vec());
            } else {
                let received = if v.is_object() {
                    "object"
                } else if v.is_int() || v.is_float() {
                    "number"
                } else if v.is_bool() {
                    "boolean"
                } else if v.is_array() {
                    "array"
                } else {
                    "value"
                };
                let msg = format!("input must be a string or ByteBuffer, got {received}");
                let ctor: js_core::js::Constructor = ctx.globals().get("TypeError")?;
                let err: js_core::js::Object = ctor.construct((msg,))?;
                return Err(ctx.throw(err.into_value()));
            }
        }

        if let Some(d) = opts.get::<_, Option<DurationArg>>("timeout")?
            && d.as_millis() > 0
        {
            result.timeout = Some(d);
        }

        if let Some(n) = opts.get::<_, Option<i64>>("maxOutput")?
            && n > 0
        {
            result.max_output = Some(n as usize);
        }

        if let Some(env) = opts.get::<_, Option<HashMap<String, String>>>("env")? {
            result.env = Some(env);
        }

        result.signal = opts.get::<_, Option<AbortSignal>>("signal")?;

        if let Some(v) = opts.get::<_, Option<Value>>("encoding")?
            && !v.is_null()
            && !v.is_undefined()
        {
            let s = <StringArg as JsStringArg>::coerce_string(ctx, &v, "encoding")?;
            result.encoding = Encoding::from_str(s.as_str());
        }

        Ok(result)
    }
}
