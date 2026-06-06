use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

use futures::{SinkExt, StreamExt, channel::mpsc};
use globset::{Glob, GlobSet, GlobSetBuilder};
use js_core::{
    RsString,
    utils::{JsStringArg, StringArg},
};
use os_path::OsPathBuf;
use rquickjs::{Ctx, Object, Value};
use tokio::fs;

use crate::error::IntoJsResult;
use crate::prelude::*;

#[derive(Default)]
pub struct WalkOptions {
    pub pattern: Option<Arc<GlobSet>>,
    pub exclude: Option<Arc<GlobSet>>,
    pub filter: GlobFilter,
    pub dot: bool,
    pub absolute: bool,
}

impl WalkOptions {
    pub fn from_js(ctx: &Ctx<'_>, options: Option<Object<'_>>) -> js::Result<Self> {
        let Some(opts) = options else {
            return Ok(Self::default());
        };

        let mut result = Self::default();

        if let Some(arr) = opts
            .get::<_, Option<Value>>("ignore")?
            .as_ref()
            .and_then(|i| i.as_array())
        {
            let mut builder = GlobSetBuilder::new();
            let mut has_patterns = false;
            for val in arr.iter::<Value>() {
                let val = val?;
                if let Some(s) = val.as_string() {
                    builder.add(Glob::new(&s.to_string()?).into_js(ctx)?);
                    has_patterns = true;
                }
            }
            if has_patterns {
                result.exclude = Some(Arc::new(builder.build().into_js(ctx)?));
            }
        }

        if let Some(absolute) = opts.get::<_, Option<bool>>("absolute")? {
            result.absolute = absolute;
        }

        if let Some(dot) = opts.get::<_, Option<bool>>("dot")? {
            result.dot = dot;
        }

        if let Some(filter) = opts.get::<_, Option<String>>("filter")? {
            result.filter = GlobFilter::from_js(Some(filter))?;
        }

        Ok(result)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WalkResType {
    Dir,
    File,
}

pub type WalkRes = (WalkResType, OsPathBuf);

#[derive(Default, Clone, Copy)]
pub enum GlobFilter {
    #[default]
    All,
    FilesOnly,
    DirsOnly,
}

impl GlobFilter {
    pub fn from_js(value: Option<String>) -> js::Result<Self> {
        let Some(s) = value else {
            return Ok(Self::All);
        };
        match s.as_str() {
            "all" => Ok(Self::All),
            "files" => Ok(Self::FilesOnly),
            "directories" => Ok(Self::DirsOnly),
            _ => Err(js::Error::new_from_js(
                "string",
                "filter must be \"all\", \"files\", or \"directories\"",
            )),
        }
    }

    pub fn matches(&self, is_dir: bool) -> bool {
        match self {
            Self::All => true,
            Self::FilesOnly => !is_dir,
            Self::DirsOnly => is_dir,
        }
    }
}

#[inline(always)]
fn is_dot_file(name: &std::ffi::OsStr) -> bool {
    name.to_str().is_some_and(|s| s.starts_with('.'))
}

#[inline(always)]
fn should_skip_pattern(path: &Path, pattern: &Option<Arc<GlobSet>>) -> bool {
    pattern.as_ref().is_some_and(|p| !p.is_match(path))
}

#[inline(always)]
fn should_skip_exclude(path: &Path, exclude: &Option<Arc<GlobSet>>) -> bool {
    exclude.as_ref().is_some_and(|e| e.is_match(path))
}

pub async fn walk_dir(
    dir: OsPathBuf,
    opts: Arc<WalkOptions>,
    mut tx: mpsc::Sender<WalkRes>,
) -> js::Result<()> {
    let dir_path: &Path = dir.as_ref();
    let mut entities = fs::read_dir(dir_path).await?;

    while let Some(entity) = entities.next_entry().await? {
        let file_name = entity.file_name();

        if !opts.dot && is_dot_file(&file_name) {
            continue;
        }

        let path = OsPathBuf::from_path_buf(entity.path()).map_err(|_| js::Error::Unknown)?;
        let path_ref: &Path = path.as_ref();

        if should_skip_pattern(path_ref, &opts.pattern) {
            continue;
        }

        if should_skip_exclude(path_ref, &opts.exclude) {
            continue;
        }

        let is_dir = entity.file_type().await?.is_dir();

        if !opts.filter.matches(is_dir) {
            continue;
        }

        if is_dir {
            if tx.send((WalkResType::Dir, path.clone())).await.is_err() {
                return Ok(());
            }
            Box::pin(walk_dir(path, opts.clone(), tx.clone())).await?;
        } else {
            if tx.send((WalkResType::File, path)).await.is_err() {
                return Ok(());
            }
        }
    }

    Ok(())
}

#[js::class]
#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
pub struct WalkIterator<'js> {
    #[qjs(skip_trace)]
    rx: mpsc::Receiver<WalkRes>,
    #[qjs(skip_trace)]
    task: tokio::task::JoinHandle<js::Result<()>>,
    #[qjs(skip_trace)]
    base: OsPathBuf,
    #[qjs(skip_trace)]
    absolute: bool,
    #[qjs(skip_trace)]
    _marker: PhantomData<&'js ()>,
}

#[inline(always)]
pub fn init<'js>(ctx: &Ctx<'js>) -> js::Result<()> {
    rquickjs::Class::<WalkIterator>::define(&ctx.globals())
}

