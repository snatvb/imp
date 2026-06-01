use std::path::Path;

use os_path::OsPath;
pub use rquickjs as js;

use crate::{meta::with_meta, typescript};

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
        let code = match ext {
            Some("ts" | "mts" | "cts") => {
                let source = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                tracing::debug!(bytes = source.len(), "read source");
                typescript::strip_types_fast_default(&source)
                    .map(with_meta(&self.cwd, name))
                    .map_err(|e| js::Error::new_loading_message(name, e))?
            }
            Some("js" | "mjs" | "cjs") => {
                let source = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                tracing::debug!(bytes = source.len(), "read source");
                with_meta(&self.cwd, name)(source)
            }
            _ => return Err(js::Error::new_loading_message(name, "not a script")),
        };

        js::module::Module::declare(ctx.clone(), name, code)
    }
}
