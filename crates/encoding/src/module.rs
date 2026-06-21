use js_core::js;

use crate::base64;
use crate::hex;
use crate::uri;
use crate::utf8;

js_core::impl_module!(EncodingModule,
    declare: |decl, declare_all| {
        decl.declare("base64")?;
        decl.declare("hex")?;
        decl.declare("utf8")?;
        decl.declare("uri")?;
        decl.declare("default")?;
        Ok(())
    },
    evaluate: |ctx, exports, export_all| {
        let base64_obj = base64::make_module(ctx)?;
        let hex_obj = hex::make_module(ctx)?;
        let utf8_obj = utf8::make_module(ctx)?;
        let uri_obj = uri::make_module(ctx)?;

        let ns = js::Object::new(ctx.clone())?;
        ns.set("base64", base64_obj.clone())?;
        ns.set("hex", hex_obj.clone())?;
        ns.set("utf8", utf8_obj.clone())?;
        ns.set("uri", uri_obj.clone())?;

        exports.export("base64", base64_obj)?;
        exports.export("hex", hex_obj)?;
        exports.export("utf8", utf8_obj)?;
        exports.export("uri", uri_obj)?;
        exports.export("default", ns)?;
        Ok(())
    },
);
