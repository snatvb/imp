use os_path::OsPathBuf;

use crate::prelude::*;
use js_core::{register_native_modules, resolver::Resolver, timers::JsTimers};

pub fn native_module_names() -> &'static [&'static str] {
    &[
        "fs/promises",
        "path",
        "imp:fs",
        "imp:inq",
        "imp:sys/input_simulate",
        "imp:sys/stdin",
        "imp:clap",
        "imp:parsers",
        "imp:time",
        "imp:subprocess",
        "imp:encoding",
        "imp:env",
    ]
}

pub fn register_all_native_modules(
    module_loader: &mut js::loader::ModuleLoader,
    builtin_resolver: &mut js::loader::BuiltinResolver,
) {
    register_native_modules!(
        module_loader,
        builtin_resolver,
        ("fs/promises", fs::FsPromisesModule),
        ("path", js_path::PathModule),
        ("imp:fs", fs::imp::ImpFsModule),
        ("imp:inq", inq::InquireModule),
        ("imp:sys/input_simulate", sys::InputSimulateModule),
        ("imp:sys/stdin", sys::StdinModule),
        ("imp:clap", imp_clap::ClapModule),
        ("imp:parsers", parsers::ParsersModule),
        ("imp:time", imp_chrono::TimeModule),
        ("imp:subprocess", subprocess::SubprocessModule),
        ("imp:encoding", encoding::EncodingModule),
        ("imp:env", env::EnvModule),
    );
}

pub async fn setup_loaders(rt: &js::AsyncRuntime, resolver: Resolver, cwd: OsPathBuf) {
    let mut builtin_resolver = js::loader::BuiltinResolver::default();
    let mut module_loader = js::loader::ModuleLoader::default();
    register_all_native_modules(&mut module_loader, &mut builtin_resolver);

    rt.set_loader(
        (resolver, builtin_resolver),
        (
            js_core::loader::ScriptLoader { cwd },
            js_core::loader::DataLoader,
            module_loader,
            js::loader::ScriptLoader::default(),
        ),
    )
    .await;
}

pub async fn setup_embedded_loaders(
    rt: &js::AsyncRuntime,
    bundle: embed::Bundle,
) -> (String, String) {
    let entry_name = bundle.entry.clone();
    let entry_raw = bundle.modules.get(&entry_name).cloned().unwrap();

    let exe = std::env::current_exe().unwrap();
    let exe_dir = OsPathBuf::from_path_buf(exe.parent().unwrap().to_path_buf()).unwrap();

    let embedded_resolver = js_core::loader::EmbeddedResolver::new(bundle.modules.keys().cloned());
    let embedded_loader = js_core::loader::EmbeddedLoader::new(bundle.modules, exe_dir.clone());

    let mut builtin_resolver = js::loader::BuiltinResolver::default();
    let mut module_loader = js::loader::ModuleLoader::default();
    register_all_native_modules(&mut module_loader, &mut builtin_resolver);

    rt.set_loader(
        (embedded_resolver, builtin_resolver),
        (
            embedded_loader,
            js_core::loader::DataLoader,
            module_loader,
            js::loader::ScriptLoader::default(),
        ),
    )
    .await;

    (entry_name, entry_raw)
}

pub fn setup_globals<'js>(
    ctx: &js::Ctx<'js>,
    exe: &str,
    filepath: &str,
    script_args: &[impl std::borrow::Borrow<str>],
) -> (JsTimers, process::ExitHandle) {
    let exit_handle = process::ExitHandle::new();

    js_core::rs_string::init_rs_string_or_panic(ctx);
    js_core::byte_buffer::init_or_panic(ctx);
    imp_clap::init(ctx, script_args).unwrap();
    let globals = ctx.globals();
    let js_timers = JsTimers::new();
    js_timers.bind_to(ctx, &globals).unwrap();
    globals
        .set("console", console::create(ctx).unwrap())
        .unwrap();
    globals.set("assert", js_core::assert::js_assert).unwrap();
    globals
        .set(
            "process",
            process::create(ctx, exe, filepath, script_args, exit_handle.clone()).unwrap(),
        )
        .unwrap();
    globals
        .set("performance", js_core::performance::create(ctx).unwrap())
        .unwrap();
    fetch::create_globals(ctx).unwrap();
    imp_url::create_globals(ctx).unwrap();
    imp_chrono::create_globals(ctx).unwrap();
    (js_timers, exit_handle)
}

pub async fn run_js_entry<'js>(
    ctx: &js::Ctx<'js>,
    entry_name: &str,
    entry_code: &str,
    script_args: &[String],
    cwd: &OsPathBuf,
) -> i32 {
    use crate::{error, event_loop};

    let exe_path = std::env::current_exe().unwrap();
    let (js_timers, exit_handle) = setup_globals(
        ctx,
        exe_path.to_string_lossy().as_ref(),
        entry_name,
        script_args,
    );

    tracing::info!(file = %entry_name, "evaluating module");
    let module = error::try_js(
        ctx,
        js::Module::declare(ctx.clone(), entry_name, entry_code),
        "module declaration failed",
    )
    .unwrap();
    error::try_js(
        ctx,
        js_core::meta::set_meta(ctx, &module, cwd, entry_name),
        "module meta setup failed",
    );
    let (_, promise) = error::try_js(ctx, module.eval(), "module evaluation failed").unwrap();
    tracing::info!("module evaluated");

    let exit_handle_for_loop = exit_handle.clone();
    event_loop::run_event_loop(ctx, js_timers, Some(exit_handle_for_loop), Some(promise)).await;
    exit_handle.exit_code()
}
