use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use rquickjs::JsLifetime;

use crate::{
    coerce::throw_type_error,
    js,
    streams::{PendingIo, PipeOptions, QueuingStrategy},
};
use js::class::{Trace, Tracer};
use js::function::{Function, This};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReadableStreamState {
    Readable,
    Closed,
    Errored,
}

#[derive(Debug, Clone)]
struct ReadRequest {
    resolve: js::Persistent<js::Function<'static>>,
    reject: js::Persistent<js::Function<'static>>,
}

#[derive(Debug)]
struct ReadableStreamInner {
    state: ReadableStreamState,
    queue: VecDeque<js::Persistent<js::Value<'static>>>,
    pending_reads: VecDeque<ReadRequest>,
    source: Option<js::Persistent<js::Object<'static>>>,
    pulling: bool,
    locked: bool,
    pending_io: PendingIo,
    high_water_mark: usize,
    queue_size: usize,
    strategy: QueuingStrategy,
    backpressure: bool,
}

impl ReadableStreamInner {
    fn desired_size(&self) -> Option<i32> {
        match self.state {
            ReadableStreamState::Closed | ReadableStreamState::Errored => None,
            ReadableStreamState::Readable => {
                Some(self.high_water_mark as i32 - self.queue_size as i32)
            }
        }
    }

    fn update_backpressure(&mut self) {
        self.backpressure = self.desired_size().is_some_and(|size| size < 0);
    }

    fn enqueue_chunk<'js>(&mut self, ctx: &js::Ctx<'js>, chunk: js::Value<'js>) -> js::Result<()> {
        if let Some(req) = self.pending_reads.pop_front() {
            let resolve = req.resolve.restore(ctx)?;
            let result = js::Object::new(ctx.clone())?;
            result.set("value", chunk)?;
            result.set("done", false)?;
            resolve.call::<_, ()>((result,))?;
        } else {
            let persistent = js::Persistent::save(ctx, chunk);
            self.queue.push_back(persistent);
        }
        Ok(())
    }

    fn close<'js>(&mut self, ctx: &js::Ctx<'js>) -> js::Result<()> {
        self.state = ReadableStreamState::Closed;
        while let Some(req) = self.pending_reads.pop_front() {
            let resolve = req.resolve.restore(ctx)?;
            let result = js::Object::new(ctx.clone())?;
            result.set("value", js::Value::new_undefined(ctx.clone()))?;
            result.set("done", true)?;
            resolve.call::<_, ()>((result,))?;
        }
        Ok(())
    }

    fn error<'js>(&mut self, ctx: &js::Ctx<'js>, err: js::Value<'js>) -> js::Result<()> {
        self.state = ReadableStreamState::Errored;
        while let Some(req) = self.pending_reads.pop_front() {
            let reject = req.reject.restore(ctx)?;
            reject.call::<_, ()>((err.clone(),))?;
        }
        Ok(())
    }
}

#[js::class]
#[derive(JsLifetime)]
pub struct ReadableStream {
    #[qjs(skip_trace)]
    inner: Rc<RefCell<ReadableStreamInner>>,
}

impl<'js> Trace<'js> for ReadableStream {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::class]
#[derive(JsLifetime)]
pub struct ReadableStreamDefaultController {
    #[qjs(skip_trace)]
    inner: Rc<RefCell<ReadableStreamInner>>,
}

impl<'js> Trace<'js> for ReadableStreamDefaultController {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct ReadableStreamDefaultReader {
    #[qjs(skip_trace)]
    inner: Rc<RefCell<ReadableStreamInner>>,
}

impl<'js> Trace<'js> for ReadableStreamDefaultReader {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::class]
#[derive(JsLifetime)]
pub struct ReadableStreamAsyncIterator {
    #[qjs(skip_trace)]
    reader: Rc<RefCell<Option<ReadableStreamDefaultReader>>>,
}

impl<'js> Trace<'js> for ReadableStreamAsyncIterator {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::methods]
impl ReadableStream {
    #[qjs(constructor)]
    fn construct<'js>(
        ctx: js::Ctx<'js>,
        underlying_source: js::function::Opt<js::Object<'js>>,
        strategy: js::function::Opt<js::Object<'js>>,
    ) -> js::Result<js::Class<'js, ReadableStream>> {
        let queuing_strategy = if let Some(strategy_obj) = strategy.0 {
            QueuingStrategy::from_js_object(&strategy_obj)?
        } else {
            QueuingStrategy::default()
        };

