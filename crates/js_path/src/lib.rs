use std::path::Path;

use rquickjs::{function, module::ModuleDef, prelude::Rest, Object};

use js_core::coerce::JsCoerce;

mod error;

use error::PathError;

#[function]
pub fn resolve<'js>(ctx: rquickjs::Ctx<'js>, args: Rest<rquickjs::Value<'js>>) -> rquickjs::Result<String> {
    let paths: Vec<String> = args
        .iter()
        .enumerate()
        .map(|(i, v)| String::coerce_js(&ctx, v, &format!("paths[{i}]")))

        .collect::<rquickjs::Result<Vec<_>>>()?;

    let cwd = std::env::current_dir()
        .map_err(|e| PathError::from_io(e, "resolve").into_exception(&ctx))?;

    let mut base = os_path::OsPathBuf::from_path_buf(cwd).map_err(|p| {
        PathError::invalid_path(format!("current directory path is not valid UTF-8: {p:?}"))
            .into_exception(&ctx)
    })?;

    for arg in &paths {
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
        decl.declare("default")?;
        decl.declare("resolve")?;
        Ok(())
    }

    fn evaluate<'js>(
        ctx: &rquickjs::Ctx<'js>,
        exports: &rquickjs::module::Exports<'js>,
    ) -> rquickjs::Result<()> {
        exports.export("resolve", js_resolve)?;

        let ns = Object::new(ctx.clone())?;
        ns.set("resolve", js_resolve)?;
        exports.export("default", ns)?;

        Ok(())
    }
}
