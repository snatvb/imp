use crate::{prelude::*, read};
use js::module::ModuleDef;

pub struct FsPromisesModule;

impl ModuleDef for FsPromisesModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("default")?;
        decl.declare("readFile")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        let readfile =
            js::Function::new(ctx.clone(), js::function::Async(read::read_file))?;

        exports.export("readFile", readfile.clone())?;

        let ns = js::Object::new(ctx.clone())?;
        ns.set("readFile", readfile)?;
        exports.export("default", ns)?;

        Ok(())
    }
}
