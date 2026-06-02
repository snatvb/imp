use super::*;

pub(super) fn posix(s: &str) -> String {
    if cfg!(windows) {
        s.replace('/', "\\")
    } else {
        s.to_string()
    }
}

mod normalize;
mod path;
mod path_buf;
