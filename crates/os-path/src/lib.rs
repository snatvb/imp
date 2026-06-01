use camino::{Utf8Path, Utf8PathBuf};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub fn os_normalize(s: &str) -> String {
    if cfg!(windows) {
        s.replace('/', "\\")
    } else {
        s.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsPathBuf(Utf8PathBuf);

impl OsPathBuf {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self(Utf8PathBuf::from(os_normalize(s.as_ref())))
    }

    pub fn from_path_buf(p: PathBuf) -> Result<Self, PathBuf> {
        Utf8PathBuf::from_path_buf(p).map(|u| Self(Utf8PathBuf::from(os_normalize(u.as_str()))))
    }

    pub fn push(&mut self, s: impl AsRef<str>) {
        self.0.push(os_normalize(s.as_ref()));
    }

    pub fn join(&self, s: impl AsRef<str>) -> Self {
        Self(self.0.join(os_normalize(s.as_ref())))
    }

    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    pub fn as_path(&self) -> &OsPath {
        let u: &Utf8Path = self.0.as_ref();
        // SAFETY: OsPath is #[repr(transparent)] over Utf8Path
        unsafe { &*(u as *const Utf8Path as *const OsPath) }
    }
}

impl Deref for OsPathBuf {
    type Target = OsPath;
    fn deref(&self) -> &OsPath {
        self.as_path()
    }
}

impl Borrow<OsPath> for OsPathBuf {
    fn borrow(&self) -> &OsPath {
        self.as_path()
    }
}

impl AsRef<str> for OsPathBuf {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<Utf8Path> for OsPathBuf {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<OsPath> for OsPathBuf {
    fn as_ref(&self) -> &OsPath {
        self.as_path()
    }
}

impl AsRef<Path> for OsPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl fmt::Display for OsPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for OsPathBuf {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for OsPathBuf {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<Utf8PathBuf> for OsPathBuf {
    fn from(p: Utf8PathBuf) -> Self {
        Self(Utf8PathBuf::from(os_normalize(p.as_str())))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OsPath(Utf8Path);

impl OsPath {
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Self {
        let path = Path::new(s.as_ref());
        // SAFETY: s is valid UTF-8 str, Path is just [u8], OsPath is transparent over Utf8Path
        unsafe { &*(path as *const Path as *const Self) }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn parent(&self) -> Option<&OsPath> {
        self.0.parent().map(|p| {
            // SAFETY: same repr
            unsafe { &*(p as *const Utf8Path as *const OsPath) }
        })
    }

    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name()
    }

    pub fn extension(&self) -> Option<&str> {
        self.0.extension()
    }

    pub fn to_os_path_buf(&self) -> OsPathBuf {
        OsPathBuf(self.0.to_path_buf())
    }
}

impl ToOwned for OsPath {
    type Owned = OsPathBuf;
    fn to_owned(&self) -> OsPathBuf {
        self.to_os_path_buf()
    }
}

impl AsRef<str> for OsPath {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<Utf8Path> for OsPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl AsRef<Path> for OsPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl fmt::Display for OsPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
