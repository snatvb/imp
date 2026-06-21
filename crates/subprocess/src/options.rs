use std::collections::HashMap;

use js_core::js::{Ctx, FromJs, Value};
use js_core::utils::{JsStringArg, StringArg};

const DEFAULT_MAX_OUTPUT: usize = 10 * 1024 * 1024;

#[derive(Default, Debug, Clone)]
pub struct RunOptions {
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub input: Option<String>,
    pub timeout: Option<u64>,
    pub max_output: Option<usize>,
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

        if let Some(n) = opts.get::<_, Option<i64>>("timeout")?
            && n > 0
        {
            result.timeout = Some(n as u64);
        }

        if let Some(n) = opts.get::<_, Option<i64>>("maxOutput")?
            && n > 0
        {
            result.max_output = Some(n as usize);
        }

        if let Some(env) = opts.get::<_, Option<HashMap<String, String>>>("env")? {
            result.env = Some(env);
        }

        Ok(result)
    }
}
