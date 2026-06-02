use super::*;

#[test]
fn normalize_empty_returns_dot() {
    assert_eq!(OsPathBuf::new("").normalize().into_string(), ".");
}

#[test]
fn normalize_dot_returns_dot() {
    assert_eq!(OsPathBuf::new(".").normalize().into_string(), ".");
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