        let source = underlying_source
            .0
            .unwrap_or_else(|| js::Object::new(ctx.clone()).unwrap());

        let pending_io = PendingIo::new();
        let stream = create_readable_stream(&ctx, source.clone(), queuing_strategy, pending_io)?;

        let inner = stream.borrow();
        let controller = ReadableStreamDefaultController {
            inner: inner.inner.clone(),
        };
        let controller_obj = js::Class::instance(ctx.clone(), controller)?;
        drop(inner);

        if let Ok(start_fn) = source.get::<_, js::Function<'js>>("start") {
            let _result: js::Value<'js> = start_fn.call((controller_obj,))?;
        }

        Ok(stream)
    }

    #[qjs(get)]
    fn locked(&self) -> bool {
        self.inner.borrow().locked
    }

    #[qjs(rename = "getReader")]
    fn get_reader<'js>(
        &self,
        ctx: js::Ctx<'js>,
    ) -> js::Result<js::Class<'js, ReadableStreamDefaultReader>> {
        let mut inner = self.inner.borrow_mut();
        if inner.locked {
            return Err(throw_type_error(
                &ctx,
                "ERR_STREAM_LOCKED",
                "ReadableStream is locked",
            ));
        }
        inner.locked = true;
        drop(inner);

        let reader = ReadableStreamDefaultReader {
            inner: self.inner.clone(),
        };
        js::Class::instance(ctx, reader)
    }

    async fn cancel<'js>(
        &self,
        ctx: js::Ctx<'js>,
        reason: js::function::Opt<js::Value<'js>>,
    ) -> js::Result<()> {
        let (source, pending_reads, was_pulling) = {
            let mut inner = self.inner.borrow_mut();
            if inner.state != ReadableStreamState::Readable {
                return Ok(());
            }
            inner.state = ReadableStreamState::Closed;

            let source = inner
                .source
                .as_ref()
                .ok_or_else(|| {
                    js::Error::new_loading_message("ReadableStream", "source not found")
                })?
                .clone()
                .restore(&ctx)?;

            let pending_reads = std::mem::take(&mut inner.pending_reads);
            let was_pulling = inner.pulling;
            if was_pulling {
                inner.pending_io.decrement();
                inner.pulling = false;
            }
            (source, pending_reads, was_pulling)
        };

        if let Ok(cancel_fn) = source.get::<_, js::Function<'js>>("cancel") {
            let reason_val = reason
                .0
                .unwrap_or_else(|| js::Value::new_undefined(ctx.clone()));
            let result: js::Value<'js> = cancel_fn.call((reason_val,))?;
            if let Ok(promise) = js::Promise::from_value(result) {
                let _: js::Value<'js> = promise.into_future().await?;
            }
        }

        for req in pending_reads {
            let resolve = req.resolve.restore(&ctx)?;
            let result = js::Object::new(ctx.clone())?;
            result.set("value", js::Value::new_undefined(ctx.clone()))?;
            result.set("done", true)?;
            resolve.call::<_, ()>((result,))?;
        }

        let _ = was_pulling;
        Ok(())
    }

    #[qjs(rename = "pipeTo")]
    async fn pipe_to<'js>(
        &self,
        ctx: js::Ctx<'js>,
        destination: js::Object<'js>,
        options: js::function::Opt<js::Object<'js>>,
    ) -> js::Result<()> {
        let pipe_options = if let Some(opts) = options.0 {
            PipeOptions::from_js_object(&opts)?
        } else {
            PipeOptions::default()
        };

        let dest_locked: bool = destination.get("locked").unwrap_or(false);
        if dest_locked {
            return Err(throw_type_error(
                &ctx,
                "ERR_STREAM_LOCKED",
                "destination stream is locked",
            ));
        }

        let get_writer_fn: js::Function<'js> = destination.get("getWriter")?;
        let writer: js::Object<'js> = get_writer_fn.call(())?;

        let reader = self.get_reader(ctx.clone())?;

        loop {
            let result = reader.borrow().read(ctx.clone()).await?;
            let done: bool = result.get("done")?;
            if done {
                if !pipe_options.prevent_close {
                    let close_fn: js::Function<'js> = writer.get("close")?;
                    let res: js::Value<'js> = close_fn.call(())?;
                    if let Ok(promise) = js::Promise::from_value(res) {
                        let _: js::Value<'js> = promise.into_future().await?;
                    }
                }
                break;
            }
            let value: js::Value<'js> = result.get("value")?;
            let write_fn: js::Function<'js> = writer.get("write")?;
            let res: js::Value<'js> = write_fn.call((value,))?;
            if let Ok(promise) = js::Promise::from_value(res) {
                let _: js::Result<js::Value<'js>> = promise.into_future().await;
            }
        }
        Ok(())
    }

    #[qjs(rename = "tee")]
    async fn tee<'js>(&self, ctx: js::Ctx<'js>) -> js::Result<js::Array<'js>> {
        let reader = self.get_reader(ctx.clone())?;

        let mut chunks = Vec::new();
        loop {
            let result = reader.borrow().read(ctx.clone()).await?;
            let done: bool = result.get("done")?;
            if done {
                break;
            }
            let value: js::Value<'js> = result.get("value")?;
            chunks.push(value);
        }

        let strategy = QueuingStrategy::default();
        let branch1 =
            create_preloaded_stream(&ctx, chunks.clone(), strategy.clone(), PendingIo::new())?;
        let branch2 = create_preloaded_stream(&ctx, chunks, strategy, PendingIo::new())?;

        let arr = js::Array::new(ctx.clone())?;
        arr.set(0, branch1)?;
        arr.set(1, branch2)?;
        Ok(arr)
    }

    fn values<'js>(
        &self,
        ctx: js::Ctx<'js>,
    ) -> js::Result<js::Class<'js, ReadableStreamAsyncIterator>> {
        let mut inner = self.inner.borrow_mut();
        if inner.locked {
            return Err(throw_type_error(
                &ctx,
                "ERR_STREAM_LOCKED",
                "ReadableStream is locked",
            ));
        }
        inner.locked = true;
        drop(inner);

        let reader = ReadableStreamDefaultReader {
            inner: self.inner.clone(),
        };

        let iterator = ReadableStreamAsyncIterator {
            reader: Rc::new(RefCell::new(Some(reader))),
        };

        js::Class::instance(ctx, iterator)
    }

    #[qjs(rename = "Symbol.asyncIterator")]
    fn async_iterator<'js>(
        &self,
        ctx: js::Ctx<'js>,
    ) -> js::Result<js::Class<'js, ReadableStreamAsyncIterator>> {
        self.values(ctx)
    }
}

