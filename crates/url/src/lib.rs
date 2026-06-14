pub mod error;
pub mod search_params;
pub mod url;

pub use url::JsUrl;

use rquickjs::{self as js, Class};

pub fn create_globals<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    Class::<search_params::UrlSearchParams>::define(&ctx.globals())?;
    Class::<url::JsUrl>::define(&ctx.globals())?;
    Ok(())
}
