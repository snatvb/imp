use js::object::Accessor;
#[allow(unused_imports)]
use js_core::error::{JsError, SystemError};
use rquickjs as js;

mod exit;
pub use exit::ExitHandle;

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
    exit_handle: ExitHandle,
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

    let handle = exit_handle.clone();
    let exit_fn = js::Function::new(ctx.clone(), move |code: js::function::Opt<i32>| {
        handle.request_exit(code.0.unwrap_or(0));
    })?;
    process.set("exit", exit_fn)?;

    let handle = exit_handle.clone();
    let ctx_for_on = ctx.clone();
    let on_fn = js::Function::new(ctx.clone(), move |event: String, cb: js::Function<'_>| {
        if event == "exit" {
            let persistent = ffi_extra::js_helpers::save_persistent_fn(&ctx_for_on, cb);
            handle.add_listener(persistent);
        }
    })?;
    process.set("on", on_fn)?;

    let handle_get = exit_handle.clone();
    let handle_set = exit_handle.clone();
    process.prop(
        "exitCode",
        Accessor::from(move || -> i32 { handle_get.exit_code() }).set(move |code: i32| {
            handle_set.set_exit_code(code);
        }),
    )?;

    let env_obj = js::Object::new(ctx.clone())?;
    for (key, value) in std::env::vars() {
        env_obj.set(key.as_str(), value.as_str())?;
    }
    if std::env::var("USER").is_err()
        && let Ok(username) = std::env::var("USERNAME")
    {
        env_obj.set("USER", username.as_str())?;
    }
    process.set("env", env_obj)?;

    process.set("platform", std::env::consts::OS)?;
    process.set("arch", std::env::consts::ARCH)?;
    process.set("pid", std::process::id())?;

    let ppid = ffi_extra::os_info::get_ppid();
    process.set("ppid", ppid)?;

    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    process.set("cpuCount", cpu_count)?;

    process.set("hostname", ffi_extra::os_info::get_hostname())?;

    let homedir = dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    process.set("homedir", homedir)?;

    process.set("version", env!("CARGO_PKG_VERSION"))?;

    Ok(process)
}
