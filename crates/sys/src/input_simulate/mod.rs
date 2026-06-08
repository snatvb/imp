#[cfg(target_os = "windows")]
mod key;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
use win as platform;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix as platform;

use crate::prelude::*;

#[inline(always)]
async fn spawn_blocking<'js, R: Send + 'static>(
    ctx: &js::Ctx<'js>,
    f: impl FnOnce() -> R + Send + 'static,
) -> js::Result<R> {
    tokio::task::spawn_blocking(f).await.into_js(ctx)
}

#[js::function]
pub async fn inject_keys<'js>(
    ctx: js::Ctx<'js>,
    keys: js::Array<'js>,
) -> js::Result<()> {
    let iter = StringArg::coerce_array_iter(&ctx, &keys, "keys");
    let key_strs: Vec<String> = iter
        .map(|arg| arg.map(|a| a.as_str().to_string()))
        .collect::<js::Result<_>>()?;

    spawn_blocking(&ctx, move || {
        let key_refs: Vec<&str> = key_strs.iter().map(|s| s.as_str()).collect();
        platform::inject_keys(&key_refs)
    })
    .await?
    .into_js(&ctx)?;

    Ok(())
}

js_core::impl_module!(InputSimulateModule,
    evaluate: |ctx, exports, all| {
        #[cfg(unix)]
        {
            platform::init().into_js(ctx)?;
        }
        let _ = export_all(ctx, exports)?;
        Ok(())
    },
    "injectKeys" => js_inject_keys,
);
