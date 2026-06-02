use std::borrow::Cow;

pub fn os_normalize<'a>(s: &'a str) -> Cow<'a, str> {
    if cfg!(windows) && s.contains('/') {
        s.replace('/', "\\").into()
    } else {
        Cow::Borrowed(s)
    }
}
