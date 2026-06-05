use crate::prelude::*;
use js::IntoJs;
use js::module::ModuleDef;

pub mod dir;
pub mod file_handle;
pub mod fs_stats;
pub mod read;
pub mod write;

pub use file_handle::FileHandle;
pub use fs_stats::FsStats;

pub struct ImpFsModule;

impl ModuleDef for ImpFsModule {
    fn declare<'js>(decl: &js::module::Declarations<'js>) -> js::Result<()> {
        decl.declare("default")?;
        decl.declare("readFile")?;
        decl.declare("open")?;
        decl.declare("mkdir")?;
        decl.declare("metadata")?;
        decl.declare("metadataBatch")?;
        decl.declare("exists")?;
        decl.declare("remove")?;
        decl.declare("removeAll")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &js::Ctx<'js>, exports: &js::module::Exports<'js>) -> js::Result<()> {
        file_handle::init(ctx)?;
        fs_stats::init(ctx)?;
        let read_file = read::js_read_file.into_js(ctx)?;
        let open_fn = file_handle::js_open.into_js(ctx)?;
        let mkdir = dir::js_mkdir.into_js(ctx)?;
        let metadata = dir::js_metadata.into_js(ctx)?;
        let metadata_batch = dir::js_metadata_batch.into_js(ctx)?;
        let remove = dir::js_remove.into_js(ctx)?;
        let remove_all = dir::js_remove_all.into_js(ctx)?;
        let exists = dir::js_exists.into_js(ctx)?;
        js_core::modules::export_ns(
            ctx,
            exports,
            &[
                ("readFile", read_file),
                ("open", open_fn),
                ("mkdir", mkdir),
                ("metadata", metadata),
                ("metadataBatch", metadata_batch),
                ("remove", remove),
                ("removeAll", remove_all),
                ("exists", exists),
            ],
        )
    }
}
