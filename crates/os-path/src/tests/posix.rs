use crate::backend::*;
use crate::*;

fn pb(s: &str) -> PosixPathBuf {
    PosixPathBuf::new(s)
}

fn p(s: &str) -> &PosixPath {
    PosixPath::new(s)
}

// --- parse_components ---

#[test]
fn parse_relative() {
    let c = Posix::parse_components("foo/bar");
    assert_eq!(c, vec![Component::Normal("foo"), Component::Normal("bar")]);
}

#[test]
fn parse_absolute() {
    let c = Posix::parse_components("/foo/bar");
    assert_eq!(
        c,
        vec![
            Component::RootDir,
            Component::Normal("foo"),
            Component::Normal("bar"),
        ]
    );
}

#[test]
fn parse_root_only() {
    let c = Posix::parse_components("/");
    assert_eq!(c, vec![Component::RootDir]);
}

#[test]
fn parse_curdir_parentdir() {
    let c = Posix::parse_components("foo/./../bar");
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
    let c = Posix::parse_components("");
    assert!(c.is_empty());
}

#[test]
fn parse_dot() {
    let c = Posix::parse_components(".");
    assert_eq!(c, vec![Component::CurDir]);
}

#[test]
fn parse_double_dot() {
    let c = Posix::parse_components("..");
    assert_eq!(c, vec![Component::ParentDir]);
}

// --- split_root ---

#[test]
fn split_root_absolute() {
    assert_eq!(Posix::split_root("/foo/bar"), ("/", "foo/bar"));
}

#[test]
fn split_root_relative() {
    assert_eq!(Posix::split_root("foo/bar"), ("", "foo/bar"));
}

#[test]
fn split_root_root_only() {
    assert_eq!(Posix::split_root("/"), ("/", ""));
}

// --- is_absolute ---

#[test]
fn absolute_root() {
    assert!(Posix::is_absolute("/foo"));
}

#[test]
fn absolute_relative() {
    assert!(!Posix::is_absolute("foo/bar"));
}

#[test]
fn absolute_empty() {
    assert!(!Posix::is_absolute(""));
}

// --- root ---

#[test]
fn root_absolute() {
    assert_eq!(p("/foo/bar").root(), "/");
}

#[test]
fn root_relative() {
    assert_eq!(p("foo/bar").root(), "");
}

#[test]
fn root_root_only() {
    assert_eq!(p("/").root(), "/");
}

// --- parent ---

#[test]
fn parent_absolute() {
    assert_eq!(p("/foo/bar").parent().unwrap().as_str(), "/foo");
}

#[test]
fn parent_root() {
    assert!(p("/").parent().is_none());
}

#[test]
fn parent_single_relative() {
    assert_eq!(p("foo").parent().unwrap().as_str(), "");
}

#[test]
fn parent_empty() {
    assert!(p("").parent().is_none());
}

// --- file_name ---

#[test]
fn file_name_regular() {
    assert_eq!(p("a/b/c.ts").file_name(), Some("c.ts"));
}

#[test]
fn file_name_no_ext() {
    assert_eq!(p("a/b/makefile").file_name(), Some("makefile"));
}

#[test]
fn file_name_root() {
    assert_eq!(p("/").file_name(), None);
}

#[test]
fn file_name_trailing_slash() {
    assert_eq!(p("a/b/").file_name(), Some("b"));
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
    let r = pb("//foo///bar//").normalize();
    assert_eq!(r.into_string(), "/foo/bar/");
}

#[test]
fn norm_resolves_parent() {
    let r = pb("/foo/bar/../baz/./quux").normalize();
    assert_eq!(r.into_string(), "/foo/baz/quux");
}

#[test]
fn norm_above_root_clamped() {
    let r = pb("/../../foo").normalize();
    assert_eq!(r.into_string(), "/foo");
}

#[test]
fn norm_relative_up() {
    let r = pb("a/../../b").normalize();
    assert_eq!(r.into_string(), "../b");
}

#[test]
fn norm_foo_dotdot_to_dot() {
    let r = pb("foo/..").normalize();
    assert_eq!(r.into_string(), ".");
}

#[test]
fn norm_node_example() {
    let r = pb("/foo/bar//baz/asdf/quux/..").normalize();
    assert_eq!(r.into_string(), "/foo/bar/baz/asdf");
}

#[test]
fn norm_trailing_preserved() {
    let r = pb("/foo/bar/").normalize();
    assert_eq!(r.into_string(), "/foo/bar/");
}

// --- resolve ---

#[test]
fn resolve_relative() {
    let base = pb("/a/b");
    let r = base.resolve("c/d");
    assert_eq!(r.into_string(), "/a/b/c/d");
}

#[test]
fn resolve_absolute() {
    let base = pb("/a/b");
    let r = base.resolve("/c/d");
    assert_eq!(r.into_string(), "/c/d");
}

#[test]
fn resolve_empty() {
    let base = pb("/a/b");
    let r = base.resolve("");
    assert_eq!(r.into_string(), "/a/b");
}

#[test]
fn resolve_dotdot() {
    let base = pb("/a/b");
    let r = base.resolve("../c");
    assert_eq!(r.into_string(), "/a/c");
}

// --- push ---

#[test]
fn push_absolute_replaces() {
    let mut p = pb("base");
    p.push("/abs");
    assert_eq!(p.into_string(), "/abs");
}

// --- relative_to ---

#[test]
fn rel_same() {
    let a = pb("/a/b/c");
    let b = pb("/a/b/c");
    assert_eq!(a.relative_to(&b).into_string(), "");
}

#[test]
fn rel_parent() {
    let a = pb("/a/b/c");
    let b = pb("/a/b");
    assert_eq!(a.relative_to(&b).into_string(), "..");
}

#[test]
fn rel_sibling() {
    let a = pb("/a/b/c");
    let b = pb("/a/b/d");
    assert_eq!(a.relative_to(&b).into_string(), "../d");
}

#[test]
fn rel_posix_example() {
    let from = pb("/data/orandea/test/aaa");
    let to = pb("/data/orandea/impl/bbb");
    assert_eq!(from.relative_to(&to).into_string(), "../../impl/bbb");
}

#[test]
fn rel_subdir() {
    let from = pb("/a/b/c");
    let to = pb("/a/b/c/d");
    assert_eq!(from.relative_to(&to).into_string(), "d");
}

// --- OsPath compat via PosixPath ---

#[test]
fn new_unicode() {
    let p = pb("путь/к/файлу.ts");
    assert_eq!(p.as_str(), "путь/к/файлу.ts");
}
