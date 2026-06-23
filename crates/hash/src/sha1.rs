use js_core::js;
use js_core::js::function::Opt;
use sha1::Sha1;

#[js::function]
pub fn sha1<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    crate::common::hash_input::<Sha1>(ctx, input, encoding)
}
