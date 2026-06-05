use js_core::{
    error::SystemError,
    utils::{JsStringArg, StringArg},
};
use rquickjs::Class;
use tokio::fs;

use crate::prelude::*;

use super::fs_stats::FsStats;

#[js::function]
pub async fn mkdir<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<()> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    fs::create_dir_all(path_str).await.map_err(|e| {
        SystemError::from_io(e, "mkdir", Some(path_str.to_string())).into_exception(&ctx)
    })?;
    Ok(())
}

#[js::function]
pub async fn exists<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<bool> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    fs::try_exists(path_str).await.map_err(|e| {
        SystemError::from_io(e, "exists", Some(path_str.to_string())).into_exception(&ctx)
    })
}

#[inline(always)]
async fn read_metadata<R>(
    ctx: &js::Ctx<'_>,
    path: &str,
    f: impl Fn(std::fs::Metadata) -> R,
) -> js::Result<R> {
    fs::symlink_metadata(path)
        .await
        .map_err(|e| SystemError::from_io(e, "exists", Some(path.to_string())).into_exception(ctx))
        .map(f)
}

#[js::function]
pub async fn is_dir<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<bool> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    read_metadata(&ctx, path_str, |p| p.is_dir()).await
}

#[js::function]
pub async fn is_file<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<bool> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    read_metadata(&ctx, path_str, |p| p.is_file()).await
}

#[js::function]
pub async fn metadata<'js>(
    ctx: js::Ctx<'js>,
    path: js::Value<'js>,
) -> js::Result<Class<'js, FsStats>> {
    let path_arg = StringArg::coerce_js(&ctx, &path, "path")?;
    let path_str = path_arg.as_str();
    let meta = fs::symlink_metadata(path_str).await.map_err(|e| {
        SystemError::from_io(e, "metadata", Some(path_str.to_string())).into_exception(&ctx)
    })?;
    Class::instance(ctx, FsStats::new(meta))
}

#[js::function(rename = "metadataBatch")]
pub async fn metadata_batch<'js>(
    ctx: js::Ctx<'js>,
    paths: js::Array<'js>,
) -> js::Result<js::Array<'js>> {
    let path_strs: Vec<String> = paths
        .iter::<js::Value>()
        .map(|path_val| {
            let path_val = path_val?;
            let path_arg = StringArg::coerce_js(&ctx, &path_val, "path")?;
            Ok::<_, js::Error>(path_arg.as_str().to_string())
        })
        .collect::<js::Result<_>>()?;

    let futures: Vec<_> = path_strs.iter().map(fs::symlink_metadata).collect();

    let results = futures::future::join_all(futures).await;

    let result = js::Array::new(ctx.clone())?;
    for (i, meta_result) in results.into_iter().enumerate() {
        let meta = meta_result.map_err(|e| {
            SystemError::from_io(e, "metadataBatch", Some(path_strs[i].clone()))
                .into_exception(&ctx)
        })?;
        let stats = Class::instance(ctx.clone(), FsStats::new(meta))?;
        result.set(i, stats)?;
    }
    Ok(result)
}
