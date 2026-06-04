use crate::prelude::*;
use js::module::ModuleDef;
use js::IntoJs;

pub mod read;

pub struct ImpFsModule;

impl ModuleDef for ImpFsModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("default")?;
        decl.declare("readFile")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        let read_file = read::js_read_file.into_js(ctx)?;
        js_core::modules::export_ns(ctx, exports, &[("readFile", read_file)])
    }
}
