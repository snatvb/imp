use js_core::error::SystemError;
use rquickjs as js;

pub fn create<'a>(ctx: &js::Ctx<'a>) -> js::Result<js::Object<'a>> {
    let process = js::Object::new(ctx.clone())?;
    let cwd_fn = js::Function::new(ctx.clone(), |ctx: js::Ctx<'_>| -> js::Result<String> {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| SystemError::from_io(e, "cwd", None).into_exception(&ctx))
    })?;
    process.set("cwd", cwd_fn)?;
    Ok(process)
}
