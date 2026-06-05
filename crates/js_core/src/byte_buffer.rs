use rquickjs::{
    self as js, ArrayBuffer, Class, Ctx, JsLifetime, Result,
    class::{Trace, Tracer},
    function::Opt,
};

use crate::rs_string::RsString;

#[js::class]
#[derive(JsLifetime)]
pub struct ByteBuffer {
    inner: Vec<u8>,
}

impl<'js> Trace<'js> for ByteBuffer {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl ByteBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        ByteBuffer { inner: data }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

#[js::methods]
impl ByteBuffer {
    #[qjs(constructor)]
    fn construct(size: usize) -> ByteBuffer {
        ByteBuffer {
            inner: vec![0u8; size],
        }
    }

    #[qjs(get, rename = "length")]
    fn length(&self) -> usize {
        self.inner.len()
    }

    #[qjs(static)]
    fn alloc(size: usize) -> ByteBuffer {
        ByteBuffer {
            inner: vec![0u8; size],
        }
    }

    #[qjs(rename = "toString")]
    fn to_string_js<'js>(&self, ctx: Ctx<'js>) -> Result<js::String<'js>> {
        let s = String::from_utf8_lossy(&self.inner);
        js::String::from_str(ctx, &s)
    }

    #[qjs(rename = "toStr")]
    fn to_str_rs<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        let s = String::from_utf8_lossy(&self.inner).into_owned();
        Class::instance(ctx, RsString::owned(s))
    }

    #[qjs(rename = "toArrayBuffer")]
    fn to_array_buffer_method<'js>(&self, ctx: Ctx<'js>) -> Result<ArrayBuffer<'js>> {
        ArrayBuffer::from_source(ctx, self.inner.clone())
    }

    fn slice(&self, start: usize, end: Opt<usize>) -> ByteBuffer {
        let end = end.0.unwrap_or(self.inner.len()).min(self.inner.len());
        let start = start.min(end);
        let sliced = self.inner[start..end].to_vec();
        ByteBuffer::new(sliced)
    }

    #[qjs(rename = "toArray")]
    fn to_array<'js>(&self, ctx: Ctx<'js>) -> Result<js::Array<'js>> {
        let arr = js::Array::new(ctx.clone())?;
        for (i, &byte) in self.inner.iter().enumerate() {
            arr.set(i, byte)?;
        }
        Ok(arr)
    }
}

pub fn init<'js>(ctx: &Ctx<'js>) -> Result<()> {
    Class::<ByteBuffer>::define(&ctx.globals())
}

pub fn init_or_panic<'js>(ctx: &Ctx<'js>) {
    if let Err(e) = init(ctx) {
        tracing::error!("byte_buffer::init error: {}", e);
        panic!("byte_buffer::init failed: {}", e);
    }
}
