use clap::Parser;

mod prelude;
mod tracing_init;
use js_core::typescript;
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

    let code = if typescript::is_ts_ext(&filepath) {
        tracing::info!("stripping TS");
        typescript::strip_types_fast_default(&code).unwrap()
    } else {
        code
    };

    tracing::info!("creating runtime");
    let rt = js::AsyncRuntime::new().unwrap();
    let ctx = js::AsyncContext::full(&rt).await.unwrap();
    tracing::info!("runtime ready");

    let mut module_loader = js::loader::ModuleLoader::default();
    module_loader.add_module("fs/promises", fs::FsPromisesModule);

    rt.set_loader(
        (
            js_core::resolver::Resolver::default(),
            js::loader::BuiltinResolver::default(),
        ),
        (
            js_core::loader::TsLoader,
            js::loader::ScriptLoader::default(),
            module_loader,
        ),
    )
    .await;

    ctx.async_with(async |ctx| {
        let globals = ctx.globals();
        globals
            .set("console", console::create(&ctx).unwrap())
            .unwrap();
        tracing::info!(file = %filepath, "evaluating module");
        let promise = js::Module::evaluate(ctx, filepath.as_str(), code.as_str()).unwrap();
        let _: js::Value<'_> = promise.into_future().await.unwrap();
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
