use std::env;

use clap::Parser;
use os_path::OsPathBuf;
use std::path::PathBuf;

mod error;
mod event_loop;
mod prelude;
mod setup;
mod tracing_init;
use js_core::{meta::with_meta, typescript};
use prelude::*;

#[derive(Debug, Parser)]
#[command(name = "ImpJS", version = "0.1.0", long_about = None)]
struct Args {
    #[arg(help = "Path to the target file")]
    filepath: PathBuf,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    script_args: Vec<String>,
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
    let filepath = std::fs::canonicalize(filepath).unwrap();
    let filepath_str = filepath.to_string_lossy();

    tracing::info!("loading source");
    let code = tokio::fs::read_to_string(&filepath).await.unwrap();
    tracing::debug!(bytes = code.len(), "source loaded");

    let cwd = OsPathBuf::from_path_buf(env::current_dir().unwrap()).unwrap();
    let code = if typescript::is_ts_ext(&filepath) {
        tracing::info!("stripping TS");
        typescript::strip_types_fast_default(&code)
            .map(with_meta(&cwd, &filepath_str))
            .unwrap()
    } else {
        code
    };

    tracing::info!("creating runtime");
    let rt = js::AsyncRuntime::new().unwrap();
    let ctx = js::AsyncContext::full(&rt).await.unwrap();
    tracing::info!("runtime ready");

    setup::setup_loaders(&rt, resolver, cwd).await;

    let exe_path = std::env::current_exe().unwrap();
    ctx.async_with(async |ctx| {
        let js_timers = setup::setup_globals(
            &ctx,
            exe_path.to_string_lossy().as_ref(),
            &filepath_str,
            args.script_args.as_slice(),
        );

        tracing::info!(file = %filepath_str, "evaluating module");
        let Some(promise) = error::try_js(
            &ctx,
            js::Module::evaluate(ctx.clone(), filepath_str.to_string(), code.as_str()),
            "module evaluation failed",
        ) else {
            return;
        };
        error::try_js(
            &ctx,
            promise.into_future::<js::Value<'_>>().await,
            "promise rejected",
        );
        tokio::task::yield_now().await;
        tracing::info!("module evaluated");

        event_loop::run_event_loop(&ctx, &rt, js_timers).await;
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
