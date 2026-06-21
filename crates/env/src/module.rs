use crate::dotenv;
use crate::expand;
use crate::ini;
use crate::loadfile;
use crate::merge;

js_core::impl_module!(EnvModule,
    declare: |decl, declare_all| {
        declare_all(decl)?;
        decl.declare("env")?;
        decl.declare("default")?;
        Ok(())
    },
    evaluate: |ctx, exports, export_all| {
        let ns = export_all(ctx, exports)?;
        exports.export("env", ns.clone())?;
        exports.export("default", ns)?;
        Ok(())
    },
    "parseIni" => ini::js_parse_ini,
    "parseDotenv" => dotenv::js_parse_dotenv,
    "expand" => expand::js_expand,
    "merge" => merge::js_merge,
    "loadFile" => loadfile::js_load_file,
);
