use std::io::{self, BufRead, Read};

use js_core::byte_buffer::ByteBuffer;
use js_core::error::SystemError;
use js_core::rs_string::RsString;

use crate::prelude::*;

fn read_line_blocking() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    handle.read_line(&mut line)?;
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    Ok(line)
}

fn read_all_blocking() -> io::Result<Vec<u8>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    handle.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[js::function]
pub async fn read_line<'js>(ctx: js::Ctx<'js>) -> js::Result<js::Class<'js, RsString>> {
    let join_result = tokio::task::spawn_blocking(read_line_blocking).await;
    let io_result = join_result.into_js(&ctx)?;
    let line = io_result
        .map_err(|e| Error::System(SystemError::from_io(e, "read", None)))
        .into_js(&ctx)?;
    js::Class::instance(ctx, RsString::owned(line))
}

#[js::function]
pub async fn read_all<'js>(ctx: js::Ctx<'js>) -> js::Result<js::Class<'js, ByteBuffer<'js>>> {
    let join_result = tokio::task::spawn_blocking(read_all_blocking).await;
    let io_result = join_result.into_js(&ctx)?;
    let data = io_result
        .map_err(|e| Error::System(SystemError::from_io(e, "read", None)))
        .into_js(&ctx)?;
    js::Class::instance(ctx.clone(), ByteBuffer::new(ctx, data)?)
}

js_core::impl_module!(StdinModule,
    "readLine" => js_read_line,
    "readAll" => js_read_all,
);
