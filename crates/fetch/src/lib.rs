pub mod client;
pub mod error;
pub mod fetch;
pub mod file_fetch;
pub mod headers;
pub mod request;
pub mod response;
pub mod stream_body;
pub mod url;

use rquickjs::{self as js, Class};

pub fn create_globals<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    Class::<headers::Headers>::define(&ctx.globals())?;
    Class::<js_core::abort::AbortSignal>::define(&ctx.globals())?;
    Class::<js_core::abort::AbortController>::define(&ctx.globals())?;
    Class::<request::Request>::define(&ctx.globals())?;
    Class::<response::Response>::define(&ctx.globals())?;
    js_core::streams::init(ctx)?;
    js_core::streams::init_dispose(ctx)?;
    ctx.globals()
        .set("fetch", js::Function::new(ctx.clone(), fetch::js_fetch)?)?;
    Ok(())
}
