use crate::json;
use crate::prelude::*;
use crate::xml;
use crate::yaml;

js_core::impl_module!(ParsersModule,
    declare: |decl, declare_all| {
        decl.declare("json")?;
        decl.declare("yaml")?;
        decl.declare("xml")?;
        decl.declare("default")?;
        Ok(())
    },
    evaluate: |ctx, exports, export_all| {
        let json_obj = js::Object::new(ctx.clone())?;
        json_obj.set("parse", js::Function::new(ctx.clone(), json::js_parse)?)?;
        json_obj.set("stringify", js::Function::new(ctx.clone(), json::js_stringify)?)?;

        let yaml_obj = js::Object::new(ctx.clone())?;
        yaml_obj.set("parse", js::Function::new(ctx.clone(), yaml::js_parse)?)?;
        yaml_obj.set("stringify", js::Function::new(ctx.clone(), yaml::js_stringify)?)?;

        let xml_obj = js::Object::new(ctx.clone())?;
        xml_obj.set("parse", js::Function::new(ctx.clone(), xml::js_parse)?)?;
        xml_obj.set("stringify", js::Function::new(ctx.clone(), xml::js_stringify)?)?;

        let ns = js::Object::new(ctx.clone())?;
        ns.set("json", json_obj.clone())?;
        ns.set("yaml", yaml_obj.clone())?;
        ns.set("xml", xml_obj.clone())?;

        exports.export("json", json_obj)?;
        exports.export("yaml", yaml_obj)?;
        exports.export("xml", xml_obj)?;
        exports.export("default", ns)?;
        Ok(())
    },
);
