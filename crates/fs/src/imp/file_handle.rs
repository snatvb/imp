use crate::prelude::*;
use std::marker::PhantomData;

use js::{Ctx, Function, Value, class::Class, function::This};
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom};

use crate::error::Error;
use js_core::ByteBuffer;
use js_core::error::{JsError, SystemError};
use js_core::utils::{JsStringArg, StringArg};

#[derive(js::class::Trace, js::JsLifetime)]
#[js::class]
pub struct FileHandle<'js> {
    #[qjs(skip_trace)]
    file: Option<BufReader<tokio::fs::File>>,
    #[qjs(skip_trace)]
    buf: Vec<u8>,
    #[qjs(skip_trace)]
    _marker: PhantomData<&'js ()>,
}

pub fn init<'js>(ctx: &Ctx<'js>) -> js::Result<()> {
    Class::<FileHandle>::define(&ctx.globals())?;

    if let Some(proto) = Class::<FileHandle>::prototype(ctx)? {
        let symbol_obj: js::Object = ctx.globals().get("Symbol")?;
        let dispose: js::Symbol = symbol_obj.get("dispose")?;

        let dispose_fn = Function::new(
            ctx.clone(),
            |this: This<Class<'js, FileHandle<'js>>>| -> js::Result<()> {
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
pub async fn open<'js>(
    ctx: Ctx<'js>,
    path: Value<'js>,
    chunk_size: usize,
) -> js::Result<FileHandle<'js>> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str().to_string();
    let file = tokio::fs::File::open(&path_str).await.map_err(|e| {
        Error::System(SystemError::from_io(e, "open", Some(path_str.clone()))).into_exception(&ctx)
    })?;
    let reader = BufReader::with_capacity(chunk_size.max(8192), file);
    let buf = vec![0u8; chunk_size];
    Ok(FileHandle {
        file: Some(reader),
        buf,
        _marker: PhantomData,
    })
}

#[js::methods]
impl<'js> FileHandle<'js> {
    #[qjs()]
    async fn read(&mut self, ctx: Ctx<'js>) -> js::Result<Value<'js>> {
        let reader = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

        let n = reader.read(&mut self.buf).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "read", None::<String>)).into_exception(&ctx)
        })?;

        if n == 0 {
            return Ok(Value::new_undefined(ctx.clone()));
        }

        let bb = ByteBuffer::new(self.buf[..n].to_vec());
        Class::instance(ctx, bb).map(|v| v.into_value())
    }

    #[qjs(rename = "readInto")]
    async fn read_into(
        &mut self,
        ctx: Ctx<'js>,
        buffer: Class<'js, ByteBuffer>,
    ) -> js::Result<Value<'js>> {
        let reader = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

        let mut bb = buffer.borrow_mut();
        let slice = bb.as_mut_slice();

        let n = reader.read(slice).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "read", None::<String>)).into_exception(&ctx)
        })?;

        if n == 0 {
            return Ok(Value::new_undefined(ctx.clone()));
        }

        Ok(Value::new_int(ctx, n as i32))
    }

    #[qjs()]
    async fn seek(&mut self, ctx: Ctx<'js>, offset: i64, whence: Value<'js>) -> js::Result<u64> {
        let whence_arg = StringArg::coerce_js(&ctx, &whence, "whence")?;
        let whence_str = whence_arg.as_str();

        let reader = self
            .file
            .as_mut()
            .ok_or_else(|| js::Error::new_from_js("string", "file is closed"))?;

        let pos = match whence_str {
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

        reader.seek(pos).await.map_err(|e| {
            Error::System(SystemError::from_io(e, "seek", None::<String>)).into_exception(&ctx)
        })
    }

    #[qjs()]
    async fn close(&mut self) -> js::Result<()> {
        self.file = None;
        Ok(())
    }
}
