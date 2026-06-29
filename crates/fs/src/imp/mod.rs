pub mod dir;
pub mod expand_home;
pub mod file_handle;
pub mod fs_stats;
pub mod glob;
pub mod read;
pub mod walk;
pub mod write;

pub use file_handle::FileHandle;
pub use fs_stats::FsStats;
pub use write::WriteHandle;

js_core::impl_module!(ImpFsModule,
    evaluate: |ctx, exports, export_all| {
        file_handle::init(ctx)?;
        fs_stats::init(ctx)?;
        walk::init(ctx)?;
        write::init(ctx)?;
        let ns = export_all(ctx, exports)?;
        exports.export("default", ns)?;
        Ok(())
    },
    "readFile" => read::js_read_file,
    "writeFile" => write::js_write_file,
    "expandHome" => expand_home::js_expand_home,
    "open" => file_handle::js_open,
    "openWrite" => write::js_open_write,
    "mkdir" => dir::js_mkdir,
    "metadata" => dir::js_metadata,
    "metadataBatch" => dir::js_metadata_batch,
    "remove" => dir::js_remove,
    "removeAll" => dir::js_remove_all,
    "exists" => dir::js_exists,
    "glob" => glob::js_glob,
    "globStream" => glob::js_glob_stream,
    "walk" => walk::js_walk,
    "chmod" => dir::js_chmod,
    "lchmod" => dir::js_lchmod,
    "symlink" => dir::js_symlink,
    "link" => dir::js_link,
);
