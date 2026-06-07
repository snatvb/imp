use js_core::error::{JsError, SystemError};
use rquickjs as js;

#[js::function]
fn cwd(ctx: js::Ctx<'_>) -> js::Result<String> {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| SystemError::from_io(e, "cwd", None).into_exception(&ctx))
}

pub fn create<'a>(ctx: &js::Ctx<'a>) -> js::Result<js::Object<'a>> {
    let process = js::Object::new(ctx.clone())?;
    process.set("cwd", js_cwd)?;
    Ok(process)
}
