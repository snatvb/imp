use js_core::js;
use js_core::js::function::Opt;

#[js::function]
pub fn blake3<'js>(
    ctx: js::Ctx<'js>,
    input: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    crate::common::blake3_input(ctx, input, encoding)
}
