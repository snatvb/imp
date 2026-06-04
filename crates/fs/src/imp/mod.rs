use crate::prelude::*;
use js::IntoJs;
use js::module::ModuleDef;

pub mod file_handle;
pub mod read;

pub use file_handle::FileHandle;

pub struct ImpFsModule;

impl ModuleDef for ImpFsModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("default")?;
        decl.declare("readFile")?;
        decl.declare("open")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        file_handle::init(ctx)?;
        let read_file = read::js_read_file.into_js(ctx)?;
        let open_fn = file_handle::js_open.into_js(ctx)?;
        js_core::modules::export_ns(ctx, exports, &[("readFile", read_file), ("open", open_fn)])
    }
}
