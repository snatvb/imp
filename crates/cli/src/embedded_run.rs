use std::env;

use embed::Bundle;

use crate::prelude::*;
use crate::setup;

pub async fn run_embedded(bundle: Bundle) {
    let script_args: Vec<String> = env::args().skip(1).collect();

    let rt = js::AsyncRuntime::new().unwrap();
    let ctx = js::AsyncContext::full(&rt).await.unwrap();

    let (entry_name, entry_code) = setup::setup_embedded_loaders(&rt, bundle).await;

    let exit_code = ctx
        .async_with(async |ctx| {
            setup::run_js_entry(&ctx, &entry_name, &entry_code, &script_args).await
        })
        .await;

    rt.idle().await;

    std::process::exit(exit_code);
}
