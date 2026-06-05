use js_core::js;
use rquickjs::{Class, Ctx, JsLifetime, Object, class::Trace, function::Constructor};

#[derive(Trace, JsLifetime)]
#[rquickjs::class]
pub struct FsStats {
    #[qjs(skip_trace)]
    meta: std::fs::Metadata,
}

pub fn init<'js>(ctx: &Ctx<'js>) -> js::Result<()> {
    Class::<FsStats>::define(&ctx.globals())
}

impl FsStats {
    pub fn new(meta: std::fs::Metadata) -> Self {
        Self { meta }
    }
}

fn create_date<'js>(ctx: &Ctx<'js>, ms: f64) -> js::Result<Object<'js>> {
    let date_ctor: Constructor = ctx.globals().get("Date")?;
    date_ctor.construct((ms,))
}

#[rquickjs::methods]
impl<'js> FsStats {
    #[qjs(get, rename = "isFile")]
    fn is_file(&self) -> bool {
        self.meta.is_file()
    }

    #[qjs(get, rename = "isDirectory")]
    fn is_directory(&self) -> bool {
        self.meta.is_dir()
    }

    #[qjs(get, rename = "isSymbolicLink")]
    fn is_symbolic_link(&self) -> bool {
        self.meta.is_symlink()
    }

    #[qjs(get, rename = "isBlockDevice")]
    fn is_block_device(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::FileTypeExt;
            self.meta.file_type().is_block_device()
        }
        #[cfg(not(unix))]
        {
            false
        }
    }

    #[qjs(get, rename = "isCharacterDevice")]
    fn is_character_device(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::FileTypeExt;
            self.meta.file_type().is_char_device()
        }
        #[cfg(not(unix))]
        {
            false
        }
    }

    #[qjs(get, rename = "isFIFO")]
    fn is_fifo(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::FileTypeExt;
            self.meta.file_type().is_fifo()
        }
        #[cfg(not(unix))]
        {
            false
        }
    }

    #[qjs(get, rename = "isSocket")]
    fn is_socket(&self) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::FileTypeExt;
            self.meta.file_type().is_socket()
        }
        #[cfg(not(unix))]
        {
            false
        }
    }

    #[qjs(get)]
    fn size(&self) -> u64 {
        self.meta.len()
    }

    #[qjs(get, rename = "atimeMs")]
    fn atime_ms(&self) -> f64 {
        self.meta
            .accessed()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(f64::NAN)
    }

    #[qjs(get, rename = "mtimeMs")]
    fn mtime_ms(&self) -> f64 {
        self.meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(f64::NAN)
    }

    #[qjs(get, rename = "ctimeMs")]
    fn ctime_ms(&self) -> f64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let secs = self.meta.ctime() as f64;
            let nsecs = self.meta.ctime_nsec() as f64;
            secs * 1000.0 + nsecs / 1_000_000.0
        }
        #[cfg(not(unix))]
        {
            self.mtime_ms()
        }
    }

    #[qjs(get, rename = "birthtimeMs")]
    fn birthtime_ms(&self) -> f64 {
        self.meta
            .created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(f64::NAN)
    }

    #[qjs(get)]
    fn atime(&self, ctx: Ctx<'js>) -> js::Result<Object<'js>> {
        create_date(&ctx, self.atime_ms())
    }

    #[qjs(get)]
    fn mtime(&self, ctx: Ctx<'js>) -> js::Result<Object<'js>> {
        create_date(&ctx, self.mtime_ms())
    }

    #[qjs(get)]
    fn ctime(&self, ctx: Ctx<'js>) -> js::Result<Object<'js>> {
        create_date(&ctx, self.ctime_ms())
    }

    #[qjs(get)]
    fn birthtime(&self, ctx: Ctx<'js>) -> js::Result<Object<'js>> {
        create_date(&ctx, self.birthtime_ms())
    }

    #[qjs(get)]
    fn mode(&self) -> u32 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            self.meta.permissions().mode()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn uid(&self) -> u32 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.uid()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn gid(&self) -> u32 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.gid()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn ino(&self) -> u64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.ino()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn nlink(&self) -> u64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.nlink()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn rdev(&self) -> u64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.rdev()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn blksize(&self) -> u64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.blksize() as u64
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn blocks(&self) -> u64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.blocks()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn dev(&self) -> u64 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            self.meta.dev()
        }
        #[cfg(not(unix))]
        {
            0
        }
    }

    #[qjs(get)]
    fn readonly(&self) -> bool {
        self.meta.permissions().readonly()
    }

    #[qjs(get)]
    fn archive(&self) -> bool {
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x20;
            self.meta.file_attributes() & FILE_ATTRIBUTE_ARCHIVE != 0
        }
        #[cfg(not(windows))]
        {
            false
        }
    }

    #[qjs(get)]
    fn hidden(&self) -> bool {
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
            self.meta.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0
        }
        #[cfg(not(windows))]
        {
            false
        }
    }

    #[qjs(get)]
    fn system(&self) -> bool {
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;
            self.meta.file_attributes() & FILE_ATTRIBUTE_SYSTEM != 0
        }
        #[cfg(not(windows))]
        {
            false
        }
    }
}
