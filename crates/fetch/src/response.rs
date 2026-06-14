use crate::headers::Headers;
use rquickjs::class::{Trace, Tracer};
use rquickjs::{self as js, ArrayBuffer, Class, Ctx, JsLifetime, Value};

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct Response {
    status: u16,
    status_text: String,
    headers: Headers,
    url: String,
    body: Vec<u8>,
}

impl<'js> Trace<'js> for Response {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl Response {
    pub fn new(
        body: Vec<u8>,
        status: u16,
        status_text: String,
        headers: Headers,
        url: String,
    ) -> Self {
        Response {
            status,
            status_text,
            headers,
            url,
            body,
        }
    }
}

fn serde_value_to_js<'js>(ctx: &Ctx<'js>, val: &serde_json::Value) -> js::Result<Value<'js>> {
    match val {
        serde_json::Value::Null => Ok(Value::new_null(ctx.clone())),
        serde_json::Value::Bool(b) => Ok(Value::new_bool(ctx.clone(), *b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64().and_then(|i| i32::try_from(i).ok()) {
                Ok(Value::new_int(ctx.clone(), i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::new_float(ctx.clone(), f))
            } else {
                Ok(Value::new_null(ctx.clone()))
            }
        }
        serde_json::Value::String(s) => {
            js::String::from_str(ctx.clone(), s).map(|v| v.into_value())
        }
        serde_json::Value::Array(arr) => {
            let js_arr = js::Array::new(ctx.clone())?;
            for (i, item) in arr.iter().enumerate() {
                js_arr.set(i, serde_value_to_js(ctx, item)?)?;
            }
            Ok(js_arr.into_value())
        }
        serde_json::Value::Object(map) => {
            let obj = js::Object::new(ctx.clone())?;
            for (k, v) in map.iter() {
                obj.set(k.as_str(), serde_value_to_js(ctx, v)?)?;
            }
            Ok(obj.into_value())
        }
    }
}

#[js::methods]
impl Response {
    #[qjs(get)]
    fn ok(&self) -> bool {
        self.status >= 200 && self.status <= 299
    }

    #[qjs(get)]
    fn headers<'js>(&self, ctx: Ctx<'js>) -> js::Result<Class<'js, Headers>> {
        Class::instance(ctx, self.headers.clone())
    }

    #[qjs(get, rename = "type")]
    fn response_type(&self) -> &str {
        "default"
    }

    #[qjs(get, rename = "statusText")]
    fn status_text(&self) -> String {
        self.status_text.clone()
    }

    #[qjs(get)]
    fn url(&self) -> String {
        self.url.clone()
    }

    #[qjs()]
    async fn text<'js>(&self, ctx: Ctx<'js>) -> js::Result<js::String<'js>> {
        js::String::from_str(ctx, &String::from_utf8_lossy(&self.body))
    }

    #[qjs()]
    async fn json<'js>(&self, ctx: Ctx<'js>) -> js::Result<Value<'js>> {
        let val: serde_json::Value =
            serde_json::from_slice(&self.body).map_err(|_| js::Error::Exception)?;
        serde_value_to_js(&ctx, &val)
    }

    #[qjs(rename = "arrayBuffer")]
    fn to_array_buffer<'js>(&self, ctx: Ctx<'js>) -> js::Result<ArrayBuffer<'js>> {
        ArrayBuffer::from_source(ctx, self.body.clone())
    }

    #[qjs(rename = "clone")]
    fn clone_response<'js>(&self, ctx: Ctx<'js>) -> js::Result<Class<'js, Response>> {
        Class::instance(ctx, self.clone())
    }
}
