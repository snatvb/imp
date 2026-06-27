use std::collections::{HashMap, HashSet};
use std::path::Path;

use serde_json;

use os_path::OsPath;
pub use rquickjs as js;

use crate::{
    meta::set_meta,
    resolver::{MODULE_EXTENSIONS, normalize_relative_path, try_with_extensions},
    typescript,
};

pub struct TsLoader;

impl js::loader::Loader for TsLoader {
    #[tracing::instrument(level = "debug", skip(self, ctx, _attrs), fields(name = %name))]
    fn load<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<js::module::Module<'js, js::module::Declared>> {
        let path = Path::new(name);

        let ext = path.extension().and_then(|e| e.to_str());
        tracing::debug!(?ext, "TsLoader::load");
        match ext {
            Some("ts" | "mts" | "cts") => {
                let source = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                tracing::debug!(bytes = source.len(), "read source");
                let code = typescript::strip_types_fast_default(&source)
                    .map_err(|e| js::Error::new_loading_message(name, e))?;
                tracing::info!(file = name, "TsLoader declared module");
                js::module::Module::declare(ctx.clone(), name, code)
            }
            _ => Err(js::Error::new_loading_message(name, "not ts")),
        }
    }
}

pub struct DataLoader;

impl js::loader::Loader for DataLoader {
    fn load<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        name: &str,
        attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<js::module::Module<'js, js::module::Declared>> {
        let path = std::path::Path::new(name);
        let ext = path.extension().and_then(|e| e.to_str());

        let module_type = attrs
            .as_ref()
            .and_then(|a| a.get_type().ok())
            .flatten()
            .or_else(|| ext.map(|e| e.to_string()));

        match module_type.as_deref() {
            Some("json") => {
                let raw = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                serde_json::from_str::<serde_json::Value>(&raw)
                    .map_err(|e| js::Error::new_loading_message(name, e.to_string()))?;
                js::module::Module::declare(ctx.clone(), name, format!("export default {raw};"))
            }
            Some("text" | "txt") => {
                let raw = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                let json_str = serde_json::to_string(&raw)
                    .map_err(|e| js::Error::new_loading_message(name, e.to_string()))?;
                js::module::Module::declare(
                    ctx.clone(),
                    name,
                    format!("export default {json_str};"),
                )
            }
            _ => Err(js::Error::new_loading_message(
                name,
                "unsupported data type",
            )),
        }
    }
}

pub struct ScriptLoader {
    pub cwd: os_path::OsPathBuf,
}

impl js::loader::Loader for ScriptLoader {
    #[tracing::instrument(level = "debug", skip(self, ctx, _attrs), fields(name = %name))]
    fn load<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<js::module::Module<'js, js::module::Declared>> {
        let path = OsPath::new(name);
        let ext = path.extension();
        tracing::debug!(?ext, path = %name, "ScriptLoader::load");
        let source = match ext {
            Some("ts" | "mts" | "cts") => {
                let source = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                tracing::debug!(bytes = source.len(), "read source");
                typescript::strip_types_fast_default(&source)
                    .map_err(|e| js::Error::new_loading_message(name, e))?
            }
            Some("js" | "mjs" | "cjs") => {
                let source = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                tracing::debug!(bytes = source.len(), "read source");
                source
            }
            _ => return Err(js::Error::new_loading_message(name, "not a script")),
        };

        let module = js::module::Module::declare(ctx.clone(), name, source)?;
        set_meta(ctx, &module, &self.cwd, name)?;
        Ok(module)
    }
}

pub struct EmbeddedResolver {
    modules: HashSet<String>,
}

impl EmbeddedResolver {
    pub fn new(module_names: impl Iterator<Item = String>) -> Self {
        Self {
            modules: module_names.collect(),
        }
    }
}

impl js::loader::Resolver for EmbeddedResolver {
    fn resolve<'js>(
        &mut self,
        _ctx: &js::Ctx<'js>,
        base: &str,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<String> {
        if name.starts_with("./") || name.starts_with("../") {
            let target_str = normalize_relative_path(base, name);
            let checker = |p: &str| self.modules.contains(p);
            try_with_extensions(&target_str, MODULE_EXTENSIONS, checker).ok_or_else(|| {
                js::Error::new_resolving_message(base, name, "module not found in bundle")
            })
        } else {
            if self.modules.contains(name) {
                return Ok(name.to_string());
            }
            Err(js::Error::new_resolving_message(
                base,
                name,
                "module not found in bundle",
            ))
        }
    }
}

pub struct EmbeddedLoader {
    modules: HashMap<String, String>,
    original_paths: HashMap<String, String>,
    cwd: os_path::OsPathBuf,
}

impl EmbeddedLoader {
    pub fn new(
        modules: HashMap<String, String>,
        original_paths: HashMap<String, String>,
        cwd: os_path::OsPathBuf,
    ) -> Self {
        Self {
            modules,
            original_paths,
            cwd,
        }
    }
}

impl js::loader::Loader for EmbeddedLoader {
    fn load<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<js::module::Module<'js, js::module::Declared>> {
        let code = self
            .modules
            .get(name)
            .ok_or_else(|| js::Error::new_loading_message(name, "module not found in bundle"))?;
        let module = js::module::Module::declare(ctx.clone(), name, code.clone())?;
        let original = self
            .original_paths
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or(name);
        set_meta(ctx, &module, &self.cwd, original)?;
        Ok(module)
    }
}
