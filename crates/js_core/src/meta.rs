use os_path::{OsPath, OsPathBuf};

pub struct Meta<'a> {
    filepath: &'a str,
    cwd: &'a OsPathBuf,
}

impl<'a> Meta<'a> {
    pub fn new(cwd: &'a OsPathBuf, filepath: &'a str) -> Self {
        Self { filepath, cwd }
    }

    pub fn get_filename(&self) -> Option<OsPathBuf> {
        OsPath::new(self.filepath).file_name().map(OsPathBuf::from)
    }

    pub fn filename(&self) -> String {
        let mut cwd = self.cwd.clone();
        cwd.push(self.filepath);
        cwd.into_string()
    }

    pub fn dir(&self) -> OsPathBuf {
        let mut full = self.cwd.clone();
        full.push(self.filepath);
        OsPath::new(full.as_str())
            .parent()
            .map(|p| OsPathBuf::from(p.as_str()))
            .unwrap_or_else(|| OsPathBuf::from("."))
    }

    pub fn url(&self) -> String {
        let path = self.filename().replace('\\', "/");
        format!("file:///{path}")
    }
}

pub fn set_meta(
    ctx: &rquickjs::Ctx<'_>,
    module: &rquickjs::module::Module<'_>,
    cwd: &OsPathBuf,
    filepath: &str,
) -> rquickjs::Result<()> {
    let meta = Meta::new(cwd, filepath);
    ffi_extra::module_meta::set_module_meta(
        ctx,
        module,
        &meta.filename(),
        meta.dir().as_path().as_str(),
        &meta.url(),
    )
}
