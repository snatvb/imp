use crate::csv;
use crate::json;
use crate::msgpack;
use crate::prelude::*;
use crate::ron;
use crate::toml;
use crate::xml;
use crate::yaml;

js_core::impl_module!(ParsersModule,
    declare: |decl, declare_all| {
        decl.declare("json")?;
        decl.declare("yaml")?;
        decl.declare("xml")?;
        decl.declare("toml")?;
        decl.declare("ron")?;
        decl.declare("csv")?;
        decl.declare("msgpack")?;
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

        let toml_obj = js::Object::new(ctx.clone())?;
        toml_obj.set("parse", js::Function::new(ctx.clone(), toml::js_parse)?)?;
        toml_obj.set("stringify", js::Function::new(ctx.clone(), toml::js_stringify)?)?;

        let ron_obj = js::Object::new(ctx.clone())?;
        ron_obj.set("parse", js::Function::new(ctx.clone(), ron::js_parse)?)?;
        ron_obj.set("stringify", js::Function::new(ctx.clone(), ron::js_stringify)?)?;

        let csv_obj = js::Object::new(ctx.clone())?;
        csv_obj.set("parse", js::Function::new(ctx.clone(), csv::js_parse)?)?;
        csv_obj.set("stringify", js::Function::new(ctx.clone(), csv::js_stringify)?)?;

        let msgpack_obj = js::Object::new(ctx.clone())?;
        msgpack_obj.set("parse", js::Function::new(ctx.clone(), msgpack::js_parse)?)?;
        msgpack_obj.set("stringify", js::Function::new(ctx.clone(), msgpack::js_stringify)?)?;

        let ns = js::Object::new(ctx.clone())?;
        ns.set("json", json_obj.clone())?;
        ns.set("yaml", yaml_obj.clone())?;
        ns.set("xml", xml_obj.clone())?;
        ns.set("toml", toml_obj.clone())?;
        ns.set("ron", ron_obj.clone())?;
        ns.set("csv", csv_obj.clone())?;
        ns.set("msgpack", msgpack_obj.clone())?;

        exports.export("json", json_obj)?;
        exports.export("yaml", yaml_obj)?;
        exports.export("xml", xml_obj)?;
        exports.export("toml", toml_obj)?;
        exports.export("ron", ron_obj)?;
        exports.export("csv", csv_obj)?;
        exports.export("msgpack", msgpack_obj)?;
        exports.export("default", ns)?;
        Ok(())
    },
);
