use js_core::error::SystemError;
use js_core::error::{JsError, make_error, make_type_error};
use js_core::js;

js_core::declare_into_js_result!();

#[derive(Debug)]
pub enum EnvError {
    Parse(String),
    Format(String),
    Expand(String),
    Io(SystemError),
    Argument(String),
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::Parse(s) => write!(f, "{s}"),
            EnvError::Format(s) => write!(f, "{s}"),
            EnvError::Expand(s) => write!(f, "{s}"),
            EnvError::Io(s) => write!(f, "{s}"),
            EnvError::Argument(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for EnvError {}

impl EnvError {
    pub fn io(e: std::io::Error, syscall: &'static str, path: Option<String>) -> Self {
        EnvError::Io(SystemError::from_io(e, syscall, path))
    }
}

impl JsError for EnvError {
    fn into_js<'js>(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        match self {
            EnvError::Io(s) => s.into_js(ctx),
            EnvError::Argument(s) => make_type_error(ctx, s),
            _ => make_error(ctx, self.to_string()),
        }
    }
}

js_core::impl_into_js_result!(IntoJsResult, EnvError);
