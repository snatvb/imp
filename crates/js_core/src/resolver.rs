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
    fn dir_of(&self, base: &str) -> PathBuf {
        if base == "<eval>" || !base.contains('/') {
            return std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        }
        Path::new(base)
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf()
    }

    fn try_file(&self, path: &Path) -> Option<String> {
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
}

impl js::loader::Resolver for Resolver {
    fn resolve<'js>(
        &mut self,
        _ctx: &js::Ctx<'js>,
        base: &str,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<String> {
        if name.starts_with("./") || name.starts_with("../") {
            let base_dir = self.dir_of(base);
            let path = base_dir.join(name);
            self.try_file(&path)
                .or_else(|| self.try_index(&path))
                .ok_or_else(|| js::Error::new_resolving_message(base, name, "module not found"))
        } else {
            let (pkg, subpath) = self.break_bare(name);
            let mut dir = self.dir_of(base);
            loop {
                let pkg_dir = dir.join("node_modules").join(&pkg);
                if pkg_dir.is_dir() {
                    let sub_path = if subpath == "." {
                        pkg_dir
                    } else {
                        pkg_dir.join(&subpath[2..])
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
            Err(js::Error::new_resolving_message(
                base,
                name,
                "module not found",
            ))
        }
    }
}
