use js_core::error::{JsError, SystemError};
use rquickjs as js;

#[js::function]
fn cwd(ctx: js::Ctx<'_>) -> js::Result<String> {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| SystemError::from_io(e, "cwd", None).into_exception(&ctx))
}

pub fn create<'a>(
    ctx: &js::Ctx<'a>,
    exe_path: &str,
    filepath: &str,
    rest_args: &[impl std::borrow::Borrow<str>],
) -> js::Result<js::Object<'a>> {
    let process = js::Object::new(ctx.clone())?;
    let js_args = js::Array::new(ctx.clone())?;

    js_args.set(0, exe_path)?;
    js_args.set(1, filepath)?;
    for (i, arg) in rest_args.iter().enumerate() {
        js_args.set(i + 2, arg.borrow().to_string())?;
    }

    process.set("cwd", js_cwd)?;
    process.set("argv", js_args)?;
    Ok(process)
}
