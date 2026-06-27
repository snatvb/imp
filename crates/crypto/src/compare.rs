use js_core::js;
use subtle::ConstantTimeEq;

use crate::common::extract_bytes;

#[js::function]
pub fn timing_safe_equal<'js>(
    ctx: js::Ctx<'js>,
    a: js::Value<'js>,
    b: js::Value<'js>,
) -> js::Result<js::Value<'js>> {
    let a_bytes = extract_bytes(&ctx, &a, "a")?;
    let b_bytes = extract_bytes(&ctx, &b, "b")?;

    if a_bytes.len() != b_bytes.len() {
        return Ok(js::Value::new_bool(ctx.clone(), false));
    }

    let result = a_bytes.ct_eq(&b_bytes);
    Ok(js::Value::new_bool(ctx.clone(), result.into()))
}
