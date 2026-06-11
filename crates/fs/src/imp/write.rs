use std::marker::PhantomData;

use rquickjs::{Ctx, Result, Value, class::Class};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, BufWriter, SeekFrom};

use crate::error::Error;
use js_core::ByteBuffer;
use js_core::error::{JsError, SystemError};
use js_core::utils::{JsStringArg, StringArg};

#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
#[rquickjs::class]
pub struct WriteHandle<'js> {
    #[qjs(skip_trace)]
    file: Option<BufWriter<tokio::fs::File>>,
    #[qjs(skip_trace)]
    _marker: PhantomData<&'js ()>,
}

pub fn init<'js>(ctx: &Ctx<'js>) -> Result<()> {
    Class::<WriteHandle>::define(&ctx.globals())
}

#[rquickjs::function]
pub async fn open_write<'js>(
    ctx: Ctx<'js>,
    path: Value<'js>,
    chunk_size: usize,
) -> Result<WriteHandle<'js>> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str().to_string();
    let file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path_str)
        .await
        .map_err(|e| {
            Error::System(SystemError::from_io(e, "open", Some(path_str.clone())))
                .into_exception(&ctx)
        })?;
    let writer = BufWriter::with_capacity(chunk_size.max(8192), file);
    Ok(WriteHandle {
        file: Some(writer),
        _marker: PhantomData,
    })
}

#[rquickjs::methods]
impl<'js> WriteHandle<'js> {
    #[qjs()]
    async fn write(&mut self, ctx: Ctx<'js>, data: Value<'js>) -> Result<usize> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| rquickjs::Error::new_from_js("string", "file is closed"))?;

        if let Ok(bb) = Class::<ByteBuffer>::from_value(&data) {
            let bb = bb.borrow();
            let bytes = bb.as_slice();
            writer.write_all(bytes).await.map_err(|e| {
                Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
            })?;
            Ok(bytes.len())
        } else if let Ok(s) = data.get::<rquickjs::String<'js>>() {
            let s = s.to_string()?;
            let bytes = s.as_bytes();
            writer.write_all(bytes).await.map_err(|e| {
                Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
            })?;
            Ok(bytes.len())
        } else if let Ok(ab) = data.get::<rquickjs::ArrayBuffer<'js>>() {
            let bytes = ab.as_bytes().ok_or_else(|| {
                rquickjs::Error::new_from_js("ArrayBuffer", "failed to read bytes")
            })?;
            writer.write_all(bytes).await.map_err(|e| {
                Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
            })?;
            Ok(bytes.len())
        } else {
            Err(rquickjs::Error::new_from_js(
                "ByteBuffer, string, or ArrayBuffer",
                "unsupported write data type",
            ))
        }
    }

    #[qjs(rename = "writeFrom")]
    async fn write_from(
        &mut self,
        ctx: Ctx<'js>,
        buffer: Class<'js, ByteBuffer>,
        offset: Option<usize>,
        length: Option<usize>,
    ) -> Result<usize> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| rquickjs::Error::new_from_js("string", "file is closed"))?;

        let bb = buffer.borrow();
        let slice = bb.as_slice();
        let offset = offset.unwrap_or(0).min(slice.len());
        let length = length
            .unwrap_or(slice.len() - offset)
            .min(slice.len() - offset);
        let data = &slice[offset..offset + length];

        writer.write_all(data).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
        })?;

        Ok(length)
    }

    #[qjs()]
    async fn flush(&mut self, ctx: Ctx<'js>) -> Result<()> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| rquickjs::Error::new_from_js("string", "file is closed"))?;

        writer.flush().await.map_err(|e| {
            Error::System(SystemError::from_io(e, "flush", None::<String>)).into_exception(&ctx)
        })
    }

    #[qjs()]
    async fn seek(&mut self, ctx: Ctx<'js>, offset: i64, whence: String) -> Result<u64> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| rquickjs::Error::new_from_js("string", "file is closed"))?;

        let pos = match whence.as_str() {
            "start" => SeekFrom::Start(offset as u64),
            "current" => SeekFrom::Current(offset),
            "end" => SeekFrom::End(offset),
            _ => {
                return Err(rquickjs::Error::new_from_js(
                    "string",
                    "whence must be start/current/end",
                ));
            }
        };

        writer.seek(pos).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "seek", None::<String>)).into_exception(&ctx)
        })
    }

    #[qjs()]
    async fn close(&mut self, ctx: Ctx<'js>) -> Result<()> {
        if let Some(mut writer) = self.file.take() {
            writer.flush().await.map_err(|e| {
                Error::System(SystemError::from_io(e, "close", None::<String>)).into_exception(&ctx)
            })?;
        }
        Ok(())
    }
}
