use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Win32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Posix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Component<'a> {
    Prefix(&'a str),
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a str),
}

pub trait PathBackend: Sized + 'static {
    const SEP: char;
    const SEP_STR: &'static str;
    const DELIM: char;

    fn is_absolute(s: &str) -> bool;
    fn parse_components(s: &str) -> Vec<Component<'_>>;
    fn split_root(s: &str) -> (&str, &str);
    fn normalize_separators<'a>(s: &'a str) -> Cow<'a, str>;
}

impl PathBackend for Posix {
    const SEP: char = '/';
    const SEP_STR: &'static str = "/";
    const DELIM: char = ':';

    fn is_absolute(s: &str) -> bool {
        s.starts_with('/')
    }

    fn parse_components(s: &str) -> Vec<Component<'_>> {
        let mut components = Vec::new();
        let len = s.len();
        if len == 0 {
            return components;
        }

        let mut i = 0;

        if s.as_bytes()[0] == b'/' {
            components.push(Component::RootDir);
            i = 1;
            while i < len && s.as_bytes()[i] == b'/' {
                i += 1;
            }
        }

        while i < len {
            let start = i;
            while i < len && s.as_bytes()[i] != b'/' {
                i += 1;
            }
            let part = &s[start..i];
            match part {
                "." => components.push(Component::CurDir),
                ".." => components.push(Component::ParentDir),
                other => components.push(Component::Normal(other)),
            }
            while i < len && s.as_bytes()[i] == b'/' {
                i += 1;
            }
        }

        components
    }

    fn split_root(s: &str) -> (&str, &str) {
        if s.is_empty() {
            return ("", "");
        }
        if s.len() == 1 && s.as_bytes()[0] == b'/' {
            return ("/", "");
        }
        (s, "")
    }

    fn normalize_separators<'a>(s: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(s)
    }
}

impl PathBackend for Win32 {
    const SEP: char = '\\';
    const SEP_STR: &'static str = "\\";
    const DELIM: char = ';';

    fn is_absolute(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let bytes = s.as_bytes();
        if bytes[0] == b'\\' {
            return true;
        }
        bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'\\'
    }

    fn parse_components(s: &str) -> Vec<Component<'_>> {
        let mut components = Vec::new();
        let len = s.len();
        if len == 0 {
            return components;
        }

        let bytes = s.as_bytes();
        let mut i = 0;

        // Try to parse Win32 prefix
        let mut prefix_len = 0;

        // Extended-length \\?\ or \\.\
        if len >= 4
            && bytes[0] == b'\\'
            && bytes[1] == b'\\'
            && (bytes[2] == b'?' || bytes[2] == b'.')
            && bytes[3] == b'\\'
        {
            // \\?\C: (without trailing \ — RootDir handles separator)
            if len >= 6 && bytes[5] == b':' {
                prefix_len = 6;
            // \\?\UNC\server\share\
            } else if len >= 8 && &s[4..7] == "UNC" && bytes[7] == b'\\' {
                let after = &s[8..];
                if let Some(a) = after.find('\\') {
                    let rest = &after[a + 1..];
                    if let Some(b) = rest.find('\\') {
                        prefix_len = 8 + a + 1 + b;
                    }
                }
                if prefix_len == 0 {
                    prefix_len = len;
                }
            } else {
                prefix_len = len;
            }
        // UNC \\server\share\
        } else if len >= 2 && bytes[0] == b'\\' && bytes[1] == b'\\' {
            let after_double = &s[2..];
            if let Some(server_end) = after_double.find('\\') {
                let after_server = &after_double[server_end + 1..];
                if let Some(share_end) = after_server.find('\\') {
                    prefix_len = 2 + server_end + 1 + share_end;
                } else if !after_server.is_empty() {
                    // \\server\share (no trailing backslash)
                    prefix_len = len;
                    components.push(Component::Prefix(&s[..prefix_len]));
                    return components;
                }
                if prefix_len == 0 {
                    // \\server\ with trailing backslash
                    prefix_len = 2 + server_end + 1;
                }
            }
        // Drive letter C:\ or C:
        } else if len >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
            prefix_len = 2;
        }

        if prefix_len > 0 {
            components.push(Component::Prefix(&s[..prefix_len]));
            i = prefix_len;
        }

        // RootDir after prefix (or at start with no prefix)
        if i < len && bytes[i] == b'\\' {
            components.push(Component::RootDir);
            i += 1;
        }

        // Skip consecutive separators after root
        while i < len && bytes[i] == b'\\' {
            i += 1;
        }

        // Remaining components
        while i < len {
            let start = i;
            while i < len && bytes[i] != b'\\' {
                i += 1;
            }
            if i > start {
                let part = &s[start..i];
                match part {
                    "." => components.push(Component::CurDir),
                    ".." => components.push(Component::ParentDir),
                    other => components.push(Component::Normal(other)),
                }
            }
            while i < len && bytes[i] == b'\\' {
                i += 1;
            }
        }

        components
    }

    fn split_root(s: &str) -> (&str, &str) {
        if s.is_empty() {
            return ("", "");
        }
        let bytes = s.as_bytes();
        let len = s.len();

        // Extended-length \\?\ or \\.\
        if len >= 4
            && bytes[0] == b'\\'
            && bytes[1] == b'\\'
            && (bytes[2] == b'?' || bytes[2] == b'.')
            && bytes[3] == b'\\'
        {
            // \\?\C:\ (prefix + RootDir = 7)
            if len >= 7 && bytes[5] == b':' && bytes[6] == b'\\' {
                return (&s[..7], &s[7..]);
            }
            // \\?\UNC\server\share\
            if len >= 8 && &s[4..7] == "UNC" && bytes[7] == b'\\' {
                let after = &s[8..];
                if let Some(a) = after.find('\\') {
                    let rest = &after[a + 1..];
                    if let Some(b) = rest.find('\\') {
                        let root_end = 8 + a + 1 + b + 1; // +1 for trailing \
                        return (&s[..root_end], &s[root_end..]);
                    }
                }
            }
            return (s, "");
        }

        // UNC \\server\share\
        if len >= 2 && bytes[0] == b'\\' && bytes[1] == b'\\' {
            let after_double = &s[2..];
            if let Some(server_end) = after_double.find('\\') {
                let after_server = &after_double[server_end + 1..];
                if let Some(share_end) = after_server.find('\\') {
                    let root_end = 2 + server_end + 1 + share_end + 1; // +1 for trailing \ after share
                    return (&s[..root_end], &s[root_end..]);
                }
                // \\server\share (no trailing backslash) — root is whole path
                return (s, "");
            }
            // \\foo — absolute path, not UNC
            return (&s[..1], &s[1..]);
        }

        // Drive letter C:\ or C:
        if len >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
            if len >= 3 && bytes[2] == b'\\' {
                return (&s[..3], &s[3..]);
            }
            return (&s[..2], &s[2..]);
        }

        // Just backslash
        if bytes[0] == b'\\' {
            return (&s[..1], &s[1..]);
        }

        ("", s)
    }

    fn normalize_separators<'a>(s: &'a str) -> Cow<'a, str> {
        if s.contains('/') {
            s.replace('/', "\\").into()
        } else {
            Cow::Borrowed(s)
        }
    }
}
