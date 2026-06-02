use crate::error::PathError;
use crate::prelude::*;

pub fn as_strings<'js>(
    ctx: &js::Ctx<'js>,
    args: js::prelude::Rest<js::Value<'js>>,
) -> js::Result<Vec<String>> {
    args.iter()
        .enumerate()
        .map(|(i, v)| String::coerce_js(ctx, v, format_args!("paths[{i}]")))
        .collect::<js::Result<Vec<_>>>()
}

pub fn resolve_paths(paths: &[String], base: os_path::OsPathBuf) -> os_path::OsPathBuf {
    paths.iter().fold(base, |acc, p| acc.resolve(p))
}

pub fn to_ospath<'js>(
    ctx: &js::Ctx<'js>,
    path: impl Into<std::path::PathBuf>,
) -> js::Result<os_path::OsPathBuf> {
    os_path::OsPathBuf::from_path_buf(path.into())
        .map_err(|p| PathError::invalid_path(format!("invalid path: {p:?}")).into_exception(ctx))
}

#[cfg(test)]
mod tests {
    use super::resolve_paths;

    #[test]
    fn resolve_paths_relative() {
        let paths = vec!["foo".to_string(), "bar".to_string()];
        let base = os_path::OsPathBuf::new("base");
        let result = resolve_paths(&paths, base);
        assert_eq!(result.into_string(), "base/foo/bar".replace('/', "\\"));
    }

    #[test]
    fn resolve_paths_absolute_resets() {
        let paths = vec!["foo".to_string(), "/abs".to_string(), "bar".to_string()];
        let base = os_path::OsPathBuf::new("base");
        let result = resolve_paths(&paths, base);
        assert_eq!(result.into_string(), "/abs/bar".replace('/', "\\"));
    }

    #[test]
    fn resolve_paths_skips_empty() {
        let paths = vec!["".to_string(), "foo".to_string(), "".to_string()];
        let base = os_path::OsPathBuf::new("base");
        let result = resolve_paths(&paths, base);
        assert_eq!(result.into_string(), "base/foo".replace('/', "\\"));
    }

    #[test]
    fn resolve_paths_empty_paths() {
        let paths: Vec<String> = vec![];
        let base = os_path::OsPathBuf::new("base");
        let result = resolve_paths(&paths, base);
        assert_eq!(result.into_string(), "base".replace('/', "\\"));
    }
}
