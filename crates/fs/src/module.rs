use crate::{prelude::*, read};
use js::IntoJs;
use js::module::ModuleDef;

pub struct FsPromisesModule;

impl ModuleDef for FsPromisesModule {
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
