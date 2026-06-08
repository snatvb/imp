use os_path::OsPathBuf;

use crate::prelude::*;
use js_core::{register_native_modules, resolver::Resolver, timers::JsTimers};

pub async fn setup_loaders(rt: &js::AsyncRuntime, resolver: Resolver, cwd: OsPathBuf) {
    let mut builtin_resolver = js::loader::BuiltinResolver::default();
    let mut module_loader = js::loader::ModuleLoader::default();
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

pub fn setup_globals<'js>(ctx: &js::Ctx<'js>, script_args: Vec<String>) -> JsTimers {
    js_core::rs_string::init_rs_string_or_panic(ctx);
    js_core::byte_buffer::init_or_panic(ctx);
    let globals = ctx.globals();
    let js_timers = JsTimers::new();
    js_timers.bind_to(ctx, &globals).unwrap();
    globals
        .set("console", console::create(ctx).unwrap())
        .unwrap();
    globals
        .set("process", process::create(ctx).unwrap())
        .unwrap();
    globals
        .set("performance", js_core::performance::create(ctx).unwrap())
        .unwrap();
    imp_clap::set_script_args(ctx, script_args).unwrap();
    js_timers
}
