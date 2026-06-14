use rquickjs::class::{Trace, Tracer};
use rquickjs::function::Opt;
use rquickjs::{self as js, Array, Ctx, Function, JsLifetime, Object, Result};

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct Headers {
    inner: Vec<(String, String)>,
}

impl<'js> Trace<'js> for Headers {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

impl Headers {
    pub fn new() -> Self {
        Headers { inner: Vec::new() }
    }

    fn find(&self, name: &str) -> Option<usize> {
        self.inner
            .iter()
            .position(|(k, _)| k.eq_ignore_ascii_case(name))
    }

    fn lower(name: &str) -> String {
        name.to_lowercase()
    }
}

#[js::methods]
impl Headers {
    #[qjs(constructor)]
    fn construct<'js>(_ctx: Ctx<'js>, init: Opt<Object<'js>>) -> Result<Headers> {
        let mut headers = Headers { inner: Vec::new() };

        if let Some(obj) = init.0 {
            for prop in obj.props::<String, String>() {
                let (k, v) = prop?;
                headers.inner.push((Self::lower(&k), v));
            }
        }

        Ok(headers)
    }

    fn get<'js>(&self, ctx: Ctx<'js>, name: String) -> Result<js::Value<'js>> {
        match self.find(&name) {
            Some(i) => Ok(js::String::from_str(ctx, &self.inner[i].1)?.into_value()),
            None => Ok(js::Value::new_null(ctx)),
        }
    }

    pub fn set(&mut self, name: String, value: String) {
        let lower = Self::lower(&name);
        match self.find(&name) {
            Some(i) => self.inner[i] = (lower, value),
            None => self.inner.push((lower, value)),
        }
    }

    pub fn append(&mut self, name: String, value: String) {
        self.inner.push((Self::lower(&name), value));
    }

    fn has(&self, name: String) -> bool {
        self.find(&name).is_some()
    }

    fn delete(&mut self, name: String) {
        if let Some(i) = self.find(&name) {
            self.inner.remove(i);
        }
    }

    fn keys<'js>(&self, ctx: Ctx<'js>) -> Result<Array<'js>> {
        let arr = Array::new(ctx.clone())?;
        for (i, (k, _)) in self.inner.iter().enumerate() {
            arr.set(i, js::String::from_str(ctx.clone(), k)?)?;
        }
        Ok(arr)
    }

    fn values<'js>(&self, ctx: Ctx<'js>) -> Result<Array<'js>> {
        let arr = Array::new(ctx.clone())?;
        for (i, (_, v)) in self.inner.iter().enumerate() {
            arr.set(i, js::String::from_str(ctx.clone(), v)?)?;
        }
        Ok(arr)
    }

    fn entries<'js>(&self, ctx: Ctx<'js>) -> Result<Array<'js>> {
        let arr = Array::new(ctx.clone())?;
        for (i, (k, v)) in self.inner.iter().enumerate() {
            let pair = Array::new(ctx.clone())?;
            pair.set(0, js::String::from_str(ctx.clone(), k)?)?;
            pair.set(1, js::String::from_str(ctx.clone(), v)?)?;
            arr.set(i, pair)?;
        }
        Ok(arr)
    }

    fn for_each<'js>(&self, ctx: Ctx<'js>, callback: Function<'js>) -> Result<()> {
        for (k, v) in &self.inner {
            let key = js::String::from_str(ctx.clone(), k)?;
            let val = js::String::from_str(ctx.clone(), v)?;
            callback.call::<_, ()>((key, val))?;
        }
        Ok(())
    }
}
