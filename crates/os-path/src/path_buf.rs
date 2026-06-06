use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use crate::backend::{Component, PathBackend, Posix, Win32};
use crate::path::PlatformPath;

pub struct PlatformPathBuf<B: PathBackend>(
    pub(crate) String,
    pub(crate) std::marker::PhantomData<B>,
);

impl<B: PathBackend> PlatformPathBuf<B> {
    pub fn new(s: impl AsRef<str>) -> Self {
        let n = B::normalize_separators(s.as_ref());
        Self(n.into_owned(), std::marker::PhantomData)
    }

    pub fn from_path_buf(p: PathBuf) -> Result<Self, PathBuf> {
        match p.into_os_string().into_string() {
            Ok(s) => Ok(Self::new(s)),
            Err(os) => match os.to_str() {
                Some(valid) => Ok(Self::new(valid)),
                None => Err(PathBuf::from(os)),
            },
        }
    }

    pub fn push(&mut self, s: impl AsRef<str>) {
        let normalized = B::normalize_separators(s.as_ref());
        let normalized = normalized.as_ref();
        if self.0.is_empty() || B::is_absolute(normalized) {
            self.0 = normalized.to_string();
        } else {
            if !self.0.ends_with(B::SEP) {
                self.0.push(B::SEP);
            }
            self.0.push_str(normalized);
        }
    }

    pub fn join(&self, s: impl AsRef<str>) -> Self {
        let mut result = self.clone();
        result.push(s);
        result
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn normalize(&self) -> Self {
        let s = &self.0;
        if s.is_empty() {
            return Self(".".to_string(), std::marker::PhantomData);
        }

        let trailing = s.ends_with(B::SEP);

        let components = B::parse_components(s);

        let mut prefix: Option<&str> = None;
        let mut rooted = false;
        let mut stack: Vec<&str> = Vec::new();

        for c in components {
            match c {
                Component::Prefix(p) => {
                    prefix = Some(p);
                    stack.clear();
                }
                Component::RootDir => {
                    rooted = true;
                }
                Component::CurDir => {}
                Component::ParentDir => {
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
                Component::Normal(n) => stack.push(n),
            }
        }

        let body = stack.join(B::SEP_STR);

        let mut result = match (prefix, rooted, body.is_empty()) {
            (Some(p), true, true) => format!("{p}{}", B::SEP_STR),
            (Some(p), true, false) => format!("{p}{}{}", B::SEP_STR, body),
            (Some(p), false, _) => format!("{p}{body}"),
            (None, true, _) => format!("{}{body}", B::SEP_STR),
            (None, false, true) => ".".into(),
            (None, false, false) => body,
        };

        if trailing && result != "." && result != ".." && !result.ends_with(B::SEP_STR) {
            result.push_str(B::SEP_STR);
        }

        Self(result, std::marker::PhantomData)
    }

    pub fn resolve(&self, arg: &str) -> Self {
        if arg.is_empty() {
            return self.clone();
        }

        let normalized = B::normalize_separators(arg);
        let normalized_ref = normalized.as_ref();

        if B::is_absolute(normalized_ref) {
            Self::new(normalized_ref).normalize()
        } else {
            self.join(normalized_ref).normalize()
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

        Self::new(rel.join(B::SEP_STR))
    }

    pub fn as_path(&self) -> &PlatformPath<B> {
        PlatformPath::<B>::new(&self.0)
    }

    pub fn to_posix(&self) -> PlatformPathBuf<Posix> {
        PlatformPathBuf::<Posix>::new(self.0.replace('\\', "/"))
    }

    pub fn to_win32(&self) -> PlatformPathBuf<Win32> {
        PlatformPathBuf::<Win32>::new(self.0.replace('/', "\\"))
    }
}

#[cfg(windows)]
impl<B: PathBackend> PlatformPathBuf<B> {
    pub fn to_native(&self) -> PlatformPathBuf<Win32> {
        self.to_win32()
    }
}

#[cfg(not(windows))]
impl<B: PathBackend> PlatformPathBuf<B> {
    pub fn to_native(&self) -> PlatformPathBuf<Posix> {
        self.to_posix()
    }
}

impl<B: PathBackend> Clone for PlatformPathBuf<B> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), std::marker::PhantomData)
    }
}

impl<B: PathBackend> Deref for PlatformPathBuf<B> {
    type Target = PlatformPath<B>;

    fn deref(&self) -> &PlatformPath<B> {
        self.as_path()
    }
}

impl<B: PathBackend> Borrow<PlatformPath<B>> for PlatformPathBuf<B> {
    fn borrow(&self) -> &PlatformPath<B> {
        self.as_path()
    }
}

impl<B: PathBackend> AsRef<str> for PlatformPathBuf<B> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<B: PathBackend> AsRef<Path> for PlatformPathBuf<B> {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl<B: PathBackend> fmt::Debug for PlatformPathBuf<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PlatformPathBuf").field(&self.0).finish()
    }
}

impl<B: PathBackend> fmt::Display for PlatformPathBuf<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<B: PathBackend> PartialEq for PlatformPathBuf<B> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<B: PathBackend> Eq for PlatformPathBuf<B> {}

impl<B: PathBackend> Hash for PlatformPathBuf<B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<B: PathBackend> From<String> for PlatformPathBuf<B> {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl<B: PathBackend> From<&str> for PlatformPathBuf<B> {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
