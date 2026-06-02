use crate::normalize::os_normalize;
use crate::path::OsPath;
use camino::Utf8Component;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::path::{MAIN_SEPARATOR_STR as SEPARATOR, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsPathBuf(pub(crate) Utf8PathBuf);

impl OsPathBuf {
    #[inline]
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(Utf8PathBuf::from(os_normalize(s.as_ref()).as_ref()))
    }

    pub fn from_path_buf(p: PathBuf) -> Result<Self, PathBuf> {
        Utf8PathBuf::from_path_buf(p)
            .map(|u| Self(Utf8PathBuf::from(os_normalize(u.as_str()).as_ref())))
    }

    #[inline]
    pub fn push(&mut self, s: impl AsRef<str>) {
        self.0.push(os_normalize(s.as_ref()).as_ref());
    }

    #[inline]
    pub fn join(&self, s: impl AsRef<str>) -> Self {
        Self(self.0.join(os_normalize(s.as_ref()).as_ref()))
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    pub fn normalize(&self) -> Self {
        let s = self.0.as_str();
        if s.is_empty() {
            return Self(Utf8PathBuf::from("."));
        }

        let trailing = s.ends_with('/') || s.ends_with('\\');

        let mut prefix = None;
        let mut rooted = false;
        let mut stack: Vec<&str> = Vec::new();

        for c in self.0.components() {
            match c {
                Utf8Component::Prefix(p) => {
                    prefix = Some(p.as_str());
                    stack.clear();
                }
                Utf8Component::RootDir => {
                    rooted = true;
                }
                Utf8Component::CurDir => {}
                Utf8Component::ParentDir => {
                    if stack.is_empty() && rooted {
                        continue;
                    }
                    if stack.last() == Some(&"..") {
                        stack.push("..");
                    } else if !stack.is_empty() {
                        stack.pop();
                    } else {
                        stack.push("..");
                    }
                }
                Utf8Component::Normal(n) => stack.push(n),
            }
        }

        let body = stack.join(SEPARATOR);

        let mut result = match (prefix, rooted, body.is_empty()) {
            (Some(p), true, true) => format!("{p}{SEPARATOR}"),
            (Some(p), true, false) => format!("{p}{SEPARATOR}{body}"),
            (Some(p), false, _) => format!("{p}{body}"),
            (None, true, _) => format!("{SEPARATOR}{body}"),
            (None, false, true) => ".".into(),
            (None, false, false) => body,
        };

        if trailing && result != "." && result != ".." && !result.ends_with(SEPARATOR) {
            result.push_str(SEPARATOR);
        }

        Self(Utf8PathBuf::from(result))
    }

    pub fn resolve(&self, arg: &str) -> Self {
        if arg.is_empty() {
            return self.clone();
        }
        if Utf8Path::new(arg).is_absolute() {
            Self::new(arg).normalize()
        } else {
            self.join(arg).normalize()
        }
    }

    pub fn relative_to(&self, other: &Self) -> Self {
        let a = self.normalize();
        let b = other.normalize();
        if a == b {
            return Self::new("");
        }

        let (a_root, a_parts) = a.root_and_normal_parts();
        let (b_root, b_parts) = b.root_and_normal_parts();

        if a_root != b_root {
            return b;
        }

        let common = a_parts
            .iter()
            .zip(&b_parts)
            .take_while(|(x, y)| x == y)
            .count();

        let rel: Vec<&str> = (0..a_parts.len() - common)
            .map(|_| "..")
            .chain(b_parts[common..].iter().copied())
            .collect();

        Self::new(rel.join(SEPARATOR))
    }

    #[inline]
    pub fn as_path(&self) -> &OsPath {
        let u: &Utf8Path = self.0.as_ref();
        // SAFETY: OsPath is #[repr(transparent)] over Utf8Path
        unsafe { &*(u as *const Utf8Path as *const OsPath) }
    }
}

impl Deref for OsPathBuf {
    type Target = OsPath;
    #[inline]
    fn deref(&self) -> &OsPath {
        self.as_path()
    }
}

impl Borrow<OsPath> for OsPathBuf {
    #[inline]
    fn borrow(&self) -> &OsPath {
        self.as_path()
    }
}

impl AsRef<str> for OsPathBuf {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<Utf8Path> for OsPathBuf {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<OsPath> for OsPathBuf {
    #[inline]
    fn as_ref(&self) -> &OsPath {
        self.as_path()
    }
}

impl AsRef<Path> for OsPathBuf {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl fmt::Display for OsPathBuf {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for OsPathBuf {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for OsPathBuf {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<Utf8PathBuf> for OsPathBuf {
    #[inline]
    fn from(p: Utf8PathBuf) -> Self {
        Self(Utf8PathBuf::from(os_normalize(p.as_str()).as_ref()))
    }
}
