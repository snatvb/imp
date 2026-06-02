use super::*;

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

// --- to_os_path_buf roundtrip ---

#[test]
fn ospath_to_owned_roundtrip() {
    let op = OsPath::new("a/b/c.ts");
    let owned = op.to_os_path_buf();
    assert_eq!(owned.as_str(), "a/b/c.ts");
}
