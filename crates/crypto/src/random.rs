use js_core::error::JsError;
use js_core::js;
use rand::RngExt;

use crate::error::CryptoError;

#[js::function]
pub fn random_bytes<'js>(ctx: js::Ctx<'js>, size: u32) -> js::Result<js::Value<'js>> {
    let n = size as usize;
    let mut buf = vec![0u8; n];
    rand::rng().fill(&mut buf[..]);
    Ok(js::Class::instance(
        ctx.clone(),
        js_core::byte_buffer::ByteBuffer::new(ctx.clone(), buf)?,
    )?
    .into_value())
}

#[js::function]
pub fn random_hex<'js>(ctx: js::Ctx<'js>, size: u32) -> js::Result<js::Value<'js>> {
    let n = size as usize;
    let mut buf = vec![0u8; n];
    rand::rng().fill(&mut buf[..]);
    Ok(js::String::from_str(ctx.clone(), &hex::encode(buf))?.into_value())
}

#[js::function]
pub fn random_uuid<'js>(ctx: js::Ctx<'js>) -> js::Result<js::Value<'js>> {
    let id = uuid::Uuid::new_v4().to_string();
    Ok(js::String::from_str(ctx.clone(), &id)?.into_value())
}

#[js::function]
pub fn random_int<'js>(ctx: js::Ctx<'js>, min: i64, max: i64) -> js::Result<js::Value<'js>> {
    if min >= max {
        return Err(
            CryptoError::range_error("randomInt: max must be greater than min")
                .into_exception(&ctx),
        );
    }
    let val: i64 = rand::rng().random_range(min..max);
    use rquickjs::IntoJs;
    (val as i32).into_js(&ctx)
}
