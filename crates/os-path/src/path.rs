use crate::path_buf::OsPathBuf;
use camino::Utf8Component;
use camino::Utf8Path;
use std::fmt;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OsPath(Utf8Path);

impl OsPath {
    #[inline]
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Self {
        let path = Path::new(s.as_ref());
        // SAFETY: s is valid UTF-8 str, Path is just [u8], OsPath is transparent over Utf8Path
        unsafe { &*(path as *const Path as *const Self) }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline]
    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    #[inline]
    pub fn parent(&self) -> Option<&OsPath> {
        self.0.parent().map(|p| {
            // SAFETY: same repr
            unsafe { &*(p as *const Utf8Path as *const OsPath) }
        })
    }

    #[inline]
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name()
    }

    #[inline]
    pub fn extension(&self) -> Option<&str> {
        self.0.extension()
    }

    #[inline]
    pub fn root(&self) -> &str {
        let s = self.0.as_str();
        let mut end = 0;
        for comp in self.0.components() {
            match comp {
                Utf8Component::Prefix(p) => end = p.as_str().len(),
                Utf8Component::RootDir => end += 1,
                _ => break,
            }
        }
        &s[..end]
    }

    pub fn root_and_normal_parts(&self) -> (&str, Vec<&str>) {
        let s = self.0.as_str();
        let mut end = 0;
        let mut parts = Vec::new();
        for comp in self.0.components() {
            match comp {
                Utf8Component::Prefix(p) => end = p.as_str().len(),
                Utf8Component::RootDir => end += 1,
                Utf8Component::Normal(n) => parts.push(n),
                _ => {}
            }
        }
        (&s[..end], parts)
    }

    #[inline]
    pub fn to_os_path_buf(&self) -> OsPathBuf {
        OsPathBuf(self.0.to_path_buf())
    }
}

impl ToOwned for OsPath {
    type Owned = OsPathBuf;
    #[inline]
    fn to_owned(&self) -> OsPathBuf {
        self.to_os_path_buf()
    }
}

impl AsRef<str> for OsPath {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<Utf8Path> for OsPath {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<Path> for OsPath {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl fmt::Display for OsPath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
