use crate::json;
use crate::prelude::*;

js_core::impl_module!(ParsersModule,
    declare: |decl, declare_all| {
        decl.declare("json")?;
        decl.declare("default")?;
        Ok(())
    },
    evaluate: |ctx, exports, export_all| {
        let json_obj = js::Object::new(ctx.clone())?;
        json_obj.set("parse", js::Function::new(ctx.clone(), json::js_parse)?)?;
        json_obj.set("stringify", js::Function::new(ctx.clone(), json::js_stringify)?)?;

        let ns = js::Object::new(ctx.clone())?;
        ns.set("json", json_obj.clone())?;

        exports.export("json", json_obj)?;
        exports.export("default", ns)?;
        Ok(())
    },
);
