use std::path::Path;

use rquickjs::{Object, function, module::ModuleDef, prelude::Rest};

mod error;
mod prelude;
mod utils;

use error::PathError;

use crate::prelude::*;
use crate::utils::as_strings;

#[function]
pub fn resolve<'js>(
    ctx: rquickjs::Ctx<'js>,
    args: Rest<rquickjs::Value<'js>>,
) -> rquickjs::Result<String> {
    let paths = as_strings(&ctx, args)?;

    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(&ctx))?;

    let mut base = os_path::OsPathBuf::from_path_buf(cwd).map_err(|p| {
        PathError::invalid_path(format!("current directory path is not valid UTF-8: {p:?}"))
            .into_exception(&ctx)
    })?;

    for arg in &paths {
        if arg.is_empty() {
            continue;
        }
        if Path::new(arg.as_str()).is_absolute() {
            base = os_path::OsPathBuf::new(arg.as_str());
        } else {
            base.push(arg.as_str());
        }
    }

    Ok(base.into_string())
}

#[function]
pub fn join<'js>(
    ctx: rquickjs::Ctx<'js>,
    args: Rest<rquickjs::Value<'js>>,
) -> rquickjs::Result<String> {
    let paths = as_strings(&ctx, args)?;
    let mut base = os_path::OsPathBuf::new("");

    for arg in &paths {
        if arg.is_empty() {
            continue;
        }
        if Path::new(arg.as_str()).is_absolute() {
            base = os_path::OsPathBuf::new(arg.as_str());
        } else {
            base.push(arg.as_str());
        }
    }
    Ok(base.into_string())
}

#[function]
pub fn basename<'js>(
    ctx: js::Ctx<'js>,
    path: js::Value<'js>,
    suffix: js::prelude::Opt<js::Value<'js>>,
) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let suffix = match &suffix.0 {
        Some(suffix) => Some(String::coerce_js(&ctx, suffix, "suffix")?),
        _ => None,
    };
    let ospath = os_path::OsPathBuf::from_path_buf(path.into()).map_err(|p| {
        PathError::invalid_path(format!("current directory path is not valid UTF-8: {p:?}"))
            .into_exception(&ctx)
    })?;
    Ok(ospath
        .file_name()
        .map(|f| suffix.map(|s| f.trim_end_matches(&s)).unwrap_or(f))
        .unwrap_or("")
        .into())
}

#[function]
pub fn dirname<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let ospath = os_path::OsPathBuf::from_path_buf(path.into()).map_err(|p| {
        PathError::invalid_path(format!("current directory path is not valid UTF-8: {p:?}"))
            .into_exception(&ctx)
    })?;
    Ok(ospath.parent().map(|p| p.as_str()).unwrap_or(".").into())
}

#[function]
pub fn extname<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let ospath = os_path::OsPathBuf::from_path_buf(path.into()).map_err(|p| {
        PathError::invalid_path(format!("current directory path is not valid UTF-8: {p:?}"))
            .into_exception(&ctx)
    })?;
    Ok(ospath.extension().unwrap_or("").into())
}

#[cfg(not(windows))]
const SEPARATOR: &str = "/";
#[cfg(windows)]
const SEPARATOR: &str = "\\";

#[cfg(not(windows))]
const DELIMITER: &str = ":";
#[cfg(windows)]
const DELIMITER: &str = ";";

pub struct PathModule;

impl ModuleDef for PathModule {
    fn declare<'js>(decl: &rquickjs::module::Declarations<'js>) -> rquickjs::Result<()> {
        decl.declare("default")?;
        decl.declare("resolve")?;
        Ok(())
    }

    fn evaluate<'js>(
        ctx: &rquickjs::Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        exports.export("resolve", js_resolve)?;

        let ns = Object::new(ctx.clone())?;
        ns.set("resolve", js_resolve)?;
        ns.set("join", js_join)?;
        ns.set("basename", js_basename)?;
        ns.set("dirname", js_basename)?;
        ns.set("extname", js_basename)?;
        ns.set("sep", SEPARATOR)?;
        ns.set("delimiter", DELIMITER)?;
        exports.export("default", ns)?;

        Ok(())
    }
}
