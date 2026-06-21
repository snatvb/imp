use js_core::error::JsError as _;
use js_core::js;
use js_core::js::{Ctx, Object, String as JsString};
use js_core::utils::StringArg;
use tokio::fs;

use crate::dotenv;
use crate::error::EnvError;
use crate::ini;

fn detect_kind(path: &str) -> &'static str {
    let p = path.replace('\\', "/");
    let lower = p.to_ascii_lowercase();
    let last = lower.rfind('/').map(|i| &lower[i + 1..]).unwrap_or(&lower);
    if last.ends_with(".ini") || last.ends_with(".cfg") {
        return "ini";
    }
    if last == ".env"
        || (last.starts_with(".env.") && last.len() > 5)
        || (last.starts_with("env.") && last.len() > 4)
        || last.ends_with(".env")
    {
        return "dotenv";
    }
    "dotenv"
}

fn build_from_string<'js>(ctx: &Ctx<'js>, s: &str, kind: &str) -> js::Result<Object<'js>> {
    match kind {
        "ini" => {
            let obj = Object::new(ctx.clone())?;
            let map = ini::parse_public(s, false).map_err(|e| e.into_exception(ctx))?;
            for (k, v) in &map {
                let js_v = ini::value_to_js_public(ctx, v)?;
                obj.set(k.as_str(), js_v)?;
            }
            Ok(obj)
        }
        _ => {
            let obj = Object::new(ctx.clone())?;
            let map = dotenv::parse_with_vars(s, true, &std::collections::HashMap::new())
                .map_err(|e| e.into_exception(ctx))?;
            for (k, v) in &map {
                obj.set(k.as_str(), JsString::from_str(ctx.clone(), v)?)?;
            }
            Ok(obj)
        }
    }
}

#[js::function]
pub async fn load_file<'js>(ctx: Ctx<'js>, path: StringArg) -> js::Result<Object<'js>> {
    let path_str = path.as_str().to_string();
    let kind = detect_kind(&path_str).to_string();
    let raw = fs::read_to_string(&path_str)
        .await
        .map_err(|e| EnvError::io(e, "read", Some(path_str.clone())).into_exception(&ctx))?;
    build_from_string(&ctx, &raw, &kind)
}