#[js::methods]
impl ReadableStreamDefaultController {
    #[qjs(get, rename = "desiredSize")]
    fn desired_size<'js>(&self, ctx: js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        let inner = self.inner.borrow();
        match inner.desired_size() {
            Some(size) => Ok(js::Value::new_int(ctx, size)),
            None => Ok(js::Value::new_null(ctx)),
        }
    }

    fn enqueue<'js>(&self, ctx: js::Ctx<'js>, chunk: js::Value<'js>) -> js::Result<()> {
        let mut inner = self.inner.borrow_mut();
        let size = inner.strategy.chunk_size(&chunk);
        inner.queue_size += size;
        inner.update_backpressure();
        inner.enqueue_chunk(&ctx, chunk)
    }

    fn close<'js>(&self, ctx: js::Ctx<'js>) -> js::Result<()> {
        let mut inner = self.inner.borrow_mut();
        inner.close(&ctx)
    }

    fn error<'js>(&self, ctx: js::Ctx<'js>, err: js::Value<'js>) -> js::Result<()> {
        let mut inner = self.inner.borrow_mut();
        inner.error(&ctx, err)
    }
}

#[js::methods]
impl ReadableStreamDefaultReader {
    async fn read<'js>(&self, ctx: js::Ctx<'js>) -> js::Result<js::Object<'js>> {
        let queue_chunk = {
            let mut inner = self.inner.borrow_mut();
            if let Some(persistent) = inner.queue.pop_front() {
                let chunk = persistent.restore(&ctx)?;
                let size = inner.strategy.chunk_size(&chunk);
                inner.queue_size -= size;
                inner.update_backpressure();
                Some(chunk)
            } else {
                None
            }
        };

        if let Some(chunk) = queue_chunk {
            let result = js::Object::new(ctx.clone())?;
            result.set("value", chunk)?;
            result.set("done", false)?;
            return Ok(result);
        }

        let state = self.inner.borrow().state;
        match state {
            ReadableStreamState::Closed => {
                let result = js::Object::new(ctx.clone())?;
                result.set("value", js::Value::new_undefined(ctx.clone()))?;
                result.set("done", true)?;
                return Ok(result);
            }
            ReadableStreamState::Errored => {
                return Err(js::Error::new_loading_message(
                    "ReadableStream",
                    "stream is errored",
                ));
            }
            ReadableStreamState::Readable => {}
        }

        let (source, needs_pull) = {
            let mut inner = self.inner.borrow_mut();
            let source = inner
                .source
                .as_ref()
                .ok_or_else(|| {
                    js::Error::new_loading_message("ReadableStream", "source not found")
                })?
                .clone()
                .restore(&ctx)?;

            let needs_pull = !inner.pulling && !inner.backpressure;
            if needs_pull {
                inner.pulling = true;
                inner.pending_io.increment();
            }
            (source, needs_pull)
        };

        if needs_pull {
            if let Ok(pull_fn) = source.get::<_, js::Function<'js>>("pull") {
                let controller_inner = self.inner.clone();
                let controller = ReadableStreamDefaultController {
                    inner: controller_inner,
                };
                let controller_obj = js::Class::instance(ctx.clone(), controller)?;

                let pull_result: js::Result<js::Value<'js>> = pull_fn.call((controller_obj,));

                if let Ok(val) = &pull_result
                    && let Ok(promise) = js::Promise::from_value(val.clone())
                {
                    let _: js::Result<js::Value<'js>> = promise.into_future().await;
                }

                {
                    let mut inner = self.inner.borrow_mut();
                    inner.pending_io.decrement();
                    inner.pulling = false;
                }

                if let Err(e) = pull_result {
                    let err_msg = format!("{:?}", e);
                    let err_str = js::String::from_str(ctx.clone(), &err_msg)?;
                    let mut inner = self.inner.borrow_mut();
                    inner.error(&ctx, err_str.into_value())?;
                }
            } else {
                let mut inner = self.inner.borrow_mut();
                inner.pending_io.decrement();
                inner.pulling = false;
            }
        }

        let (promise, resolve, reject) = js::Promise::new(&ctx)?;
        let resolve_persistent = js::Persistent::save(&ctx, resolve);
        let reject_persistent = js::Persistent::save(&ctx, reject);

        self.inner
            .borrow_mut()
            .pending_reads
            .push_back(ReadRequest {
                resolve: resolve_persistent,
                reject: reject_persistent,
            });

        let result: js::Object<'js> = promise.into_future().await?;
        Ok(result)
    }

    async fn cancel<'js>(
        &self,
        ctx: js::Ctx<'js>,
        reason: js::function::Opt<js::Value<'js>>,
    ) -> js::Result<()> {
        let (source, pending_reads) = {
            let mut inner = self.inner.borrow_mut();
            inner.locked = false;

            if inner.state != ReadableStreamState::Readable {
                return Ok(());
            }
            inner.state = ReadableStreamState::Closed;

            let source = inner
                .source
                .as_ref()
                .ok_or_else(|| {
                    js::Error::new_loading_message("ReadableStream", "source not found")
                })?
                .clone()
                .restore(&ctx)?;

            let pending_reads = std::mem::take(&mut inner.pending_reads);

            if inner.pulling {
                inner.pending_io.decrement();
                inner.pulling = false;
            }

            (source, pending_reads)
        };

        if let Ok(cancel_fn) = source.get::<_, js::Function<'js>>("cancel") {
            let reason_val = reason
                .0
                .unwrap_or_else(|| js::Value::new_undefined(ctx.clone()));
            let result: js::Value<'js> = cancel_fn.call((reason_val,))?;
            if let Ok(promise) = js::Promise::from_value(result) {
                let _: js::Value<'js> = promise.into_future().await?;
            }
        }

        for req in pending_reads {
            let resolve = req.resolve.restore(&ctx)?;
            let result = js::Object::new(ctx.clone())?;
            result.set("value", js::Value::new_undefined(ctx.clone()))?;
            result.set("done", true)?;
            resolve.call::<_, ()>((result,))?;
        }

        Ok(())
    }

    #[qjs(rename = "releaseLock")]
    fn release_lock(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.locked = false;
    }

    #[qjs(get)]
    fn closed<'js>(&self, ctx: js::Ctx<'js>) -> js::Result<js::Promise<'js>> {
        let (promise, resolve, _) = js::Promise::new(&ctx)?;
        resolve.call::<_, ()>((js::Value::new_undefined(ctx.clone()),))?;
        Ok(promise)
    }
}

