use os_path::{PathBackend, PlatformPathBuf};

use crate::error::PathError;
use crate::prelude::*;
use crate::utils::as_strings;

fn base<'a, B: PathBackend>(p: &'a PlatformPathBuf<B>, suffix: Option<&str>) -> &'a str {
    p.file_name()
        .map(|f| suffix.map(|s| f.trim_end_matches(&s)).unwrap_or(f))
        .unwrap_or("")
}

fn dir<B: PathBackend>(p: &PlatformPathBuf<B>) -> &str {
    p.parent().map(|p| p.as_str()).unwrap_or(".")
}

fn resolve_paths_inner<B: PathBackend>(
    paths: &[String],
    base: PlatformPathBuf<B>,
) -> PlatformPathBuf<B> {
    paths.iter().fold(base, |acc, p| acc.resolve(p))
}

pub fn resolve<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    args: js::prelude::Rest<js::Value<'js>>,
) -> js::Result<String> {
    let paths = as_strings(ctx, args)?;
    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(ctx))?;
    let base = PlatformPathBuf::<B>::from_path_buf(cwd)
        .map_err(|p| PathError::invalid_path(format!("invalid path: {p:?}")).into_exception(ctx))?;
    Ok(resolve_paths_inner(&paths, base).normalize().into_string())
}

pub fn join<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    args: js::prelude::Rest<js::Value<'js>>,
    sep: &str,
) -> js::Result<String> {
    let paths = as_strings(ctx, args)?;
    let joined = paths.join(sep);
    Ok(PlatformPathBuf::<B>::new(joined).normalize().into_string())
}

pub fn basename<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
    suffix: js::prelude::Opt<js::Value<'js>>,
) -> js::Result<String> {
    let path_str = String::coerce_js(ctx, &path, "path")?;
    let suffix = suffix
        .0
        .map(|s| String::coerce_js(ctx, &s, "suffix"))
        .transpose()?;
    let p = PlatformPathBuf::<B>::new(path_str);
    Ok(base(&p, suffix.as_deref()).into())
}

pub fn dirname<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<String> {
    let path_str = String::coerce_js(ctx, &path, "path")?;
    let p = PlatformPathBuf::<B>::new(path_str);
    Ok(dir(&p).into())
}

pub fn extname<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<String> {
    let path_str = String::coerce_js(ctx, &path, "path")?;
    let p = PlatformPathBuf::<B>::new(path_str);
    Ok(p.extension().unwrap_or("").into())
}

pub fn normalize<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<String> {
    let path_str = String::coerce_js(ctx, &path, "path")?;
    Ok(PlatformPathBuf::<B>::new(path_str)
        .normalize()
        .into_string())
}

pub fn is_absolute<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<bool> {
    let path_str = String::coerce_js(ctx, &path, "path")?;
    Ok(PlatformPathBuf::<B>::new(path_str).is_absolute())
}

pub fn format<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    arg: js::Value<'js>,
    sep: &str,
) -> js::Result<String> {
    let obj = js::Object::coerce_js(ctx, &arg, "arg")?;

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
        Ok(format!("{d}{sep}{base_val}"))
    } else if let Some(r) = root {
        Ok(format!("{r}{base_val}"))
    } else {
        Ok(base_val)
    }
}

pub fn parse<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<js::Object<'js>> {
    let path_str = String::coerce_js(ctx, &path, "path")?;
    let p = PlatformPathBuf::<B>::new(&path_str);

    let root = p.root().to_string();
    let base = p.file_name().unwrap_or("").to_string();
    let ext = p.extension().map(|e| format!(".{e}")).unwrap_or_default();
    let name = if ext.is_empty() {
        base.clone()
    } else {
        base[..base.len() - ext.len()].to_string()
    };
    let dir = match p.parent() {
        Some(p) => p.as_str().to_string(),
        None => root.clone(),
    };

    let res = js::Object::new(ctx.clone())?;
    res.set("root", &root)?;
    res.set("dir", &dir)?;
    res.set("base", &base)?;
    res.set("name", &name)?;
    res.set("ext", &ext)?;

    Ok(res)
}

pub fn relative<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    from: js::Value<'js>,
    to: js::Value<'js>,
) -> js::Result<String> {
    let from_str = String::coerce_js(ctx, &from, "from")?;
    let to_str = String::coerce_js(ctx, &to, "to")?;

    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(ctx))?;
    let base = PlatformPathBuf::<B>::from_path_buf(cwd)
        .map_err(|p| PathError::invalid_path(format!("invalid path: {p:?}")).into_exception(ctx))?;

    let from_resolved = resolve_paths_inner(&[from_str], base.clone());
    let to_resolved = resolve_paths_inner(&[to_str], base);

    Ok(from_resolved.relative_to(&to_resolved).into_string())
}
