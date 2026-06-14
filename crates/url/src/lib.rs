pub mod error;
pub mod search_params;
pub mod url;

use rquickjs::{self as js, Class};

pub fn create_globals<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    Class::<search_params::UrlSearchParams>::define(&ctx.globals())?;
    Class::<url::UrlUrl>::define(&ctx.globals())?;
    Ok(())
}