#[js::methods]
impl ReadableStreamAsyncIterator {
    async fn next<'js>(&self, ctx: js::Ctx<'js>) -> js::Result<js::Object<'js>> {
        let reader = {
            let reader_ref = self.reader.borrow();
            reader_ref
                .as_ref()
                .ok_or_else(|| js::Error::new_from_js("iterator", "reader already consumed"))?
                .clone()
        };

        reader.read(ctx).await
    }

    #[qjs(rename = "Symbol.asyncIterator")]
    fn async_iterator<'js>(
        &self,
        ctx: js::Ctx<'js>,
    ) -> js::Result<js::Class<'js, ReadableStreamAsyncIterator>> {
        let iterator = ReadableStreamAsyncIterator {
            reader: self.reader.clone(),
        };
        js::Class::instance(ctx, iterator)
    }

    #[qjs(constructor)]
    fn _construct(_ctx: js::Ctx<'_>) -> js::Result<()> {
        Err(js::Error::new_from_js(
            "constructor",
            "ReadableStreamAsyncIterator",
        ))
    }
}

pub fn init_async_iterator<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    use js::class::Class;
    let globals = ctx.globals();
    let stream_constructor: js::Object<'js> = globals.get("ReadableStream")?;
    let stream_proto: js::Object<'js> = stream_constructor.get("prototype")?;
    let symbol_obj: js::Object<'js> = globals.get("Symbol")?;
    let symbol_async_iterator: js::Symbol = symbol_obj.get("asyncIterator")?;

    let ctx_clone = ctx.clone();
    let async_iter_fn = Function::new(
        ctx.clone(),
        move |this: This<Class<'js, ReadableStream>>| -> js::Result<js::Class<'js, ReadableStreamAsyncIterator>> {
            let ctx = ctx_clone.clone();
            let stream = this.0.borrow();
            let mut inner = stream.inner.borrow_mut();
            if inner.locked {
                return Err(throw_type_error(&ctx, "ERR_STREAM_LOCKED", "ReadableStream is locked"));
            }
            inner.locked = true;
            drop(inner);

            let reader = ReadableStreamDefaultReader {
                inner: stream.inner.clone(),
            };

            let iterator = ReadableStreamAsyncIterator {
                reader: Rc::new(RefCell::new(Some(reader))),
            };

            js::Class::instance(ctx, iterator)
        },
    )?;

    stream_proto.set(symbol_async_iterator, async_iter_fn)?;
    Ok(())
}

