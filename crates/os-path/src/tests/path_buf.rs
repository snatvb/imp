use camino::Utf8PathBuf;
use std::path::PathBuf;
use super::*;

// --- construction ---

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
    assert_eq!(p.as_str(), if cfg!(windows) { "\\abs" } else { "/abs" });
}

#[test]
fn push_empty() {
    let mut p = OsPathBuf::new("base");
    p.push("");
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

// --- parent (via Deref to OsPath) ---

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
    let parent = p.parent();
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().as_str(), "");
}

#[test]
fn parent_of_empty() {
    let p = OsPathBuf::new("");
    assert!(p.parent().is_none());
}

// --- file_name (via Deref) ---

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

// --- extension (via Deref) ---

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

#[test]
fn extension_on_dir_returns_none() {
    let p = OsPathBuf::new("a/b");
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

#[test]
fn from_path_buf_non_utf8_returns_err() {
    let result = OsPathBuf::from_path_buf(PathBuf::new());
    assert!(result.is_ok());
}

// --- From trait ---

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
    let raw = "a\tb/c\nd.ts";
    let p = OsPathBuf::new(raw);
    assert!(p.as_str().contains("a\tb"));
    assert!(p.as_str().contains("c\nd.ts"));
}

#[test]
fn clone_equality() {
    let p1 = OsPathBuf::new("a/b/c.ts");
    let p2 = p1.clone();
    assert_eq!(p1, p2);
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
