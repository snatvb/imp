use std::sync::Arc;

use globset::{Glob, GlobSetBuilder};
use js_core::utils::StringArg;
use os_path::OsPathBuf;
use rquickjs::{Ctx, Object};

use crate::error::IntoJsResult;
use crate::imp::walk::{WalkIterator, WalkOptions, create_iterator};
use crate::prelude::*;

#[js::function]
pub async fn glob<'js>(
    ctx: Ctx<'js>,
    dir: StringArg,
    pattern: StringArg,
    options: js::function::Opt<Object<'js>>,
) -> js::Result<js::Array<'js>> {
    let dir_buf = OsPathBuf::new(dir.as_str());

    let glob_set = Arc::new(
        GlobSetBuilder::new()
            .add(Glob::new(pattern.as_str()).into_js(&ctx)?)
            .build()
            .into_js(&ctx)?,
    );

    let mut opts = WalkOptions::from_js(&ctx, options.into_inner())?;
    opts.pattern = Some(glob_set);

    let mut iterator = create_iterator(ctx.clone(), dir_buf, opts)?;

    let result = js::Array::new(ctx.clone())?;
    let mut i = 0;

    loop {
        let next_obj = iterator.next(ctx.clone()).await?;
        let done: bool = next_obj.get("done")?;

        if done {
            break;
        }

        let value: js::Value = next_obj.get("value")?;
        result.set(i, value)?;
        i += 1;
    }

    Ok(result)
}

#[js::function(rename = "globStream")]
pub fn glob_stream<'js>(
    ctx: Ctx<'js>,
    dir: StringArg,
    pattern: StringArg,
    options: js::function::Opt<Object<'js>>,
) -> js::Result<WalkIterator<'js>> {
    let dir_buf = OsPathBuf::new(dir.as_str());

    let glob_set = Arc::new(
        GlobSetBuilder::new()
            .add(Glob::new(pattern.as_str()).into_js(&ctx)?)
            .build()
            .into_js(&ctx)?,
    );

    let mut opts = WalkOptions::from_js(&ctx, options.into_inner())?;
    opts.pattern = Some(glob_set);

    create_iterator(ctx, dir_buf, opts)
}
