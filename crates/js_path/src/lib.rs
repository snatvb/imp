use std::path::Path;

use rquickjs::{function, module::ModuleDef, prelude::Rest};

mod error;

use error::PathError;

#[function]
pub fn resolve(ctx: rquickjs::Ctx<'_>, args: Rest<String>) -> rquickjs::Result<String> {
    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(&ctx))?;

    let mut base = os_path::OsPathBuf::from_path_buf(cwd).map_err(|p| {
        PathError::invalid_path(format!("current directory path is not valid UTF-8: {p:?}"))
            .into_exception(&ctx)
    })?;

    for arg in args.iter() {
        if arg.is_empty() {
            continue;
        }
        if Path::new(arg.as_str()).is_absolute() {
            base = os_path::OsPathBuf::new(arg.as_str());
        } else {
            base.push(arg.as_str());
        }
    }

    Ok(base.into_string())
}

pub struct PathModule;

impl ModuleDef for PathModule {
    fn declare<'js>(decl: &rquickjs::module::Declarations<'js>) -> rquickjs::Result<()> {
        decl.declare("resolve")?;
        Ok(())
    }

    fn evaluate<'js>(
        _ctx: &rquickjs::Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        exports.export("resolve", js_resolve)?;
        Ok(())
    }
}
