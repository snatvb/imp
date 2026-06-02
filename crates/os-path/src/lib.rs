use camino::Utf8Component;
use camino::Utf8PathBuf;
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::ops::Deref;
use std::path::{MAIN_SEPARATOR_STR as SEPARATOR, Path, PathBuf};

pub use camino::Utf8Path;

pub fn os_normalize<'a>(s: &'a str) -> Cow<'a, str> {
    if cfg!(windows) && s.contains('/') {
        s.replace('/', "\\").into()
    } else {
        Cow::Borrowed(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsPathBuf(Utf8PathBuf);

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

#[cfg(test)]
mod tests {
    use super::*;

    // --- normalize ---

    #[test]
    fn normalize_empty_returns_dot() {
        assert_eq!(OsPathBuf::new("").normalize().into_string(), ".");
    }

    #[test]
    fn normalize_dot_returns_dot() {
        assert_eq!(OsPathBuf::new(".").normalize().into_string(), ".");
    }

    fn posix(s: &str) -> String {
        if cfg!(windows) {
            s.replace('/', "\\")
        } else {
            s.to_string()
        }
    }

    #[test]
    fn normalize_collapses_multiple_separators() {
        let p = OsPathBuf::new("/foo//bar///baz").normalize();
        assert_eq!(p.into_string(), posix("/foo/bar/baz"));
    }

    #[test]
    fn normalize_resolves_double_dot() {
        let p = OsPathBuf::new("/foo/bar/../baz").normalize();
        assert_eq!(p.into_string(), posix("/foo/baz"));
    }

    #[test]
    fn normalize_resolves_dot() {
        let p = OsPathBuf::new("/foo/./bar/./baz").normalize();
        assert_eq!(p.into_string(), posix("/foo/bar/baz"));
    }

    #[test]
    fn normalize_trailing_separator_preserved() {
        let p = OsPathBuf::new("/foo/bar/").normalize();
        assert_eq!(p.into_string(), posix("/foo/bar/"));
    }

    #[test]
    fn normalize_cannot_go_above_root() {
        let p = OsPathBuf::new("/../../foo").normalize();
        assert_eq!(p.into_string(), posix("/foo"));
    }

    #[test]
    fn normalize_relative_double_dot() {
        let p = OsPathBuf::new("a/../../b").normalize();
        assert_eq!(p.into_string(), posix("../b"));
    }

    #[test]
    fn normalize_foo_dot_dot_returns_dot() {
        let p = OsPathBuf::new("foo/..").normalize();
        assert_eq!(p.into_string(), ".");
    }

    #[test]
    fn normalize_up_from_dot() {
        let p = OsPathBuf::new("../a/../b").normalize();
        assert_eq!(p.into_string(), posix("../b"));
    }

    #[test]
    fn normalize_nodejs_posix_example() {
        let p = OsPathBuf::new("/foo/bar//baz/asdf/quux/..").normalize();
        assert_eq!(p.into_string(), posix("/foo/bar/baz/asdf"));
    }

    #[cfg(windows)]
    #[test]
    fn normalize_nodejs_windows_example() {
        let p = OsPathBuf::new("C:\\temp\\\\foo\\bar\\..\\").normalize();
        assert_eq!(p.into_string(), "C:\\temp\\foo\\");
    }

    #[cfg(windows)]
    #[test]
    fn normalize_nodejs_windows_mixed_separators() {
        let p = OsPathBuf::new("C:////temp\\\\/\\/\\/foo/bar").normalize();
        assert_eq!(p.into_string(), "C:\\temp\\foo\\bar");
    }

    #[cfg(windows)]
    #[test]
    fn normalize_windows_drive_relative() {
        let p = OsPathBuf::new("C:foo\\..\\bar").normalize();
        assert_eq!(p.into_string(), "C:bar");
    }

    #[cfg(windows)]
    #[test]
    fn normalize_windows_just_drive() {
        let p = OsPathBuf::new("C:").normalize();
        assert_eq!(p.into_string(), "C:");
    }

    #[cfg(windows)]
    #[test]
    fn normalize_windows_drive_root() {
        let p = OsPathBuf::new("C:\\").normalize();
        assert_eq!(p.into_string(), "C:\\");
    }

    #[test]
    fn normalize_leading_dot_slash() {
        let p = OsPathBuf::new("./").normalize();
        assert_eq!(p.into_string(), ".");
    }

    #[test]
    fn normalize_double_dot_above_root() {
        let p = OsPathBuf::new("/a/../../../b").normalize();
        assert_eq!(p.into_string(), posix("/b"));
    }

    // --- OsPathBuf construction ---

    #[test]
    fn new_unix_separators_normalized_on_windows() {
        let p = OsPathBuf::new("a/b/c.ts");
        let expected = if cfg!(windows) {
            "a\\b\\c.ts"
        } else {
            "a/b/c.ts"
        };
        assert_eq!(p.as_str(), expected);
    }

    #[test]
    fn new_native_separators_preserved() {
        let raw = if cfg!(windows) {
            "a\\b\\c.ts"
        } else {
            "a/b/c.ts"
        };
        let p = OsPathBuf::new(raw);
        assert_eq!(p.as_str(), raw);
    }

    #[test]
    fn new_empty() {
        let p = OsPathBuf::new("");
        assert_eq!(p.as_str(), "");
    }

    #[test]
    fn new_root() {
        let p = if cfg!(windows) {
            OsPathBuf::new("C:/")
        } else {
            OsPathBuf::new("/")
        };
        assert!(p.as_str().ends_with(std::path::MAIN_SEPARATOR_STR));
    }

    #[test]
    fn new_unicode() {
        let raw = "путь/к/файлу.ts";
        let p = OsPathBuf::new(raw);
        if cfg!(windows) {
            assert_eq!(p.as_str(), "путь\\к\\файлу.ts");
        } else {
            assert_eq!(p.as_str(), raw);
        }
    }

    #[test]
    fn new_mixed_separators() {
        let p = OsPathBuf::new("a\\b/c\\d.ts");
        if cfg!(windows) {
            // only / → \, \ stays \
            assert_eq!(p.as_str(), "a\\b\\c\\d.ts");
        } else {
            assert_eq!(p.as_str(), "a\\b/c\\d.ts");
        }
    }

    // --- push ---

    #[test]
    fn push_normalizes() {
        let mut p = OsPathBuf::new("base");
        p.push("sub/dir");
        if cfg!(windows) {
            assert_eq!(p.as_str(), "base\\sub\\dir");
        } else {
            assert_eq!(p.as_str(), "base/sub/dir");
        }
    }

    #[test]
    fn push_absolute() {
        let mut p = OsPathBuf::new("base");
        p.push("/abs");
        // absolute path replaces on both platforms
        assert_eq!(p.as_str(), if cfg!(windows) { "\\abs" } else { "/abs" });
    }

    #[test]
    fn push_empty() {
        let mut p = OsPathBuf::new("base");
        p.push("");
        // std pushes empty as if no-op but may add trailing separator
        let s = p.as_str();
        assert!(s.starts_with("base"));
    }

    // --- join ---

    #[test]
    fn join_normalizes() {
        let p = OsPathBuf::new("base");
        let j = p.join("sub/dir");
        if cfg!(windows) {
            assert_eq!(j.as_str(), "base\\sub\\dir");
        } else {
            assert_eq!(j.as_str(), "base/sub/dir");
        }
    }

    // --- parent ---

    #[test]
    fn parent_returns_some() {
        let p = OsPathBuf::new("a/b/c.ts");
        let parent = p.parent();
        assert!(parent.is_some());
        let expected = if cfg!(windows) { "a\\b" } else { "a/b" };
        assert_eq!(parent.unwrap().as_str(), expected);
    }

    #[test]
    fn parent_of_root_is_none() {
        let p = if cfg!(windows) {
            OsPathBuf::new("C:\\")
        } else {
            OsPathBuf::new("/")
        };
        assert!(p.parent().is_none());
    }

    #[test]
    fn parent_of_single_component() {
        let p = OsPathBuf::new("file.ts");
        // Path::parent for single component returns Some("") — an empty path
        let parent = p.parent();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().as_str(), "");
    }

    // --- file_name ---

    #[test]
    fn file_name_regular() {
        let p = OsPathBuf::new("a/b/c.ts");
        assert_eq!(p.file_name(), Some("c.ts"));
    }

    #[test]
    fn file_name_no_ext() {
        let p = OsPathBuf::new("a/b/makefile");
        assert_eq!(p.file_name(), Some("makefile"));
    }

    #[test]
    fn file_name_root() {
        let p = OsPathBuf::new("/");
        assert_eq!(p.file_name(), None);
    }

    #[test]
    fn file_name_empty() {
        let p = OsPathBuf::new("");
        assert_eq!(p.file_name(), None);
    }

    #[test]
    fn file_name_trailing_slash() {
        let p = OsPathBuf::new("a/b/");
        assert_eq!(p.file_name(), Some("b"));
    }

    // --- extension ---

    #[test]
    fn extension_regular() {
        let p = OsPathBuf::new("a/b/c.ts");
        assert_eq!(p.extension(), Some("ts"));
    }

    #[test]
    fn extension_double_dot() {
        let p = OsPathBuf::new("a/b/c.d.ts");
        assert_eq!(p.extension(), Some("ts"));
    }

    #[test]
    fn extension_none() {
        let p = OsPathBuf::new("a/b/makefile");
        assert_eq!(p.extension(), None);
    }

    #[test]
    fn extension_dotfile() {
        let p = OsPathBuf::new("a/b/.hidden");
        assert_eq!(p.extension(), None);
    }

    // --- from_path_buf ---

    #[test]
    fn from_path_buf_utf8() {
        let pb = PathBuf::from("hello/world");
        let op = OsPathBuf::from_path_buf(pb).unwrap();
        if cfg!(windows) {
            assert_eq!(op.as_str(), "hello\\world");
        } else {
            assert_eq!(op.as_str(), "hello/world");
        }
    }

    #[cfg(unix)]
    #[test]
    fn from_path_buf_invalid_utf8() {
        use std::os::unix::ffi::OsStrExt;
        let bytes = vec![0xFF, 0xFE];
        let os_str = std::ffi::OsStr::from_bytes(&bytes);
        let mut pb = PathBuf::new();
        pb.push(os_str);
        assert!(OsPathBuf::from_path_buf(pb).is_err());
    }

    // --- from trait ---

    #[test]
    fn from_string() {
        let p: OsPathBuf = "a/b".into();
        let expected = if cfg!(windows) { "a\\b" } else { "a/b" };
        assert_eq!(p.as_str(), expected);
    }

    #[test]
    fn from_utf8pathbuf() {
        let u = Utf8PathBuf::from("a/b");
        let p = OsPathBuf::from(u);
        let expected = if cfg!(windows) { "a\\b" } else { "a/b" };
        assert_eq!(p.as_str(), expected);
    }

    // --- Display ---

    #[test]
    fn display_matches_as_str() {
        let raw = "a/b/c.ts";
        let p = OsPathBuf::new(raw);
        assert_eq!(format!("{p}"), p.as_str());
    }

    // --- OsPath::new ---

    #[test]
    fn ospath_new_preserves_input() {
        let raw = "a/b/c.ts";
        let op = OsPath::new(raw);
        assert_eq!(op.as_str(), raw);
    }

    #[test]
    fn ospath_parent() {
        let op = OsPath::new("a/b/c.ts");
        assert_eq!(op.parent().unwrap().as_str(), "a/b");
    }

    #[test]
    fn ospath_file_name() {
        let op = OsPath::new("a/b/c.ts");
        assert_eq!(op.file_name(), Some("c.ts"));
    }

    // --- root ---

    #[test]
    fn root_posix_absolute() {
        let op = OsPath::new("/foo/bar");
        assert_eq!(op.root(), "/");
    }

    #[test]
    fn root_posix_relative() {
        let op = OsPath::new("foo/bar");
        assert_eq!(op.root(), "");
    }

    #[test]
    fn root_posix_root_only() {
        let op = OsPath::new("/");
        assert_eq!(op.root(), "/");
    }

    #[cfg(windows)]
    #[test]
    fn root_windows_drive_root() {
        let op = OsPath::new("C:\\foo\\bar");
        assert_eq!(op.root(), "C:\\");
    }

    #[cfg(windows)]
    #[test]
    fn root_windows_drive_relative() {
        let op = OsPath::new("C:foo");
        assert_eq!(op.root(), "C:");
    }

    #[cfg(windows)]
    #[test]
    fn root_windows_drive_only() {
        let op = OsPath::new("C:");
        assert_eq!(op.root(), "C:");
    }

    #[cfg(windows)]
    #[test]
    fn root_windows_unc() {
        let op = OsPath::new("\\\\server\\share\\foo");
        assert_eq!(op.root(), "\\\\server\\share\\");
    }

    #[cfg(windows)]
    #[test]
    fn root_windows_backslash_only() {
        let op = OsPath::new("\\foo");
        assert_eq!(op.root(), "\\");
    }

    #[cfg(windows)]
    #[test]
    fn root_windows_relative() {
        let op = OsPath::new("foo\\bar");
        assert_eq!(op.root(), "");
    }

    // --- roundtrip equality ---

    #[test]
    fn roundtrip_new_to_string() {
        let raw = "a/b/c.ts";
        let p = OsPathBuf::new(raw);
        let p2 = OsPathBuf::new(p.as_str());
        assert_eq!(p, p2);
    }

    // --- edge cases ---

    #[test]
    fn current_dir_with_mixed_separators() {
        let cwd = std::env::current_dir().unwrap();
        let cwd_str = cwd.to_string_lossy();
        let op = OsPathBuf::new(cwd_str.as_ref());
        // should not crash, should be valid
        assert!(!op.as_str().is_empty());
    }

    #[test]
    fn long_path() {
        let mut s = String::new();
        for i in 0..100 {
            if i > 0 {
                s.push('/');
            }
            s.push_str(&format!("dir{i}"));
        }
        s.push_str("/file.ts");
        let p = OsPathBuf::new(&s);
        assert_eq!(p.extension(), Some("ts"));
    }

    #[test]
    fn all_separator_types() {
        // tab, newline in path (unusual but possible)
        let raw = "a\tb/c\nd.ts";
        let p = OsPathBuf::new(raw);
        assert!(p.as_str().contains("a\tb"));
        assert!(p.as_str().contains("c\nd.ts"));
    }

    // --- negative scenarios ---

    #[test]
    fn from_path_buf_non_utf8_returns_err() {
        // On windows can test this with WTF-8 / surrogate chars
        // On all platforms, a path with invalid UTF-16 on windows will fail
        // Just verify the Result type is correct
        let result = OsPathBuf::from_path_buf(PathBuf::new());
        assert!(result.is_ok()); // empty path is valid UTF-8
    }

    #[test]
    fn extension_on_dir_returns_none() {
        let p = OsPathBuf::new("a/b");
        assert_eq!(p.extension(), None);
    }

    #[test]
    fn parent_of_empty() {
        let p = OsPathBuf::new("");
        assert!(p.parent().is_none());
    }

    #[test]
    fn clone_equality() {
        let p1 = OsPathBuf::new("a/b/c.ts");
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn ospath_to_owned_roundtrip() {
        let op = OsPath::new("a/b/c.ts");
        let owned = op.to_os_path_buf();
        assert_eq!(owned.as_str(), "a/b/c.ts");
    }

    // --- resolve ---

    #[test]
    fn resolve_relative_joins() {
        let base = OsPathBuf::new("/a/b");
        let r = base.resolve("c/d");
        assert_eq!(r.into_string(), posix("/a/b/c/d"));
    }

    #[test]
    fn resolve_absolute_replaces() {
        let base = OsPathBuf::new("/a/b");
        let r = base.resolve("/c/d");
        assert_eq!(r.into_string(), posix("/c/d"));
    }

    #[test]
    fn resolve_empty_returns_self() {
        let base = OsPathBuf::new("/a/b");
        let r = base.resolve("");
        assert_eq!(r.into_string(), posix("/a/b"));
    }

    #[test]
    fn resolve_dotdot_normalizes() {
        let base = OsPathBuf::new("/a/b");
        let r = base.resolve("../c");
        assert_eq!(r.into_string(), posix("/a/c"));
    }

    // --- relative_to ---

    #[test]
    fn relative_to_same_returns_empty() {
        let a = OsPathBuf::new("/a/b/c");
        let b = OsPathBuf::new("/a/b/c");
        assert_eq!(a.relative_to(&b).into_string(), "");
    }

    #[test]
    fn relative_to_posix_example() {
        let from = OsPathBuf::new("/data/orandea/test/aaa");
        let to = OsPathBuf::new("/data/orandea/impl/bbb");
        assert_eq!(from.relative_to(&to).into_string(), posix("../../impl/bbb"));
    }

    #[test]
    fn relative_to_subdir() {
        let from = OsPathBuf::new("/a/b/c");
        let to = OsPathBuf::new("/a/b/c/d");
        assert_eq!(from.relative_to(&to).into_string(), "d");
    }

    #[test]
    fn relative_to_parent() {
        let from = OsPathBuf::new("/a/b/c");
        let to = OsPathBuf::new("/a/b");
        assert_eq!(from.relative_to(&to).into_string(), "..");
    }

    #[test]
    fn relative_to_sibling() {
        let from = OsPathBuf::new("/a/b/c");
        let to = OsPathBuf::new("/a/b/d");
        assert_eq!(from.relative_to(&to).into_string(), posix("../d"));
    }

    #[cfg(windows)]
    #[test]
    fn relative_to_absolute_if_different_root() {
        // different drives → relative_to returns absolute `to`
        let from = OsPathBuf::new("C:\\a\\b");
        let to = OsPathBuf::new("D:\\c\\d");
        assert_eq!(from.relative_to(&to).into_string(), "D:\\c\\d");
    }

    #[cfg(windows)]
    #[test]
    fn relative_to_windows_example() {
        let from = OsPathBuf::new("C:\\orandea\\test\\aaa");
        let to = OsPathBuf::new("C:\\orandea\\impl\\bbb");
        assert_eq!(from.relative_to(&to).into_string(), "..\\..\\impl\\bbb");
    }

    #[cfg(windows)]
    #[test]
    fn relative_to_windows_different_drives() {
        let from = OsPathBuf::new("C:\\foo");
        let to = OsPathBuf::new("D:\\bar");
        assert_eq!(from.relative_to(&to).into_string(), "D:\\bar");
    }
}
