use rquickjs::class::{Trace, Tracer};
use rquickjs::function::Opt;
use rquickjs::{self as js, Ctx, JsLifetime, Object, Result};

use crate::headers::Headers;
use crate::url::AnyURL;
use js_core::abort::AbortSignal;
use js_core::utils::StringArg;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct Request {
    method: String,
    url: String,
    headers: Headers,
    body: Option<String>,
    signal: Option<AbortSignal>,
}

impl<'js> Trace<'js> for Request {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::methods]
impl Request {
    #[qjs(constructor)]
    fn construct<'js>(_ctx: Ctx<'js>, url: AnyURL, init: Opt<Object<'js>>) -> Result<Request> {
        let mut method = String::from("GET");
        let mut headers = Headers::new();
        let mut body: Option<String> = None;
        let mut signal: Option<AbortSignal> = None;

        if let Some(obj) = init.0 {
            if let Ok(m) = obj.get::<_, StringArg>("method") {
                method = m.as_str().to_string();
            }
            if let Ok(h) = obj.get::<_, Headers>("headers") {
                headers = h;
            }
            if let Ok(b) = obj.get::<_, Option<String>>("body") {
                body = b;
            }
            if let Ok(s) = obj.get::<_, AbortSignal>("signal") {
                signal = Some(s);
            }
        }

        Ok(Request {
            method,
            url: url.as_str().to_string(),
            headers,
            body,
            signal,
        })
    }

    #[qjs(get)]
    fn method(&self) -> String {
        self.method.clone()
    }

    #[qjs(get)]
    fn url(&self) -> String {
        self.url.clone()
    }

    #[qjs(get)]
    fn headers(&self) -> Headers {
        self.headers.clone()
    }

    #[qjs(get)]
    fn body(&self) -> Option<String> {
        self.body.clone()
    }

    #[qjs(get)]
    fn signal(&self) -> Option<AbortSignal> {
        self.signal.clone()
    }

    fn clone(&self) -> Request {
        Clone::clone(self)
    }
}
