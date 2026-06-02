use os_path::OsPathBuf;
use rquickjs::{Object, function, module::ModuleDef};
use std::path::MAIN_SEPARATOR_STR as SEPARATOR;

mod error;
mod prelude;
mod utils;

use error::PathError;

use crate::prelude::*;
use crate::utils::{as_strings, resolve_paths, to_ospath};

#[function]
pub fn resolve<'js>(
    ctx: js::Ctx<'js>,
    args: js::prelude::Rest<js::Value<'js>>,
) -> js::Result<String> {
    let paths = as_strings(&ctx, args)?;

    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(&ctx))?;

    let base = to_ospath(&ctx, cwd)?;

    Ok(resolve_paths(&paths, base).normalize().into_string())
}

#[function]
pub fn join<'js>(ctx: js::Ctx<'js>, args: js::prelude::Rest<js::Value<'js>>) -> js::Result<String> {
    let paths = as_strings(&ctx, args)?;
    let joined = paths.join(SEPARATOR);
    Ok(OsPathBuf::new(joined).normalize().into_string())
}

fn base<'a>(ospath: &'a OsPathBuf, suffix: Option<&str>) -> &'a str {
    ospath
        .file_name()
        .map(|f| suffix.map(|s| f.trim_end_matches(&s)).unwrap_or(f))
        .unwrap_or("")
}

#[function]
pub fn basename<'js>(
    ctx: js::Ctx<'js>,
    path: js::Value<'js>,
    suffix: js::prelude::Opt<js::Value<'js>>,
) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let suffix = suffix
        .0
        .map(|s| String::coerce_js(&ctx, &s, "suffix"))
        .transpose()?;
    let ospath = to_ospath(&ctx, path)?;
    Ok(base(&ospath, suffix.as_deref()).into())
}

fn dir(ospath: &OsPathBuf) -> &str {
    ospath.parent().map(|p| p.as_str()).unwrap_or(".")
}

#[function]
pub fn dirname<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let ospath = to_ospath(&ctx, path)?;
    Ok(dir(&ospath).into())
}

#[function]
pub fn extname<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let ospath = to_ospath(&ctx, path)?;
    Ok(ospath.extension().unwrap_or("").into())
}

#[function]
pub fn normalize<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let ospath = os_path::OsPathBuf::new(path);
    Ok(ospath.normalize().into_string())
}

#[function]
pub fn is_absolute<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<bool> {
    let path = String::coerce_js(&ctx, &path, "path")?;
    let ospath = os_path::OsPathBuf::new(path);

    if ospath.is_absolute() {
        return Ok(true);
    }

    #[cfg(windows)]
    {
        // Node: path starting with \ is absolute (UNC, \foo)
        let s = ospath.as_str();
        if s.starts_with('\\') {
            return Ok(true);
        }
    }

    Ok(false)
}

#[function]
pub fn format<'js>(ctx: js::Ctx<'js>, arg: js::Value<'js>) -> js::Result<String> {
    let obj = js::Object::coerce_js(&ctx, &arg, "arg")?;

    let dir: Option<String> = obj.get("dir")?;
    let root: Option<String> = obj.get("root")?;
    let base: Option<String> = obj.get("base")?;
    let name: Option<String> = obj.get("name")?;
    let ext: Option<String> = obj.get("ext")?;

    let base_val = match base {
        Some(b) => b,
        None => {
            let n = name.unwrap_or_default();
            let e = ext.unwrap_or_default();
            let e = if !e.is_empty() && !e.starts_with('.') {
                format!(".{e}")
            } else {
                e
            };
            format!("{n}{e}")
        }
    };

    if let Some(d) = dir {
        Ok(format!("{d}{SEPARATOR}{base_val}"))
    } else if let Some(r) = root {
        Ok(format!("{r}{base_val}"))
    } else {
        Ok(base_val)
    }
}

#[function]
pub fn parse<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<js::Object<'js>> {
    let path_str = String::coerce_js(&ctx, &path, "path")?;

    let ospath = os_path::OsPathBuf::new(&path_str);

    let root = ospath.root().to_string();
    let base = ospath.file_name().unwrap_or("").to_string();
    let ext = ospath
        .extension()
        .map(|e| format!(".{e}"))
        .unwrap_or_default();
    let name = if ext.is_empty() {
        base.clone()
    } else {
        base[..base.len() - ext.len()].to_string()
    };
    let dir = match ospath.parent() {
        Some(p) => p.as_str().to_string(),
        None => root.clone(),
    };

    let res = Object::new(ctx.clone())?;
    res.set("root", &root)?;
    res.set("dir", &dir)?;
    res.set("base", &base)?;
    res.set("name", &name)?;
    res.set("ext", &ext)?;

    Ok(res)
}

const DELIMITER: &str = if cfg!(windows) { ";" } else { ":" };

pub struct PathModule;

impl ModuleDef for PathModule {
    fn declare<'js>(decl: &rquickjs::module::Declarations<'js>) -> rquickjs::Result<()> {
        decl.declare("default")?;
        decl.declare("resolve")?;
        decl.declare("isAbsolute")?;
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
        ns.set("dirname", js_dirname)?;
        ns.set("extname", js_extname)?;
        ns.set("format", js_format)?;
        ns.set("normalize", js_normalize)?;
        ns.set("isAbsolute", js_is_absolute)?;
        ns.set("parse", js_parse)?;
        ns.set("sep", SEPARATOR)?;
        ns.set("delimiter", DELIMITER)?;
        exports.export("default", ns)?;

        Ok(())
    }
}
