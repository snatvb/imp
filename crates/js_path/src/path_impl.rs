use std::any::TypeId;

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
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    let suffix = suffix
        .0
        .map(|s| StringArg::coerce_js(ctx, &s, "suffix").map(|a| a.as_str().to_string()))
        .transpose()?;
    let p = PlatformPathBuf::<B>::new(path_str);
    Ok(base(&p, suffix.as_deref()).into())
}

pub fn dirname<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<String> {
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    let p = PlatformPathBuf::<B>::new(path_arg.as_str());
    Ok(dir(&p).into())
}

pub fn extname<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<String> {
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    let p = PlatformPathBuf::<B>::new(path_arg.as_str());
    Ok(p.extension().unwrap_or("").into())
}

pub fn normalize<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<String> {
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    Ok(PlatformPathBuf::<B>::new(path_arg.as_str())
        .normalize()
        .into_string())
}

pub fn is_absolute<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<bool> {
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    Ok(PlatformPathBuf::<B>::new(path_arg.as_str()).is_absolute())
}

#[allow(clippy::extra_unused_type_parameters)]
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
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    let p = PlatformPathBuf::<B>::new(path_str);

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

#[allow(clippy::extra_unused_type_parameters)]
pub fn to_namespaced_path<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<js::Value<'js>> {
    let path_arg = StringArg::coerce_js(ctx, &path, "path")?;
    let s = path_arg.as_str();

    let result: String = if TypeId::of::<B>() == TypeId::of::<os_path::Posix>()
        || s.starts_with(r"\\?\")
        || s.starts_with(r"\\.\")
    {
        s.to_string()
    } else {
        let s = s.replace('/', "\\");
        if s.starts_with("\\\\") && !s.starts_with(r"\\?\") {
            format!(r"\\?\UNC\{}", &s[2..])
        } else if s.len() >= 2 && s.as_bytes()[0].is_ascii_alphabetic() && s.as_bytes()[1] == b':' {
            let (drive, rest) = s.split_at(2);
            let rest = if rest.is_empty() || !rest.starts_with('\\') {
                format!("\\{}", rest)
            } else {
                rest.to_string()
            };
            format!(r"\\?\{}", drive) + &rest
        } else {
            s
        }
    };

    Ok(js::String::from_str(ctx.clone(), &result)?.into())
}

pub fn relative<'js, B: PathBackend>(
    ctx: &js::Ctx<'js>,
    from: js::Value<'js>,
    to: js::Value<'js>,
) -> js::Result<String> {
    let from_arg = StringArg::coerce_js(ctx, &from, "from")?;
    let to_arg = StringArg::coerce_js(ctx, &to, "to")?;
    let from_str = from_arg.as_str().to_string();
    let to_str = to_arg.as_str().to_string();

    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(ctx))?;
    let base = PlatformPathBuf::<B>::from_path_buf(cwd)
        .map_err(|p| PathError::invalid_path(format!("invalid path: {p:?}")).into_exception(ctx))?;

    let from_resolved = resolve_paths_inner(&[from_str], base.clone());
    let to_resolved = resolve_paths_inner(&[to_str], base);

    Ok(from_resolved.relative_to(&to_resolved).into_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use os_path::{Posix, Win32};

    fn win32_path(s: &str) -> String {
        to_namespaced_path_impl::<Win32>(s)
    }

    fn posix_path(s: &str) -> String {
        to_namespaced_path_impl::<Posix>(s)
    }

    fn to_namespaced_path_impl<B: PathBackend>(s: &str) -> String {
        let s = String::from(s);
        if TypeId::of::<B>() == TypeId::of::<Posix>() {
            return s;
        }

        if s.starts_with(r"\\?\") || s.starts_with(r"\\.\") {
            return s;
        }

        let s = s.replace('/', "\\");

        if s.starts_with("\\\\") && !s.starts_with(r"\\?\") {
            return format!(r"\\?\UNC\{}", &s[2..]);
        }

        if s.len() >= 2 && s.as_bytes()[0].is_ascii_alphabetic() && s.as_bytes()[1] == b':' {
            let (drive, rest) = s.split_at(2);
            let rest = if rest.is_empty() || !rest.starts_with('\\') {
                format!("\\{}", rest)
            } else {
                rest.to_string()
            };
            return format!(r"\\?\{}", drive) + &rest;
        }

        s
    }

    #[test]
    fn posix_noop() {
        assert_eq!(posix_path("/foo/bar"), "/foo/bar");
        assert_eq!(posix_path("C:\\foo"), "C:\\foo");
        assert_eq!(posix_path("\\\\server\\share"), "\\\\server\\share");
    }

    #[test]
    fn win32_already_namespaced() {
        assert_eq!(win32_path(r"\\?\C:\foo"), r"\\?\C:\foo");
        assert_eq!(win32_path(r"\\?\UNC\server\share"), r"\\?\UNC\server\share");
        assert_eq!(win32_path(r"\\.\PhysicalDrive0"), r"\\.\PhysicalDrive0");
    }

    #[test]
    fn win32_unc() {
        assert_eq!(
            win32_path(r"\\server\share\foo"),
            r"\\?\UNC\server\share\foo"
        );
        assert_eq!(win32_path(r"\\server\share"), r"\\?\UNC\server\share");
    }

    #[test]
    fn win32_drive_path() {
        assert_eq!(win32_path(r"C:\foo"), r"\\?\C:\foo");
        assert_eq!(win32_path(r"D:\bar\baz"), r"\\?\D:\bar\baz");
    }

    #[test]
    fn win32_forward_slash() {
        assert_eq!(win32_path("C:/foo"), r"\\?\C:\foo");
        assert_eq!(win32_path("C:/foo/bar"), r"\\?\C:\foo\bar");
    }

    #[test]
    fn win32_drive_only() {
        assert_eq!(win32_path("C:"), r"\\?\C:\");
        assert_eq!(win32_path("C:foo"), r"\\?\C:\foo");
    }

    #[test]
    fn win32_relative() {
        assert_eq!(win32_path(r"foo\bar"), r"foo\bar");
        assert_eq!(win32_path("foo"), "foo");
        assert_eq!(win32_path("."), ".");
        assert_eq!(win32_path(".."), "..");
    }

    #[test]
    fn win32_empty() {
        assert_eq!(win32_path(""), "");
    }
}
