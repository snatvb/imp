use std::cell::RefCell;
use std::marker::PhantomData;
use std::path::Path;
use std::rc::Rc;
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
fn matches_pattern(rel_posix: &str, pattern: &Option<Arc<GlobSet>>) -> bool {
    pattern.as_ref().is_none_or(|p| p.is_match(rel_posix))
}

#[inline(always)]
fn matches_exclude(rel_posix: &str, exclude: &Option<Arc<GlobSet>>) -> bool {
    exclude.as_ref().is_none_or(|e| !e.is_match(rel_posix))
}

pub async fn walk_dir(
    dir: OsPathBuf,
    base: OsPathBuf,
    opts: Arc<WalkOptions>,
    mut tx: mpsc::Sender<WalkRes>,
) -> js::Result<()> {
    let dir_path: &Path = dir.as_ref();
    let mut entities = fs::read_dir(dir_path).await?;
    let needs_posix = opts.pattern.is_some() || opts.exclude.is_some();

    while let Some(entity) = entities.next_entry().await? {
        let file_name = entity.file_name();

        if !opts.dot && is_dot_file(&file_name) {
            continue;
        }

        let path = OsPathBuf::from_path_buf(entity.path()).map_err(|_| js::Error::Unknown)?;

        if needs_posix {
            let rel_posix = base.relative_to(&path).to_posix().into_string();

            if !matches_exclude(&rel_posix, &opts.exclude) {
                continue;
            }

            let is_dir = entity.file_type().await?.is_dir();

            if is_dir {
                let emit = opts.filter.matches(true) && matches_pattern(&rel_posix, &opts.pattern);
                if emit && tx.send((WalkResType::Dir, path.clone())).await.is_err() {
                    return Ok(());
                }
                Box::pin(walk_dir(path, base.clone(), opts.clone(), tx.clone())).await?;
            } else {
                let emit = opts.filter.matches(false) && matches_pattern(&rel_posix, &opts.pattern);
                if emit && tx.send((WalkResType::File, path)).await.is_err() {
                    return Ok(());
                }
            }
        } else {
            let is_dir = entity.file_type().await?.is_dir();

            if is_dir {
                if opts.filter.matches(true) && tx.send((WalkResType::Dir, path.clone())).await.is_err() {
                    return Ok(());
                }
                Box::pin(walk_dir(path, base.clone(), opts.clone(), tx.clone())).await?;
            } else {
                if opts.filter.matches(false) && tx.send((WalkResType::File, path)).await.is_err() {
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

struct WalkIteratorInner {
    rx: mpsc::Receiver<WalkRes>,
    task: Option<tokio::task::JoinHandle<js::Result<()>>>,
    base: OsPathBuf,
    absolute: bool,
}

#[js::class]
#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
pub struct WalkIterator<'js> {
    #[qjs(skip_trace)]
    inner: Rc<RefCell<WalkIteratorInner>>,
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
    let base_for_walk = base.clone();

    let task = tokio::spawn(async move { walk_dir(dir, base_for_walk, opts_arc, tx).await });

    Ok(WalkIterator {
        inner: Rc::new(RefCell::new(WalkIteratorInner {
            rx,
            task: Some(task),
            base,
            absolute,
        })),
        _marker: PhantomData,
    })
}

impl Clone for WalkIterator<'_> {
    fn clone(&self) -> Self {
        WalkIterator {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

#[js::methods]
impl<'js> WalkIterator<'js> {
    #[qjs(rename = rquickjs::atom::PredefinedAtom::SymbolAsyncIterator)]
    fn async_iterator(&self, ctx: Ctx<'js>) -> js::Result<js::Class<'js, WalkIterator<'js>>> {
        let cloned = self.clone();
        js::Class::instance(ctx, cloned)
    }

    #[qjs()]
    #[allow(clippy::await_holding_refcell_ref)]
    pub async fn next(&mut self, ctx: Ctx<'js>) -> js::Result<js::Object<'js>> {
        let mut inner = self.inner.borrow_mut();
        match inner.rx.next().await {
            Some((_, path)) => {
                let result_path = if inner.absolute {
                    path
                } else {
                    inner.base.relative_to(&path)
                };

                let rs_string = RsString::owned(result_path.to_string());
                let value = js::Class::instance(ctx.clone(), rs_string)?.into_value();

                let obj = js::Object::new(ctx.clone())?;
                obj.set("value", value)?;
                obj.set("done", false)?;
                Ok(obj)
            }
            None => {
                if let Some(mut task) = inner.task.take() {
                    let _ = (&mut task).await;
                }
                let obj = js::Object::new(ctx.clone())?;
                obj.set("value", js::Value::new_undefined(ctx.clone()))?;
                obj.set("done", true)?;
                Ok(obj)
            }
        }
    }

    #[qjs(rename = "return")]
    #[allow(clippy::await_holding_refcell_ref)]
    async fn return_method(&mut self, ctx: Ctx<'js>) -> js::Result<js::Object<'js>> {
        let mut inner = self.inner.borrow_mut();
        if let Some(mut task) = inner.task.take() {
            task.abort();
            let _ = (&mut task).await;
        }

        let obj = js::Object::new(ctx.clone())?;
        obj.set("value", js::Value::new_undefined(ctx.clone()))?;
        obj.set("done", true)?;
        Ok(obj)
    }
}

#[js::function]
pub fn walk<'js>(
    ctx: Ctx<'js>,
    dir: js::Value<'js>,
    options: js::function::Opt<Object<'js>>,
) -> js::Result<WalkIterator<'js>> {
    let dir_str = StringArg::coerce_js(&ctx, &dir, "dir")?;
    let dir_buf = OsPathBuf::new(dir_str.as_str());
    let opts = WalkOptions::from_js(&ctx, options.into_inner())?;

    create_iterator(ctx, dir_buf, opts)
}