#[inline(always)]
pub fn create_iterator<'js>(
    _ctx: Ctx<'js>,
    dir: OsPathBuf,
    opts: WalkOptions,
) -> js::Result<WalkIterator<'js>> {
    let (tx, rx) = mpsc::channel::<WalkRes>(100);
    let opts_arc = Arc::new(opts);
    let base = dir.clone();
    let absolute = opts_arc.absolute;

    let task = tokio::spawn(async move { walk_dir(dir, opts_arc, tx).await });

    Ok(WalkIterator {
        rx,
        task,
        base,
        absolute,
        _marker: PhantomData,
    })
}

#[js::methods]
impl<'js> WalkIterator<'js> {
    #[qjs(rename = rquickjs::atom::PredefinedAtom::SymbolAsyncIterator)]
    fn async_iterator(&self, ctx: Ctx<'js>) -> js::Result<js::Value<'js>> {
        let func: js::Function = ctx.eval("(function(obj) { return obj; })")?;
        let class_obj = rquickjs::Class::<WalkIterator>::from_value(&ctx.globals().get("this")?)?;
        func.call((class_obj,)).map(|v: js::Value| v)
    }

    pub async fn next(&mut self, ctx: Ctx<'js>) -> js::Result<js::Object<'js>> {
        match self.rx.next().await {
            Some((_, path)) => {
                let result_path = if self.absolute {
                    path
                } else {
                    path.relative_to(&self.base)
                };

                let rs_string = RsString::owned(result_path.to_string());
                let value = js::Class::instance(ctx.clone(), rs_string)?.into_value();

                let obj = js::Object::new(ctx.clone())?;
                obj.set("value", value)?;
                obj.set("done", false)?;
                Ok(obj)
            }
            None => {
                let _ = (&mut self.task).await;
                let obj = js::Object::new(ctx.clone())?;
                obj.set("value", js::Value::new_undefined(ctx.clone()))?;
                obj.set("done", true)?;
                Ok(obj)
            }
        }
    }

    #[qjs(rename = "return")]
    async fn return_method(&mut self, ctx: Ctx<'js>) -> js::Result<js::Object<'js>> {
        self.task.abort();
        let _ = (&mut self.task).await;

        let obj = js::Object::new(ctx.clone())?;
        obj.set("value", js::Value::new_undefined(ctx.clone()))?;
        obj.set("done", true)?;
        Ok(obj)
    }
}

#[js::function]
pub async fn walk<'js>(
    ctx: Ctx<'js>,
    dir: js::Value<'js>,
    options: Option<Object<'js>>,
) -> js::Result<WalkIterator<'js>> {
    let dir_str = StringArg::coerce_js(&ctx, &dir, "dir")?;
    let dir_buf = OsPathBuf::new(dir_str.as_str());
    let opts = WalkOptions::from_js(&ctx, options)?;

    create_iterator(ctx, dir_buf, opts)
}