pub fn setup_dispose<'js>(ctx: &js::Ctx<'js>) -> js::Result<()> {
    use js::class::Class;

    if let Some(proto) = Class::<ReadableStream>::prototype(ctx)? {
        let symbol_obj: js::Object = ctx.globals().get("Symbol")?;
        let dispose: js::Symbol = symbol_obj.get("dispose")?;
        let dispose_fn = Function::new(
            ctx.clone(),
            |this: This<Class<'js, ReadableStream>>| -> js::Result<()> {
                let stream = this.0.borrow_mut();
                let mut inner = stream.inner.borrow_mut();
                if inner.state == ReadableStreamState::Readable {
                    inner.state = ReadableStreamState::Closed;
                }
                Ok(())
            },
        )?;
        proto.set(dispose, dispose_fn)?;
    }

    if let Some(proto) = Class::<ReadableStreamDefaultReader>::prototype(ctx)? {
        let symbol_obj: js::Object = ctx.globals().get("Symbol")?;
        let dispose: js::Symbol = symbol_obj.get("dispose")?;
        let dispose_fn = Function::new(
            ctx.clone(),
            |this: This<Class<'js, ReadableStreamDefaultReader>>| -> js::Result<()> {
                let reader = this.0.borrow_mut();
                let mut inner = reader.inner.borrow_mut();
                inner.locked = false;
                Ok(())
            },
        )?;
        proto.set(dispose, dispose_fn)?;
    }

    Ok(())
}

