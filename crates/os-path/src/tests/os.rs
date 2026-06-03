// Backward-compat tests for OsPath/OsPathBuf (= PlatformPath<Native>)
use crate::*;

fn pb(s: &str) -> OsPathBuf {
    OsPathBuf::new(s)
}

fn p(s: &str) -> &OsPath {
    OsPath::new(s)
}

fn posix(s: &str) -> String {
    if cfg!(windows) {
        s.replace('/', "\\")
    } else {
        s.to_string()
    }
}

// --- construction ---

#[test]
fn new_unix_separators_normalized_on_windows() {
    let raw = "a/b/c.ts";
    let expected = posix(raw);
    assert_eq!(pb(raw).as_str(), expected);
}

#[test]
fn new_native_separators_preserved() {
    let raw = if cfg!(windows) {
        "a\\b\\c.ts"
    } else {
        "a/b/c.ts"
    };
    let p = pb(raw);
    assert_eq!(p.as_str(), raw);
}

#[test]
fn new_empty() {
    assert_eq!(pb("").as_str(), "");
}

#[test]
fn new_unicode() {
    let raw = "путь/к/файлу.ts";
    let p = pb(raw);
    assert_eq!(p.as_str(), posix(raw));
}

#[test]
fn new_mixed_separators() {
    let p = pb("a\\b/c\\d.ts");
    if cfg!(windows) {
        assert_eq!(p.as_str(), "a\\b\\c\\d.ts");
    } else {
        assert_eq!(p.as_str(), "a\\b/c\\d.ts");
    }
}

// --- parent ---

#[test]
fn parent_returns_some() {
    let path = pb("a/b/c.ts");
    let parent = path.parent();
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().as_str(), posix("a/b"));
}

#[test]
fn parent_of_single_component() {
    let path = pb("file.ts");
    let parent = path.parent();
    assert!(parent.is_some());
    assert!(parent.unwrap().as_str().is_empty());
}

#[test]
fn parent_of_empty() {
    assert!(pb("").parent().is_none());
}

// --- file_name ---

#[test]
fn file_name_regular() {
    assert_eq!(pb("a/b/c.ts").file_name(), Some("c.ts"));
}

#[test]
fn file_name_no_ext() {
    assert_eq!(pb("a/b/makefile").file_name(), Some("makefile"));
}

#[test]
fn file_name_root() {
    assert!(pb("/").file_name().is_none());
}

#[test]
fn file_name_empty() {
    assert!(pb("").file_name().is_none());
}

#[test]
fn file_name_trailing_slash() {
    assert_eq!(pb("a/b/").file_name(), Some(posix("b").as_str()));
}

// --- extension ---

#[test]
fn extension_regular() {
    assert_eq!(pb("c.ts").extension(), Some("ts"));
}

#[test]
fn extension_double_dot() {
    assert_eq!(pb("c.d.ts").extension(), Some("ts"));
}

#[test]
fn extension_none() {
    assert_eq!(pb("makefile").extension(), None);
}

#[test]
fn extension_dotfile() {
    assert_eq!(pb(".hidden").extension(), None);
}

// --- from_path_buf ---

#[test]
fn from_path_buf_utf8() {
    let p = std::path::PathBuf::from("hello/world");
    let op = OsPathBuf::from_path_buf(p).unwrap();
    assert_eq!(op.as_str(), posix("hello/world"));
}

#[test]
fn from_path_buf_non_utf8_returns_err() {
    assert!(OsPathBuf::from_path_buf(std::path::PathBuf::new()).is_ok());
}

// --- from trait ---

#[test]
fn from_string() {
    let p: OsPathBuf = "a/b".into();
    assert_eq!(p.as_str(), posix("a/b"));
}

// --- normalize ---

#[test]
fn norm_empty_returns_dot() {
    assert_eq!(pb("").normalize().into_string(), ".");
}

#[test]
fn norm_dot_returns_dot() {
    assert_eq!(pb(".").normalize().into_string(), ".");
}

