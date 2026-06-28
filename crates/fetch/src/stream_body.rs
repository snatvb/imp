use js_core::js;
use js_core::streams::{PendingIo, QueuingStrategy, create_preloaded_stream};

pub fn create_fetch_stream<'js>(
    ctx: &js::Ctx<'js>,
    body: Vec<u8>,
    pending_io: PendingIo,
) -> js::Result<js::Class<'js, js_core::streams::ReadableStream>> {
    let arr_buf = js::ArrayBuffer::from_source(ctx.clone(), body)?;
    let chunk = arr_buf.into_value();
    create_preloaded_stream(ctx, vec![chunk], QueuingStrategy::default(), pending_io)
}
