use std::collections::BTreeMap;

use js_core::js;
use js_core::js::function::Rest;
use js_core::js::{Ctx, Object, String as JsString, Value};

#[js::function]
pub fn merge<'js>(ctx: Ctx<'js>, sources: Rest<Object<'js>>) -> js::Result<Object<'js>> {
    let mut merged: BTreeMap<String, String> = BTreeMap::new();
    for src in sources.0.iter() {
        for item in src.props::<String, Value>() {
            let (k, v) = item?;
            let s = if let Some(js_str) = v.as_string() {
                js_str.to_string()?
            } else if let Some(i) = v.as_int() {
                i.to_string()
            } else if let Some(f) = v.as_float() {
                f.to_string()
            } else if let Some(b) = v.as_bool() {
                b.to_string()
            } else {
                format!("{v:?}")
            };
            merged.insert(k, s);
        }
    }
    let out = Object::new(ctx.clone())?;
    for (k, v) in &merged {
        out.set(k.as_str(), JsString::from_str(ctx.clone(), v)?)?;
    }
    Ok(out)
}
