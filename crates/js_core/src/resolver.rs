use std::path::{Path, PathBuf};

pub use rquickjs as js;

pub struct Resolver {
    pub extensions: Vec<&'static str>,
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            extensions: vec![".ts", ".mts", ".js", ".mjs", ".json"],
        }
    }
}

impl Resolver {
    pub fn resolve_entry(&self, path: &Path) -> Option<String> {
        if path.is_dir() {
            self.try_index(path)
        } else if path.is_file() {
            Some(path.to_string_lossy().to_string())
        } else {
            self.try_file(path)
        }
    }

    pub fn dir_of(&self, base: &str) -> PathBuf {
        if base == "<eval>" || !base.contains('/') {
            return std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        }
        Path::new(base)
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf()
    }

    pub fn try_file(&self, path: &Path) -> Option<String> {
        for ext in &self.extensions {
            let candidate = if ext.starts_with('.') {
                let mut p = path.to_path_buf();
                p.set_extension(ext.trim_start_matches('.'));
                p
            } else {
                path.with_extension(ext)
            };
            if candidate.is_file() {
                return Some(candidate.to_string_lossy().to_string());
            }
        }
        None
    }

    fn try_index(&self, dir: &Path) -> Option<String> {
        for ext in &self.extensions {
            let candidate = dir
                .join("index")
                .with_extension(ext.trim_start_matches('.'));
            if candidate.is_file() {
                return Some(candidate.to_string_lossy().to_string());
            }
        }
        None
    }

    fn break_bare(&self, name: &str) -> (String, String) {
        if name.starts_with('@') {
            // @scope/pkg/sub/path
            let mut parts = name.splitn(3, '/');
            let scope = parts.next().unwrap();
            let pkg = parts.next().unwrap_or("");

            if pkg.is_empty() {
                return (name.to_string(), ".".to_string());
            }

            let sub = parts.next().unwrap_or("");
            let pkg_name = format!("{scope}/{pkg}");
            let subpath = if sub.is_empty() {
                "."
            } else {
                &format!("./{sub}")
            };
            (pkg_name.to_string(), subpath.to_string())
        } else {
            let mut parts = name.splitn(2, '/');
            let pkg = parts.next().unwrap();
            let sub = parts.next().unwrap_or("");
            let subpath = if sub.is_empty() {
                "."
            } else {
                &format!("./{sub}")
            };
            (pkg.to_string(), subpath.to_string())
        }
    }

    fn resolve_impl(&self, base: &str, name: &str) -> Result<String, String> {
        if name.starts_with("./") || name.starts_with("../") {
            let base_dir = self.dir_of(base);
            let path = self.push_rel(&base_dir, name);
            self.try_file(&path)
                .or_else(|| self.try_index(&path))
                .ok_or_else(|| "module not found".to_string())
        } else {
            let (pkg, subpath) = self.break_bare(name);
            let mut dir = self.dir_of(base);
            loop {
                let pkg_dir = self.push_rel(&dir.join("node_modules"), &pkg);
                if pkg_dir.is_dir() {
                    let sub_path = if subpath == "." {
                        pkg_dir
                    } else {
                        self.push_rel(&pkg_dir, &subpath)
                    };
                    if let Some(found) = self
                        .try_file(&sub_path)
                        .or_else(|| self.try_index(&sub_path))
                    {
                        return Ok(found);
                    }
                }
                match dir.parent() {
                    Some(parent) => dir = parent.to_path_buf(),
                    None => break,
                }
            }
            Err("module not found".to_string())
        }
    }

    fn push_rel(&self, base: &Path, rel: &str) -> PathBuf {
        let mut p = base.to_path_buf();
        for segment in rel.split('/') {
            match segment {
                "." => {}
                ".." => {
                    p.pop();
                }
                seg => p.push(seg),
            }
        }
        p
    }
}

