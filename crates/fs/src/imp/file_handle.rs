use std::marker::PhantomData;

use rquickjs::{ArrayBuffer, Ctx, Result, class::Class};
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

use crate::error::Error;
use js_core::error::SystemError;

#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
#[rquickjs::class]
pub struct FileHandle<'js> {
    #[qjs(skip_trace)]
    file: Option<tokio::fs::File>,
    #[qjs(skip_trace)]
    buf: Vec<u8>,
    #[qjs(skip_trace)]
    chunk_size: usize,
    #[qjs(skip_trace)]
    _marker: PhantomData<&'js ()>,
}

pub fn init<'js>(ctx: &Ctx<'js>) -> Result<()> {
    Class::<FileHandle>::define(&ctx.globals())
}

#[rquickjs::function]
pub async fn open<'js>(ctx: Ctx<'js>, path: String, chunk_size: usize) -> Result<FileHandle<'js>> {
    let file = tokio::fs::File::open(&path).await.map_err(|e| {
        Error::System(SystemError::from_io(e, "open", Some(path.clone()))).into_exception(&ctx)
    })?;
    let buf = vec![0u8; chunk_size];
    Ok(FileHandle {
        file: Some(file),
        buf,
        chunk_size,
        _marker: PhantomData,
    })
}

#[rquickjs::methods]
impl<'js> FileHandle<'js> {
    #[qjs()]
    async fn read(&mut self, ctx: Ctx<'js>) -> Result<Option<ArrayBuffer<'js>>> {
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| rquickjs::Error::new_from_js("string", "file is closed"))?;

        let n = file.read(&mut self.buf).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "read", None::<String>)).into_exception(&ctx)
        })?;

        if n == 0 {
            return Ok(None);
        }

        self.buf.truncate(n);
        let old_buf = std::mem::replace(&mut self.buf, vec![0u8; self.chunk_size]);
        let ab = ArrayBuffer::from_source(ctx.clone(), old_buf)?;

        Ok(Some(ab))
    }

    #[qjs()]
    async fn seek(&mut self, ctx: Ctx<'js>, offset: i64, whence: String) -> Result<u64> {
        let file = self
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

        file.seek(pos).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "seek", None::<String>)).into_exception(&ctx)
        })
    }

    #[qjs()]
    async fn close(&mut self) -> Result<()> {
        self.file = None;
        Ok(())
    }
}
