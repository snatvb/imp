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
    let exe_name = exe.file_name().unwrap().to_string_lossy().to_string();

    let embedded_resolver = js_core::loader::EmbeddedResolver::new(bundle.modules.keys().cloned());
    let embedded_loader = js_core::loader::EmbeddedLoader::new(bundle.modules, exe_dir.clone());

    let mut builtin_resolver = js::loader::BuiltinResolver::default();
    let mut module_loader = js::loader::ModuleLoader::default();
    register_all_native_modules(&mut module_loader, &mut builtin_resolver);

    rt.set_loader(
        (embedded_resolver, builtin_resolver),
        (
            embedded_loader,
            module_loader,
            js::loader::ScriptLoader::default(),
        ),
    )
    .await;

    let entry_code = js_core::meta::with_meta(&exe_dir, &exe_name)(entry_raw);

    (entry_name, entry_code)
}

pub fn setup_globals<'js>(
    ctx: &js::Ctx<'js>,
    exe: &str,
    filepath: &str,
    script_args: &[impl std::borrow::Borrow<str>],
) -> JsTimers {
    js_core::rs_string::init_rs_string_or_panic(ctx);
    js_core::byte_buffer::init_or_panic(ctx);
    imp_clap::init(ctx, script_args).unwrap();
    let globals = ctx.globals();
    let js_timers = JsTimers::new();
    js_timers.bind_to(ctx, &globals).unwrap();
    globals
        .set("console", console::create(ctx).unwrap())
        .unwrap();
    globals
        .set(
            "process",
            process::create(ctx, exe, filepath, script_args).unwrap(),
        )
        .unwrap();
    globals
        .set("performance", js_core::performance::create(ctx).unwrap())
        .unwrap();
    js_timers
}

pub async fn run_js_entry<'js>(
    ctx: &js::Ctx<'js>,
    rt: &js::AsyncRuntime,
    entry_name: &str,
    entry_code: &str,
    script_args: &[String],
) {
    use crate::{error, event_loop};

    let exe_path = std::env::current_exe().unwrap();
    let js_timers = setup_globals(
        ctx,
        exe_path.to_string_lossy().as_ref(),
        entry_name,
        script_args,
    );

    tracing::info!(file = %entry_name, "evaluating module");
    let Some(promise) = error::try_js(
        ctx,
        js::Module::evaluate(ctx.clone(), entry_name.to_string(), entry_code),
        "module evaluation failed",
    ) else {
        return;
    };
    error::try_js(
        ctx,
        promise.into_future::<js::Value<'_>>().await,
        "promise rejected",
    );
    tokio::task::yield_now().await;
    tracing::info!("module evaluated");

    event_loop::run_event_loop(ctx, rt, js_timers).await;
}
