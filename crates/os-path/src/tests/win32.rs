use crate::backend::*;
use crate::*;

fn pb(s: &str) -> Win32PathBuf {
    Win32PathBuf::new(s)
}

fn p(s: &str) -> &Win32Path {
    Win32Path::new(s)
}

// --- parse_components ---

#[test]
fn parse_relative() {
    let c = Win32::parse_components(r"foo\bar");
    assert_eq!(c, vec![Component::Normal("foo"), Component::Normal("bar")]);
}

#[test]
fn parse_absolute_drive() {
    let c = Win32::parse_components(r"C:\foo\bar");
    assert_eq!(
        c,
        vec![
            Component::Prefix("C:"),
            Component::RootDir,
            Component::Normal("foo"),
            Component::Normal("bar"),
        ]
    );
}

#[test]
fn parse_drive_relative() {
    let c = Win32::parse_components(r"C:foo\bar");
    assert_eq!(
        c,
        vec![
            Component::Prefix("C:"),
            Component::Normal("foo"),
            Component::Normal("bar")
        ]
    );
}

#[test]
fn parse_just_drive() {
    let c = Win32::parse_components("C:");
    assert_eq!(c, vec![Component::Prefix("C:")]);
}

#[test]
fn parse_drive_root() {
    let c = Win32::parse_components(r"C:\");
    assert_eq!(c, vec![Component::Prefix("C:"), Component::RootDir]);
}

#[test]
fn parse_backslash_root() {
    let c = Win32::parse_components(r"\foo");
    assert_eq!(c, vec![Component::RootDir, Component::Normal("foo")]);
}

#[test]
fn parse_just_backslash() {
    let c = Win32::parse_components(r"\");
    assert_eq!(c, vec![Component::RootDir]);
}

#[test]
fn parse_unc() {
    let c = Win32::parse_components(r"\\server\share\foo");
    assert_eq!(
        c,
        vec![
            Component::Prefix(r"\\server\share"),
            Component::RootDir,
            Component::Normal("foo"),
        ]
    );
}

#[test]
fn parse_unc_no_trailing() {
    let c = Win32::parse_components(r"\\server\share");
    assert_eq!(c, vec![Component::Prefix(r"\\server\share")]);
}

#[test]
fn parse_extended_length() {
    let c = Win32::parse_components(r"\\?\C:\foo");
    assert_eq!(
        c,
        vec![
            Component::Prefix(r"\\?\C:"),
            Component::RootDir,
            Component::Normal("foo"),
        ]
    );
}

#[test]
fn parse_curdir_parentdir() {
    let c = Win32::parse_components(r"foo\.\..\bar");
    assert_eq!(
        c,
        vec![
            Component::Normal("foo"),
            Component::CurDir,
            Component::ParentDir,
            Component::Normal("bar"),
        ]
    );
}

#[test]
fn parse_empty() {
    let c = Win32::parse_components("");
    assert!(c.is_empty());
}

// --- split_root ---

#[test]
fn split_root_drive_absolute() {
    assert_eq!(Win32::split_root(r"C:\foo\bar"), (r"C:\", r"foo\bar"));
}

#[test]
fn split_root_drive_relative() {
    assert_eq!(Win32::split_root("C:foo"), ("C:", "foo"));
}

#[test]
fn split_root_just_drive() {
    assert_eq!(Win32::split_root("C:"), ("C:", ""));
}

#[test]
fn split_root_unc() {
    assert_eq!(
        Win32::split_root(r"\\server\share\foo"),
        (r"\\server\share\", "foo")
    );
}

#[test]
fn split_root_backslash() {
    assert_eq!(Win32::split_root(r"\foo"), (r"\", "foo"));
}

#[test]
fn split_root_relative() {
    assert_eq!(Win32::split_root(r"foo\bar"), ("", r"foo\bar"));
}

// --- is_absolute ---

#[test]
fn absolute_drive_root() {
    assert!(Win32::is_absolute(r"C:\foo"));
}

#[test]
fn absolute_backslash() {
    assert!(Win32::is_absolute(r"\foo"));
}

#[test]
fn absolute_relative() {
    assert!(!Win32::is_absolute(r"foo\bar"));
}

#[test]
fn absolute_drive_no_slash() {
    assert!(!Win32::is_absolute("C:foo"));
}

// --- root ---

#[test]
fn root_drive_absolute() {
    assert_eq!(p(r"C:\foo\bar").root(), r"C:\");
}

#[test]
fn root_drive_relative() {
    assert_eq!(p("C:foo").root(), "C:");
}

#[test]
fn root_just_drive() {
    assert_eq!(p("C:").root(), "C:");
}

#[test]
fn root_unc() {
    assert_eq!(p(r"\\server\share\foo").root(), r"\\server\share\");
}

#[test]
fn root_backslash() {
    assert_eq!(p(r"\foo").root(), r"\");
}

#[test]
fn root_relative() {
    assert_eq!(p(r"foo\bar").root(), "");
}

// --- parent ---

#[test]
fn parent_drive_absolute() {
    let parent = p(r"C:\foo\bar").parent();
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().as_str(), r"C:\foo");
}

#[test]
fn parent_drive_root() {
    assert!(p(r"C:\").parent().is_none());
}

#[test]
fn parent_backslash_root() {
    assert!(p(r"\").parent().is_none());
}

#[test]
fn parent_single_relative() {
    let parent = p("foo").parent();
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().as_str(), "");
}

#[test]
fn parent_empty() {
    assert!(p("").parent().is_none());
}

// --- file_name ---

#[test]
fn file_name_regular() {
    assert_eq!(p(r"a\b\c.ts").file_name(), Some("c.ts"));
}

#[test]
fn file_name_no_ext() {
    assert_eq!(p(r"a\b\makefile").file_name(), Some("makefile"));
}

#[test]
fn file_name_root() {
    assert_eq!(p(r"\").file_name(), None);
}

#[test]
fn file_name_trailing_slash() {
    assert_eq!(p(r"a\b\").file_name(), Some("b"));
}

// --- extension ---

#[test]
fn ext_regular() {
    assert_eq!(p("c.ts").extension(), Some("ts"));
}

#[test]
fn ext_double_dot() {
    assert_eq!(p("c.d.ts").extension(), Some("ts"));
}

#[test]
fn ext_none() {
    assert_eq!(p("makefile").extension(), None);
}

#[test]
fn ext_dotfile() {
    assert_eq!(p(".hidden").extension(), None);
}

// --- normalize ---

#[test]
fn norm_empty_to_dot() {
    assert_eq!(pb("").normalize().into_string(), ".");
}

#[test]
fn norm_dot_stays_dot() {
    assert_eq!(pb(".").normalize().into_string(), ".");
}

#[test]
fn norm_collapses_seps() {
    let r = pb(r"C:\\temp\\\\foo\\bar\\..\\").normalize();
    assert_eq!(r.into_string(), r"C:\temp\foo\");
}

#[test]
fn norm_drive_relative() {
    let r = pb(r"C:foo\..\bar").normalize();
    assert_eq!(r.into_string(), "C:bar");
}

#[test]
fn norm_just_drive() {
    let r = pb("C:").normalize();
    assert_eq!(r.into_string(), "C:");
}

#[test]
fn norm_drive_root() {
    let r = pb(r"C:\").normalize();
    assert_eq!(r.into_string(), r"C:\");
}

#[test]
fn norm_above_root_clamped() {
    let r = pb(r"\..\..\foo").normalize();
    assert_eq!(r.into_string(), r"\foo");
}

#[test]
fn norm_relative_up() {
    let r = pb(r"a\..\..\b").normalize();
    assert_eq!(r.into_string(), r"..\b");
}

#[test]
fn norm_foo_dotdot_to_dot() {
    let r = pb("foo\\..").normalize();
    assert_eq!(r.into_string(), ".");
}

// --- resolve ---

#[test]
fn resolve_relative() {
    let base = pb(r"\a\b");
    let r = base.resolve(r"c\d");
    assert_eq!(r.into_string(), r"\a\b\c\d");
}

#[test]
fn resolve_absolute() {
    let base = pb(r"\a\b");
    let r = base.resolve(r"\c\d");
    assert_eq!(r.into_string(), r"\c\d");
}

#[test]
fn resolve_empty() {
    let base = pb(r"\a\b");
    let r = base.resolve("");
    assert_eq!(r.into_string(), r"\a\b");
}

#[test]
fn resolve_dotdot() {
    let base = pb(r"\a\b");
    let r = base.resolve(r"..\c");
    assert_eq!(r.into_string(), r"\a\c");
}

// --- relative_to ---

#[test]
fn rel_same() {
    let a = pb(r"\a\b\c");
    let b = pb(r"\a\b\c");
    assert_eq!(a.relative_to(&b).into_string(), "");
}

#[test]
fn rel_parent() {
    let a = pb(r"\a\b\c");
    let b = pb(r"\a\b");
    assert_eq!(a.relative_to(&b).into_string(), "..");
}

#[test]
fn rel_sibling() {
    let a = pb(r"\a\b\c");
    let b = pb(r"\a\b\d");
    assert_eq!(a.relative_to(&b).into_string(), r"..\d");
}

#[test]
fn rel_different_drives() {
    let a = pb(r"C:\a\b");
    let b = pb(r"D:\c\d");
    assert_eq!(a.relative_to(&b).into_string(), r"D:\c\d");
}

#[test]
fn rel_unc_example() {
    let a = pb(r"C:\orandea\test\aaa");
    let b = pb(r"C:\orandea\impl\bbb");
    assert_eq!(a.relative_to(&b).into_string(), r"..\..\impl\bbb");
}
