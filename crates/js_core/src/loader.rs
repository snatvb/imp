pub use rquickjs as js;

use crate::typescript;

pub struct TsLoader;

impl js::loader::Loader for TsLoader {
    fn load<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        name: &str,
        _attrs: Option<js::loader::ImportAttributes<'js>>,
    ) -> js::Result<js::module::Module<'js, js::module::Declared>> {
        let path = std::path::Path::new(name);

        let ext = path.extension().and_then(|e| e.to_str());
        match ext {
            Some("ts" | "mts" | "cts") => {
                let source = std::fs::read_to_string(path)
                    .map_err(|_| js::Error::new_loading_message(name, "file not found"))?;
                let code = typescript::strip_types_fast_default(&source)
                    .map_err(|e| js::Error::new_loading_message(name, e))?;
                js::module::Module::declare(ctx.clone(), name, code)
            }
            _ => Err(js::Error::new_loading_message(name, "not ts")),
        }
    }
}
