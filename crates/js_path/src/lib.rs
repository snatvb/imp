use crate::macros::make_path_wrappers;
use rquickjs::{function, module::ModuleDef};

mod error;
mod macros;
mod path_impl;
mod posix_mod;
mod prelude;
mod utils;
mod win32_mod;

use prelude::*;

const DELIMITER: &str = if cfg!(windows) { ";" } else { ":" };

make_path_wrappers!(os_path::Native, std::path::MAIN_SEPARATOR_STR, DELIMITER);

pub struct PathModule;

impl ModuleDef for PathModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("default")?;
        decl.declare("resolve")?;
        decl.declare("isAbsolute")?;
        decl.declare("toNamespacedPath")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        exports.export("resolve", js_resolve)?;
        exports.export("isAbsolute", js_is_absolute)?;
        exports.export("toNamespacedPath", js_to_namespaced_path)?;

        let win32_ns = js::Object::new(ctx.clone())?;
        win32_mod::populate(&win32_ns)?;

        let posix_ns = js::Object::new(ctx.clone())?;
        posix_mod::populate(&posix_ns)?;

        let ns = js::Object::new(ctx.clone())?;
        populate(&ns)?;
        ns.set("win32", win32_ns)?;
        ns.set("posix", posix_ns)?;

        exports.export("default", ns)?;

        Ok(())
    }
}
