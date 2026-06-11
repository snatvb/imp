use crate::prelude::*;
use std::marker::PhantomData;

use js::{Ctx, Function, Value, class::Class, function::This};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, BufWriter, SeekFrom};

use crate::error::Error;
use js_core::ByteBuffer;
use js_core::error::{JsError, SystemError};
use js_core::utils::{JsStringArg, StringArg};

#[derive(js::class::Trace, js::JsLifetime)]
#[js::class]
pub struct WriteHandle<'js> {
    #[qjs(skip_trace)]
    file: Option<BufWriter<tokio::fs::File>>,
    #[qjs(skip_trace)]
    _marker: PhantomData<&'js ()>,
}

pub fn init<'js>(ctx: &Ctx<'js>) -> js::Result<()> {
    Class::<WriteHandle>::define(&ctx.globals())?;

    if let Some(proto) = Class::<WriteHandle>::prototype(ctx)? {
        let symbol_obj: js::Object = ctx.globals().get("Symbol")?;
        let dispose: js::Symbol = symbol_obj.get("dispose")?;

        let dispose_fn = Function::new(
            ctx.clone(),
            |this: This<Class<'js, WriteHandle<'js>>>| -> js::Result<()> {
                let mut handle = this.0.borrow_mut();
                handle.file = None;
                Ok(())
            },
        )?;

        proto.set(dispose, dispose_fn)?;
    }

    Ok(())
}

#[js::function]
pub async fn open_write<'js>(
    ctx: Ctx<'js>,
    path: Value<'js>,
    chunk_size: usize,
) -> js::Result<WriteHandle<'js>> {
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

#[js::methods]
impl<'js> WriteHandle<'js> {
    #[qjs()]
    async fn write(&mut self, ctx: Ctx<'js>, data: Value<'js>) -> js::Result<usize> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

        if let Ok(bb) = Class::<ByteBuffer>::from_value(&data) {
            let bb = bb.borrow();
            let bytes = bb.as_slice();
            writer.write_all(bytes).await.map_err(|e| {
                Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
            })?;
            Ok(bytes.len())
        } else if let Ok(s) = data.get::<js::String<'js>>() {
            let s = s.to_string()?;
            let bytes = s.as_bytes();
            writer.write_all(bytes).await.map_err(|e| {
                Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
            })?;
            Ok(bytes.len())
        } else if let Ok(ab) = data.get::<js::ArrayBuffer<'js>>() {
            let bytes = ab
                .as_bytes()
                .ok_or_else(|| js::Error::new_from_js("ArrayBuffer", "failed to read bytes"))?;
            writer.write_all(bytes).await.map_err(|e| {
                Error::System(SystemError::from_io(e, "write", None::<String>)).into_exception(&ctx)
            })?;
            Ok(bytes.len())
        } else {
            Err(js::Error::new_from_js(
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
    ) -> js::Result<usize> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

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
    async fn flush(&mut self, ctx: Ctx<'js>) -> js::Result<()> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

        writer.flush().await.map_err(|e| {
            Error::System(SystemError::from_io(e, "flush", None::<String>)).into_exception(&ctx)
        })
    }

    #[qjs()]
    async fn seek(&mut self, ctx: Ctx<'js>, offset: i64, whence: String) -> js::Result<u64> {
        let writer = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

        let pos = match whence.as_str() {
            "start" => SeekFrom::Start(offset as u64),
            "current" => SeekFrom::Current(offset),
            "end" => SeekFrom::End(offset),
            _ => {
                return Err(js::Error::new_from_js(
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
    async fn close(&mut self, ctx: Ctx<'js>) -> js::Result<()> {
        if let Some(mut writer) = self.file.take() {
            writer.flush().await.map_err(|e| {
                Error::System(SystemError::from_io(e, "close", None::<String>)).into_exception(&ctx)
            })?;
        }
        Ok(())
    }
}
