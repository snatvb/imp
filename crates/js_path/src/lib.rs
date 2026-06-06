use crate::macros::make_path_wrappers;
use js_core::js;
use rquickjs::function;

mod error;
mod macros;
mod path_impl;
mod posix_mod;
mod prelude;
mod utils;
mod win32_mod;

const DELIMITER: &str = if cfg!(windows) { ";" } else { ":" };

make_path_wrappers!(os_path::Native, std::path::MAIN_SEPARATOR_STR, DELIMITER);

const EXPORT_NAMES: &[&str] = &[
    "resolve",
    "join",
    "basename",
    "dirname",
    "extname",
    "normalize",
    "format",
    "parse",
    "relative",
    "isAbsolute",
    "toNamespacedPath",
    "sep",
    "delimiter",
];

js_core::impl_module!(PathModule,
    declare: |decl, declare_all| {
        declare_all(decl)?;
        for name in EXPORT_NAMES {
            decl.declare(*name)?;
        }
        decl.declare("win32")?;
        decl.declare("posix")?;
        decl.declare("default")?;
        Ok(())
    },
    evaluate: |ctx, exports, export_all| {
        let ns = export_all(ctx, exports)?;
        populate(&ns)?;

        let win32_ns = js::Object::new(ctx.clone())?;
        win32_mod::populate(&win32_ns)?;

        let posix_ns = js::Object::new(ctx.clone())?;
        posix_mod::populate(&posix_ns)?;

        ns.set("win32", win32_ns.clone())?;
        ns.set("posix", posix_ns.clone())?;

        for name in EXPORT_NAMES {
            exports.export(*name, ns.get::<_, js::Value>(*name)?)?;
        }
        exports.export("win32", win32_ns)?;
        exports.export("posix", posix_ns)?;
        exports.export("default", ns)?;

        Ok(())
    },
);
