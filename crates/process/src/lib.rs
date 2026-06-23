use js_core::error::{JsError, SystemError};
use rquickjs as js;

#[js::function]
fn cwd(ctx: js::Ctx<'_>) -> js::Result<String> {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| SystemError::from_io(e, "cwd", None).into_exception(&ctx))
}

#[js::function]
fn exit(code: js::function::Opt<i32>) {
    std::process::exit(code.0.unwrap_or(0))
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
    process.set("exit", js_exit)?;
    process.set("argv", js_args)?;

    let env_obj = js::Object::new(ctx.clone())?;
    for (key, value) in std::env::vars() {
        env_obj.set(key.as_str(), value.as_str())?;
    }
    process.set("env", env_obj)?;

    process.set("platform", std::env::consts::OS)?;
    process.set("arch", std::env::consts::ARCH)?;
    process.set("pid", std::process::id())?;

    let ppid = get_ppid();
    process.set("ppid", ppid)?;

    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    process.set("cpuCount", cpu_count)?;

    process.set("hostname", get_hostname())?;

    let homedir = dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    process.set("homedir", homedir)?;

    process.set("version", env!("CARGO_PKG_VERSION"))?;

    Ok(process)
}

fn get_ppid() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::getppid() as u32 }
    }
    #[cfg(windows)]
    {
        unsafe { windows_sys::Win32::System::Threading::GetCurrentProcessId() }
    }
}

fn get_hostname() -> String {
    #[cfg(unix)]
    {
        let mut buf = [0u8; 256];
        let ret = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
        if ret == 0 {
            if let Ok(s) = std::ffi::CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_str() {
                return s.to_string();
            }
        }
        String::new()
    }
    #[cfg(windows)]
    {
        std::env::var("COMPUTERNAME").unwrap_or_default()
    }
}
