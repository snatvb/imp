use os_path::{OsPath, OsPathBuf};

use crate::utils::JsString;

pub struct Meta<'a> {
    filepath: &'a str,
    cwd: &'a OsPathBuf,
}

impl<'a> Meta<'a> {
    pub fn new(cwd: &'a OsPathBuf, filepath: &'a str) -> Self {
        Self { filepath, cwd }
    }

    pub fn get_filename(&self) -> Option<OsPathBuf> {
        OsPath::new(self.filepath)
            .file_name()
            .map(OsPathBuf::from)
    }

    pub fn filename(&self) -> String {
        let mut cwd = self.cwd.clone();
        cwd.push(self.filepath);
        cwd.into_string()
    }

    pub fn dir(&self) -> OsPathBuf {
        self.cwd.join(
            OsPath::new(self.filepath)
                .parent()
                .unwrap_or(OsPath::new(".")),
        )
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn into_define(&self) -> String {
        let dirname = self.dir().js_string();
        let filepath = self.filename().js_string();
        tracing::debug!(?dirname, ?filepath, "Meta::into_define");
        format!(
            r#"
import.meta.filename="{filepath}";
import.meta.dirname="{dirname}";
        "#
        )
    }
}

pub fn with_meta<'a>(
    cwd: &'a OsPathBuf,
    filepath: &'a str,
) -> impl FnOnce(String) -> String + 'a {
    let meta = Meta::new(cwd, filepath).into_define();
    move |code: String| format!("{meta}\n{code}")
}
