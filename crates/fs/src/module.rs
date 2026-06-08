use crate::{prelude::*, read};
use js::module::ModuleDef;

pub struct FsPromisesModule;

impl ModuleDef for FsPromisesModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("readFile")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        exports.export(
            "readFile",
            js::Function::new(ctx.clone(), js::function::Async(read::read_file))?,
        )?;
        Ok(())
    }
}
