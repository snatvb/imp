use js_core::js;
use js_core::js::function::Opt;
use md5::Md5;

#[js::function]
pub fn md5<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    crate::common::hash_input::<Md5>(ctx, input, encoding)
}
