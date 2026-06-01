use clap::Parser;

mod prelude;
use js_core::typescript;
use prelude::*;

#[derive(Debug, Parser)]
#[command(name = "ImpJS", version = "0.1.0", long_about = None)]
struct Args {
    #[arg(help = "Path to the target file")]
    filepath: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let code = tokio::fs::read_to_string(&args.filepath).await.unwrap();
    let code = if typescript::is_ts_ext(&args.filepath) {
        typescript::strip_types_fast_default(&code).unwrap()
    } else {
        code
    };

    let rt = js::AsyncRuntime::new().unwrap();
    let ctx = js::AsyncContext::full(&rt).await.unwrap();

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
        let promise =
            js::Module::evaluate(ctx, args.filepath.to_string_lossy().as_ref(), code.as_str())
                .unwrap();
        let _: js::Value<'_> = promise.into_future().await.unwrap();
    })
    .await;

    rt.idle().await;
}
