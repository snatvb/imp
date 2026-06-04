use std::env;

use clap::Parser;
use os_path::OsPathBuf;
use std::path::PathBuf;

mod error;
mod prelude;
mod tracing_init;
use js_core::{meta::with_meta, register_native_modules, typescript};
use prelude::*;

#[derive(Debug, Parser)]
#[command(name = "ImpJS", version = "0.1.0", long_about = None)]
struct Args {
    #[arg(help = "Path to the target file")]
    filepath: PathBuf,
    #[cfg(debug_assertions)]
    #[arg(short, long, help = "Enable tracing output (debug only)")]
    trace: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.trace_enabled() {
        tracing_init::init();
    }
    let _span = tracing::info_span!("imp", file = %args.filepath.display()).entered();

    tracing::info!("resolving entry");
    let resolver = js_core::resolver::Resolver::default();
    let Some(filepath) = resolver.resolve_entry(&args.filepath) else {
        panic!(
            "Can't find script by path {}",
            args.filepath.to_string_lossy()
        )
    };
    tracing::info!(file = %filepath, "entry resolved");

    tracing::info!("loading source");
    let code = tokio::fs::read_to_string(&filepath).await.unwrap();
    tracing::debug!(bytes = code.len(), "source loaded");

    let cwd = OsPathBuf::from_path_buf(env::current_dir().unwrap()).unwrap();
    let code = if typescript::is_ts_ext(&filepath) {
        tracing::info!("stripping TS");
        typescript::strip_types_fast_default(&code)
            .map(with_meta(&cwd, &filepath))
            .unwrap()
    } else {
        code
    };

    tracing::info!("creating runtime");
    let rt = js::AsyncRuntime::new().unwrap();
    let ctx = js::AsyncContext::full(&rt).await.unwrap();
    tracing::info!("runtime ready");

    let mut builtin_resolver = js::loader::BuiltinResolver::default();
    let mut module_loader = js::loader::ModuleLoader::default();
    register_native_modules!(
        module_loader,
        builtin_resolver,
        ("fs/promises", fs::FsPromisesModule),
        ("path", js_path::PathModule),
        ("imp:fs", fs::imp::ImpFsModule),
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

    ctx.async_with(async |ctx| {
        match js_core::rs_string::init_rs_string(&ctx) {
            Ok(()) => {}
            Err(e) => {
                tracing::error!("init_rs_string error: {}", e);
                panic!("init_rs_string failed: {}", e);
            }
        }
        match js_core::byte_buffer::init(&ctx) {
            Ok(()) => {}
            Err(e) => {
                tracing::error!("byte_buffer::init error: {}", e);
                panic!("byte_buffer::init failed: {}", e);
            }
        }
        let globals = ctx.globals();
        globals
            .set("console", console::create(&ctx).unwrap())
            .unwrap();
        globals
            .set("process", process::create(&ctx).unwrap())
            .unwrap();
        globals
            .set("performance", js_core::performance::create(&ctx).unwrap())
            .unwrap();
        tracing::info!(file = %filepath, "evaluating module");
        let promise = error::expect_js(
            &ctx,
            js::Module::evaluate(ctx.clone(), filepath.as_str(), code.as_str()),
            "module evaluation failed",
        );
        let _: js::Value<'_> =
            error::expect_js(&ctx, promise.into_future().await, "promise rejected");
        tracing::info!("module evaluated");
    })
    .await;

    rt.idle().await;
}

impl Args {
    fn trace_enabled(&self) -> bool {
        #[cfg(debug_assertions)]
        {
            self.trace
        }
        #[cfg(not(debug_assertions))]
        {
            false
        }
    }
}
