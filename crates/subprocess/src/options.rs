use std::collections::HashMap;

use js_core::abort::AbortSignal;
use js_core::js::{Ctx, FromJs, Value};
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
    pub input: Option<String>,
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
            result.input = Some(<StringArg as JsStringArg>::coerce_string(ctx, &v, "input")?);
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
