#[cfg(debug_assertions)]
pub fn init() {
    use tracing_subscriber::{EnvFilter, fmt};
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("cli=debug,js_core=trace,fs=debug"));
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_writer(std::io::stderr)
        .init();
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn init() {}
