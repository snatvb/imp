use js::function::Opt;
use js_core::js;

use crate::handler::{self, SignalHandle, SignalName};
use crate::platform;

pub struct SignalModule;

impl js::module::ModuleDef for SignalModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("on")?;
        decl.declare("once")?;
        decl.declare("removeAll")?;
        decl.declare("pending")?;
        decl.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        let handle = SignalHandle::new();
        handler::set_handle(handle.clone());

        let ns = js::Object::new(ctx.clone())?;

        let h = handle.clone();
        let ctx_clone = ctx.clone();
        let on_fn = js::Function::new(ctx.clone(), move |name: String, cb: js::Function<'_>| {
            let sig = match SignalName::from_js(&name) {
                Some(s) => s,
                None => {
                    return Err(js::Error::Exception);
                }
            };
            let persistent = ffi_extra::js_helpers::save_persistent_fn(&ctx_clone, cb);
            let id = h.add_handler(sig, persistent, false);

            if h.should_listen(sig) {
                let tx = h.tx().clone();
                tokio::spawn(async move {
                    platform::listen_signal(sig, tx).await;
                });
            }

            let h2 = h.clone();
            let dispose = js::Function::new(ctx_clone.clone(), move || {
                h2.remove_handler(sig, id);
            })?;
            Ok(dispose)
        })?;
        ns.set("on", on_fn.clone())?;

        let h = handle.clone();
        let ctx_clone = ctx.clone();
        let once_fn = js::Function::new(ctx.clone(), move |name: String, cb: js::Function<'_>| {
            let sig = match SignalName::from_js(&name) {
                Some(s) => s,
                None => {
                    return Err(js::Error::Exception);
                }
            };
            let persistent = ffi_extra::js_helpers::save_persistent_fn(&ctx_clone, cb);
            let id = h.add_handler(sig, persistent, true);

            if h.should_listen(sig) {
                let tx = h.tx().clone();
                tokio::spawn(async move {
                    platform::listen_signal(sig, tx).await;
                });
            }

            let h2 = h.clone();
            let dispose = js::Function::new(ctx_clone.clone(), move || {
                h2.remove_handler(sig, id);
            })?;
            Ok(dispose)
        })?;
        ns.set("once", once_fn.clone())?;

        let h = handle.clone();
        let remove_all_fn = js::Function::new(ctx.clone(), move |name: Opt<String>| {
            let sig = name.0.and_then(|n| SignalName::from_js(&n));
            h.remove_all(sig);
        })?;
        ns.set("removeAll", remove_all_fn.clone())?;

        let h = handle.clone();
        let pending_fn = js::Function::new(ctx.clone(), move || -> Vec<String> {
            h.pending_signals()
                .iter()
                .map(|s| s.as_str().to_string())
                .collect()
        })?;
        ns.set("pending", pending_fn.clone())?;

        exports.export("on", on_fn)?;
        exports.export("once", once_fn)?;
        exports.export("removeAll", remove_all_fn)?;
        exports.export("pending", pending_fn)?;
        exports.export("default", ns)?;
        Ok(())
    }
}
