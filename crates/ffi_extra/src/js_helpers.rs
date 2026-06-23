use rquickjs as js;

pub fn is_job_pending(ctx: &js::Ctx<'_>) -> bool {
    unsafe {
        let rt = js::qjs::JS_GetRuntime(ctx.as_raw().as_ptr());
        js::qjs::JS_IsJobPending(rt)
    }
}

pub fn save_persistent_fn(
    ctx: &js::Ctx<'_>,
    cb: js::Function<'_>,
) -> js::Persistent<js::Function<'static>> {
    unsafe {
        let ctx_static: &js::Ctx<'static> = std::mem::transmute(ctx);
        let cb_static: js::Function<'static> = std::mem::transmute(cb);
        js::Persistent::save(ctx_static, cb_static)
    }
}