impl js::loader::Resolver for Resolver {
    fn resolve<'js>(
        &mut self,
        _ctx: &js::Ctx<'js>,
        base: &str,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<String> {
        self.resolve_impl(base, name)
            .map_err(|e| js::Error::new_resolving_message(base, name, &e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_base() -> String {
        let dir = env!("CARGO_MANIFEST_DIR");
        format!("{dir}/tests/resolver_fixture/main.ts")
    }

    #[test]
    fn break_bare_plain() {
        let r = Resolver::default();
        let (pkg, sub) = r.break_bare("lodash");
        assert_eq!(pkg, "lodash");
        assert_eq!(sub, ".");
    }

    #[test]
    fn break_bare_sub() {
        let r = Resolver::default();
        let (pkg, sub) = r.break_bare("lodash/map");
        assert_eq!(pkg, "lodash");
        assert_eq!(sub, "./map");
    }

    #[test]
    fn break_bare_deep() {
        let r = Resolver::default();
        let (pkg, sub) = r.break_bare("lodash/map/merge/deep");
        assert_eq!(pkg, "lodash");
        assert_eq!(sub, "./map/merge/deep");
    }

    #[test]
    fn break_bare_scope() {
        let r = Resolver::default();
        let (pkg, sub) = r.break_bare("@scope/pkg/sub");
        assert_eq!(pkg, "@scope/pkg");
        assert_eq!(sub, "./sub");
    }

    #[test]
    fn break_bare_scope_only() {
        let r = Resolver::default();
        let (pkg, sub) = r.break_bare("@scope");
        assert_eq!(pkg, "@scope");
        assert_eq!(sub, ".");
    }

    fn n(path: &str) -> String {
        path.replace('\\', "/")
    }

    #[test]
    fn resolve_relative_file() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./helper").unwrap();
        assert!(n(&result).ends_with("helper.ts"), "got: {result}");
    }

    #[test]
    fn resolve_relative_dir() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./dir").unwrap();
        assert!(n(&result).ends_with("dir/index.ts"), "got: {result}");
    }

    #[test]
    fn resolve_bare_logger() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "logger").unwrap();
        assert!(n(&result).ends_with("logger/index.ts"), "got: {result}");
    }

    #[test]
    fn resolve_bare_logger_sub() {
        let r = Resolver::default();
        let result = r
            .resolve_impl(&fixture_base(), "logger/lib/format")
            .unwrap();
        assert!(
            n(&result).ends_with("logger/lib/format.js"),
            "got: {result}"
        );
    }

    #[test]
    fn resolve_bare_scope() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "@scope/pkg").unwrap();
        assert!(n(&result).ends_with("@scope/pkg/index.js"), "got: {result}");
    }

    #[test]
    fn resolve_json_direct() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./data").unwrap();
        assert!(n(&result).ends_with("data.json"), "got: {result}");
    }

    #[test]
    fn resolve_mjs_direct() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./config").unwrap();
        assert!(n(&result).ends_with("config.mjs"), "got: {result}");
    }

    #[test]
    fn resolve_mts_direct() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./module").unwrap();
        assert!(n(&result).ends_with("module.mts"), "got: {result}");
    }

    #[test]
    fn resolve_extension_order_ts_over_js() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./foo").unwrap();
        assert!(n(&result).ends_with("foo.ts"), "got: {result}");
    }

    #[test]
    fn resolve_dir_index_mjs() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./esm").unwrap();
        assert!(n(&result).ends_with("esm/index.mjs"), "got: {result}");
    }

    #[test]
    fn resolve_dir_index_mts() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./types").unwrap();
        assert!(n(&result).ends_with("types/index.mts"), "got: {result}");
    }

    #[test]
    fn resolve_bare_not_found() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "nonexistent");
        assert!(result.is_err(), "expected error, got: {result:?}");
    }

    #[test]
    fn resolve_relative_not_found() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./missing");
        assert!(result.is_err(), "expected error, got: {result:?}");
    }

    #[test]
    fn resolve_dir_without_index() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./deep");
        assert!(result.is_err(), "expected error, got: {result:?}");
    }

    #[test]
    fn resolve_bare_subpath_missing() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "logger/missing");
        assert!(result.is_err(), "expected error, got: {result:?}");
    }

    #[test]
    fn resolve_bare_scope_no_pkg() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "@scope");
        assert!(result.is_err(), "expected error, got: {result:?}");
    }

    #[test]
    fn resolve_empty_name() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "");
        assert!(result.is_err(), "expected error, got: {result:?}");
    }

    #[test]
    fn resolve_error_message() {
        let r = Resolver::default();
        let result = r.resolve_impl(&fixture_base(), "./missing");
        assert_eq!(result.unwrap_err(), "module not found");
    }
}
