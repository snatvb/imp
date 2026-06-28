use std::{cell::RefCell, rc::Rc};

use crate::js;

mod pipe;
mod readable;
mod strategies;

pub use pipe::*;
pub use readable::*;
pub use strategies::*;

#[derive(Debug, Default, Clone)]
pub struct PendingIo(Rc<RefCell<usize>>);

impl PendingIo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(&self) {
        let mut count = self.0.borrow_mut();
        *count += 1;
    }

    pub fn decrement(&self) {
        let mut count = self.0.borrow_mut();
        if *count > 0 {
            *count -= 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        *self.0.borrow() == 0
    }
}

pub fn init<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    js::Class::<ReadableStream>::define(&ctx.globals())?;
    js::Class::<ReadableStreamDefaultController>::define(&ctx.globals())?;
    js::Class::<ReadableStreamDefaultReader>::define(&ctx.globals())?;
    js::Class::<ReadableStreamAsyncIterator>::define(&ctx.globals())?;
    let _ = init_async_iterator(ctx);
    Ok(())
}

pub fn init_or_panic<'js>(ctx: &js::Ctx<'js>) {
    if let Err(e) = init(ctx) {
        tracing::error!("streams::init error: {}", e);
        panic!("streams::init failed: {}", e);
    }
}

pub fn init_dispose<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    readable::setup_dispose(ctx)
}
