use core::ptr::NonNull;
use rquickjs::{
    self as js, ArrayBuffer, Class, Ctx, JsLifetime, Result, TypedArray,
    class::{Trace, Tracer},
    function::Opt,
};

use crate::rs_string::RsString;

#[js::class]
#[derive(JsLifetime)]
pub struct ByteBuffer<'js> {
    inner: ArrayBuffer<'js>,
    #[qjs(skip_trace)]
    data_ptr: NonNull<u8>,
    #[qjs(skip_trace)]
    data_len: usize,
}

impl<'js> Trace<'js> for ByteBuffer<'js> {
    fn trace<'a>(&self, tracer: Tracer<'a, 'js>) {
        self.inner.as_value().trace(tracer);
    }
}

impl<'js> ByteBuffer<'js> {
    fn from_arraybuffer(inner: ArrayBuffer<'js>) -> Result<Self> {
        let data = inner.as_bytes().expect("ArrayBuffer detached");
        let data_ptr = NonNull::new(data.as_ptr() as *mut u8).expect("null pointer");
        let data_len = data.len();
        Ok(ByteBuffer {
            inner,
            data_ptr,
            data_len,
        })
    }

    pub fn new(ctx: Ctx<'js>, data: Vec<u8>) -> Result<Self> {
        let inner = ArrayBuffer::from_source(ctx, data)?;
        Self::from_arraybuffer(inner)
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data_ptr.as_ptr(), self.data_len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data_ptr.as_ptr(), self.data_len) }
    }
}

#[js::methods]
impl<'js> ByteBuffer<'js> {
    #[qjs(constructor)]
    fn construct(ctx: Ctx<'js>, size: usize) -> Result<ByteBuffer<'js>> {
        let inner = ArrayBuffer::new(ctx, vec![0u8; size])?;
        Self::from_arraybuffer(inner)
    }

    #[qjs(get, rename = "length")]
    fn length(&self) -> usize {
        self.inner.len()
    }

    #[qjs(static)]
    fn alloc(ctx: Ctx<'js>, size: usize) -> Result<ByteBuffer<'js>> {
        let inner = ArrayBuffer::new(ctx, vec![0u8; size])?;
        Self::from_arraybuffer(inner)
    }

    #[qjs(static, rename = "fromArray")]
    fn from_array(ctx: Ctx<'js>, arr: js::Array<'js>) -> Result<ByteBuffer<'js>> {
        let mut bytes = Vec::with_capacity(arr.len());
        for i in 0..arr.len() {
            bytes.push(arr.get::<u8>(i)?);
        }
        let inner = ArrayBuffer::from_source(ctx, bytes)?;
        Self::from_arraybuffer(inner)
    }

    #[qjs(rename = "toString")]
    fn to_string_js(&self, ctx: Ctx<'js>) -> Result<js::String<'js>> {
        let data = self.as_slice();
        let s = String::from_utf8_lossy(data);
        js::String::from_str(ctx, &s)
    }

    #[qjs(rename = "toStr")]
    fn to_str_rs(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        let data = self.as_slice();
        let s = String::from_utf8_lossy(data).into_owned();
        Class::instance(ctx, RsString::owned(s))
    }

    #[qjs(rename = "toArrayBuffer")]
    fn to_array_buffer_method(&self) -> ArrayBuffer<'js> {
        self.inner.clone()
    }

    fn slice(&self, ctx: Ctx<'js>, start: usize, end: Opt<usize>) -> Result<ByteBuffer<'js>> {
        let len = self.inner.len();
        let end = end.0.unwrap_or(len).min(len);
        let start = start.min(end);
        let data = self.as_slice();
        let sliced = data[start..end].to_vec();
        ByteBuffer::new(ctx, sliced)
    }

    #[qjs(rename = "toArray")]
    fn to_array(&self) -> Result<TypedArray<'js, u8>> {
        TypedArray::from_arraybuffer(self.inner.clone())
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
