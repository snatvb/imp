use imp_url::JsUrl;
use js_core::utils::StringArg;
use rquickjs::{self as js, Class, Ctx, FromJs, Type, Value};

pub enum AnyURL {
    Str(StringArg),
    Url(JsUrl),
}

impl<'js> FromJs<'js> for AnyURL {
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> js::Result<Self> {
        if let Ok(class) = Class::<JsUrl>::from_js(ctx, value.clone()) {
            return Ok(AnyURL::Url(class.borrow().clone()));
        }
        if let Ok(s) = StringArg::from_js(ctx, value.clone()) {
            return Ok(AnyURL::Str(s));
        }
        let received = match value.type_of() {
            Type::String => "string",
            Type::Object => "object",
            _ => "unknown",
        };
        Err(js::Error::new_from_js(received, "string or URL"))
    }
}

impl AnyURL {
    pub fn as_str(&self) -> String {
        match self {
            AnyURL::Str(s) => s.as_str().to_string(),
            AnyURL::Url(u) => u.as_str(),
        }
    }
}
