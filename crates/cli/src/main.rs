use std::env;

use clap::{Parser, Subcommand};
use os_path::OsPathBuf;
use std::path::{Path, PathBuf};

mod embedded_run;
mod error;
mod event_loop;
mod prelude;
mod setup;
mod tracing_init;
use js_core::{meta::with_meta, typescript};
use prelude::*;

#[derive(Debug, Parser)]
#[command(name = "ImpJS", version = "0.1.0", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(help = "Path to the target file")]
    filepath: Option<PathBuf>,

    #[arg(last = true, hide = true)]
    script_args: Vec<String>,

    #[cfg(debug_assertions)]
    #[arg(short, long, help = "Enable tracing output (debug only)")]
    trace: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run a script")]
    Run {
        #[arg(help = "Path to the target file")]
        filepath: PathBuf,
        #[arg(last = true, hide = true)]
        script_args: Vec<String>,
        #[cfg(debug_assertions)]
        #[arg(short, long, help = "Enable tracing output (debug only)")]
        trace: bool,
    },
    #[command(about = "Compile a script into a standalone executable")]
    Compile {
        #[arg(help = "Path to the entry script")]
        filepath: PathBuf,
        #[arg(help = "Output binary name")]
        output: String,
    },
}

#[tokio::main]
async fn main() {
    if let Some(bundle) = embed::read_embedded() {
        embedded_run::run_embedded(bundle).await;
        return;
    }

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Compile { filepath, output }) => {
            compile(&filepath, &output);
        }
        Some(Commands::Run {
            filepath,
            script_args,
            #[cfg(debug_assertions)]
            trace,
        }) => {
            #[cfg(debug_assertions)]
            if trace {
                tracing_init::init();
            }
            run_script(filepath, script_args).await;
        }
        None => {
            #[cfg(debug_assertions)]
            if cli.trace_enabled() {
                tracing_init::init();
            }
            let filepath = cli.filepath.expect("filepath is required");
            run_script(filepath, cli.script_args).await;
        }
    }
}

fn compile(filepath: &Path, output: &str) {
    let resolver = js_core::resolver::Resolver::default();
    let Some(entry) = resolver.resolve_entry(filepath) else {
        panic!("Can't find script by path {}", filepath.display());
    };

    let native_names = setup::native_module_names();
    let bundle = js_core::bundler::bundle(std::path::Path::new(&entry), &resolver, native_names)
        .unwrap_or_else(|e| panic!("Bundle failed: {}", e));

    let exe = std::env::current_exe().unwrap();
    let mut output_path = PathBuf::from(output);
    if cfg!(windows) && output_path.extension().is_none() {
        output_path.set_extension("exe");
    }

    embed::write_embedded(&exe, &output_path, &bundle)
        .unwrap_or_else(|e| panic!("Write embedded failed: {}", e));

    println!("Compiled: {}", output_path.display());
}

async fn run_script(filepath: PathBuf, script_args: Vec<String>) {
    let _span = tracing::info_span!("imp", file = %filepath.display()).entered();

    tracing::info!("resolving entry");
    let resolver = js_core::resolver::Resolver::default();
    let Some(filepath) = resolver.resolve_entry(&filepath) else {
        panic!("Can't find script by path {}", filepath.display());
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

    ctx.async_with(async |ctx| {
        setup::run_js_entry(&ctx, &rt, &filepath_str, &code, &script_args).await;
    })
    .await;

    rt.idle().await;
}

#[cfg(debug_assertions)]
impl Cli {
    fn trace_enabled(&self) -> bool {
        self.trace
    }
}