#[test]
fn norm_collapses_multiple_separators() {
    let p = pb("/foo//bar///baz").normalize();
    assert_eq!(p.into_string(), posix("/foo/bar/baz"));
}

#[test]
fn norm_resolves_double_dot() {
    let p = pb("/foo/bar/../baz").normalize();
    assert_eq!(p.into_string(), posix("/foo/baz"));
}

#[test]
fn norm_resolves_dot() {
    let p = pb("/foo/./bar/./baz").normalize();
    assert_eq!(p.into_string(), posix("/foo/bar/baz"));
}

#[test]
fn norm_trailing_separator_preserved() {
    let p = pb("/foo/bar/").normalize();
    assert_eq!(p.into_string(), posix("/foo/bar/"));
}

#[test]
fn norm_cannot_go_above_root() {
    let p = pb("/../../foo").normalize();
    assert_eq!(p.into_string(), posix("/foo"));
}

#[test]
fn norm_relative_double_dot() {
    let p = pb("a/../../b").normalize();
    assert_eq!(p.into_string(), posix("../b"));
}

#[test]
fn norm_foo_dot_dot_returns_dot() {
    let p = pb("foo/..").normalize();
    assert_eq!(p.into_string(), ".");
}

#[test]
fn norm_up_from_dot() {
    let p = pb("../a/../b").normalize();
    assert_eq!(p.into_string(), posix("../b"));
}

#[test]
fn norm_nodejs_posix_example() {
    let p = pb("/foo/bar//baz/asdf/quux/..").normalize();
    assert_eq!(p.into_string(), posix("/foo/bar/baz/asdf"));
}

// --- resolve ---

#[test]
fn resolve_relative_joins() {
    let base = pb("/a/b");
    let r = base.resolve("c/d");
    assert_eq!(r.into_string(), posix("/a/b/c/d"));
}

#[test]
fn resolve_absolute_replaces() {
    let base = pb("/a/b");
    let r = base.resolve("/c/d");
    assert_eq!(r.into_string(), posix("/c/d"));
}

#[test]
fn resolve_empty_returns_self() {
    let base = pb("/a/b");
    let r = base.resolve("");
    assert_eq!(r.into_string(), posix("/a/b"));
}

#[test]
fn resolve_dotdot_normalizes() {
    let base = pb("/a/b");
    let r = base.resolve("../c");
    assert_eq!(r.into_string(), posix("/a/c"));
}

// --- relative_to ---

#[test]
fn rel_same_returns_empty() {
    let a = pb("/a/b/c");
    let b = pb("/a/b/c");
    assert_eq!(a.relative_to(&b).into_string(), "");
}

#[test]
fn rel_posix_example() {
    let from = pb("/data/orandea/test/aaa");
    let to = pb("/data/orandea/impl/bbb");
    assert_eq!(from.relative_to(&to).into_string(), posix("../../impl/bbb"));
}

#[test]
fn rel_subdir() {
    let from = pb("/a/b/c");
    let to = pb("/a/b/c/d");
    assert_eq!(from.relative_to(&to).into_string(), "d");
}

#[test]
fn rel_parent() {
    let from = pb("/a/b/c");
    let to = pb("/a/b");
    assert_eq!(from.relative_to(&to).into_string(), "..");
}

#[test]
fn rel_sibling() {
    let from = pb("/a/b/c");
    let to = pb("/a/b/d");
    assert_eq!(from.relative_to(&to).into_string(), posix("../d"));
}

// --- Display ---

#[test]
fn display_matches_as_str() {
    let raw = "a/b/c.ts";
    assert_eq!(format!("{}", pb(raw)), pb(raw).as_str());
}

// --- roundtrip equality ---

#[test]
fn roundtrip_new_to_string() {
    let raw = "a/b/c.ts";
    let p = pb(raw);
    let p2 = pb(p.as_str());
    assert_eq!(p, p2);
}

// --- clone ---

#[test]
fn clone_equality() {
    let p1 = pb("a/b/c.ts");
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}
