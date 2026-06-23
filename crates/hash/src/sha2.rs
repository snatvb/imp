use js_core::js;
use js_core::js::function::Opt;
use sha2::{Sha256, Sha512};

#[js::function]
pub fn sha256<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    crate::common::hash_input::<Sha256>(ctx, input, encoding)
}

#[js::function]
pub fn sha512<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    crate::common::hash_input::<Sha512>(ctx, input, encoding)
}