pub fn create_readable_stream<'js>(
    ctx: &js::Ctx<'js>,
    source: js::Object<'js>,
    strategy: QueuingStrategy,
    pending_io: PendingIo,
) -> js::Result<js::Class<'js, ReadableStream>> {
    let high_water_mark = strategy.high_water_mark();

    let stream = ReadableStream {
        inner: Rc::new(RefCell::new(ReadableStreamInner {
            state: ReadableStreamState::Readable,
            queue: VecDeque::new(),
            pending_reads: VecDeque::new(),
            source: Some(js::Persistent::save(ctx, source)),
            pulling: false,
            locked: false,
            pending_io,
            high_water_mark,
            queue_size: 0,
            strategy,
            backpressure: false,
        })),
    };
    js::Class::instance(ctx.clone(), stream)
}

pub fn create_preloaded_stream<'js>(
    ctx: &js::Ctx<'js>,
    chunks: Vec<js::Value<'js>>,
    strategy: QueuingStrategy,
    pending_io: PendingIo,
) -> js::Result<js::Class<'js, ReadableStream>> {
    let high_water_mark = strategy.high_water_mark();

    let mut queue_size: usize = 0;
    let mut queue = VecDeque::new();
    for chunk in chunks {
        queue_size += strategy.chunk_size(&chunk);
        queue.push_back(js::Persistent::save(ctx, chunk));
    }

    let mut inner = ReadableStreamInner {
        state: ReadableStreamState::Readable,
        queue,
        pending_reads: VecDeque::new(),
        source: None,
        pulling: false,
        locked: false,
        pending_io,
        high_water_mark,
        queue_size,
        strategy,
        backpressure: false,
    };
    inner.close(ctx)?;

    let stream = ReadableStream {
        inner: Rc::new(RefCell::new(inner)),
    };
    js::Class::instance(ctx.clone(), stream)
}
