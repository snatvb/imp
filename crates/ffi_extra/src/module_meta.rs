use rquickjs as js;
use std::ffi::CString;

/// # Safety
/// Caller must ensure the Module's internal layout matches the expected representation:
/// `{ ptr: NonNull<JSModuleDef>, ctx: Ctx<'js>, PhantomData<T> }` (2 * usize on x64).
/// Verified at compile time by static assertion in lib.rs.
pub unsafe fn module_to_raw(module: &js::module::Module<'_>) -> *mut js::qjs::JSModuleDef {
    unsafe {
        std::ptr::read(module as *const js::module::Module as *const *mut js::qjs::JSModuleDef)
    }
}

pub fn set_module_meta(
    ctx: &js::Ctx<'_>,
    module: &js::module::Module<'_>,
    filename: &str,
    dirname: &str,
    url: &str,
) -> js::Result<()> {
    let ctx_ptr = ctx.as_raw().as_ptr();
    let module_ptr = unsafe { module_to_raw(module) };
    unsafe {
        let meta_val = js::qjs::JS_GetImportMeta(ctx_ptr, module_ptr);
        set_str_property(ctx_ptr, meta_val, "filename", filename);
        set_str_property(ctx_ptr, meta_val, "dirname", dirname);
        set_str_property(ctx_ptr, meta_val, "url", url);
    }
    Ok(())
}

unsafe fn set_str_property(
    ctx: *mut js::qjs::JSContext,
    obj: js::qjs::JSValue,
    key: &str,
    val: &str,
) {
    unsafe {
        let key_c = CString::new(key).unwrap();
        let val_c = CString::new(val).unwrap();
        let val_js = js::qjs::JS_NewStringLen(ctx, val_c.as_ptr(), val.len() as u64);
        js::qjs::JS_SetPropertyStr(ctx, obj, key_c.as_ptr(), val_js);
    }
}
