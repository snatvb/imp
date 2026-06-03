use std::fmt;
use std::marker::PhantomData;
use std::path::Path;

use crate::backend::{Component, PathBackend};
use crate::path_buf::PlatformPathBuf;

/// Unsized path view, parameterized by backend.
/// `PhantomData<B>` before `str` keeps `B` used but doesn't affect layout (ZST).
#[repr(transparent)]
pub struct PlatformPath<B: PathBackend> {
    _phantom: PhantomData<B>,
    inner: str,
}

impl<B: PathBackend> PlatformPath<B> {
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Self {
        let s = s.as_ref();
        unsafe { &*(s as *const str as *const Self) }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn is_absolute(&self) -> bool {
        B::is_absolute(&self.inner)
    }

    pub fn parent(&self) -> Option<&PlatformPath<B>> {
        let s = &self.inner;
        if s.is_empty() {
            return None;
        }

        let (root, body) = B::split_root(s);

        if body.is_empty() {
            return None;
        }

        let root_len = root.len();
        match body.rfind(B::SEP) {
            Some(sep_pos) => {
                let end = root_len + sep_pos;
                Some(Self::new(&s[..end]))
            }
            None => Some(Self::new(root)),
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        let s = &self.inner;
        if s.is_empty() {
            return None;
        }

        let (_root, body) = B::split_root(s);

        if body.is_empty() {
            return None;
        }

        let trimmed = body.trim_end_matches(B::SEP);
        if trimmed.is_empty() {
            return None;
        }

        match trimmed.rfind(B::SEP) {
            Some(pos) => Some(&trimmed[pos + 1..]),
            None => Some(trimmed),
        }
    }

    pub fn extension(&self) -> Option<&str> {
        let name = self.file_name()?;
        let dot_pos = name.rfind('.')?;
        if dot_pos == 0 {
            return None;
        }
        Some(&name[dot_pos + 1..])
    }

    pub fn root(&self) -> &str {
        let (root, _body) = B::split_root(&self.inner);
        root
    }

    pub fn root_and_normal_parts(&self) -> (&str, Vec<&str>) {
        let s = &self.inner;
        let mut root_end = 0;
        let mut parts = Vec::new();
        for component in B::parse_components(s) {
            match component {
                Component::Prefix(p) => root_end = p.len(),
                Component::RootDir => root_end += 1,
                Component::Normal(n) => parts.push(n),
                _ => {}
            }
        }
        (&s[..root_end], parts)
    }

    pub fn to_platform_path_buf(&self) -> PlatformPathBuf<B> {
        PlatformPathBuf(self.inner.to_string(), PhantomData)
    }
}

impl<B: PathBackend> ToOwned for PlatformPath<B> {
    type Owned = PlatformPathBuf<B>;

    fn to_owned(&self) -> PlatformPathBuf<B> {
        self.to_platform_path_buf()
    }
}

impl<B: PathBackend> AsRef<str> for PlatformPath<B> {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl<B: PathBackend> AsRef<Path> for PlatformPath<B> {
    fn as_ref(&self) -> &Path {
        Path::new(&self.inner)
    }
}

impl<B: PathBackend> fmt::Display for PlatformPath<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<B: PathBackend> fmt::Debug for PlatformPath<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PlatformPath").field(&&self.inner).finish()
    }
}

impl<B: PathBackend> PartialEq for PlatformPath<B> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<B: PathBackend> Eq for PlatformPath<B> {}
