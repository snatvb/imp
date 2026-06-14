use std::cell::RefCell;
use std::rc::Rc;

use rquickjs::class::{Trace, Tracer};
use rquickjs::function::Opt;
use rquickjs::{self as js, Ctx, JsLifetime};

use crate::error::Error;
use crate::search_params::UrlSearchParams;
use js_core::error::JsError as _;

fn invalid_url<'js>(ctx: &Ctx<'js>, msg: String) -> js::Error {
    Error::InvalidUrl(msg).into_exception(ctx)
}

fn invalid_base_url<'js>(ctx: &Ctx<'js>, msg: String) -> js::Error {
    Error::InvalidBaseUrl(msg).into_exception(ctx)
}

#[js::class]
#[qjs(rename = "URL")]
#[derive(JsLifetime, Clone)]
pub struct UrlUrl {
    #[qjs(skip_trace)]
    inner: Rc<RefCell<url::Url>>,
    #[qjs(skip_trace)]
    search_params: UrlSearchParams,
}

impl<'js> Trace<'js> for UrlUrl {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::methods]
impl UrlUrl {
    #[qjs(constructor)]
    fn construct<'js>(ctx: Ctx<'js>, input: String, base: Opt<String>) -> js::Result<UrlUrl> {
        let parsed = match base.0 {
            Some(b) => {
                let base_url =
                    url::Url::parse(&b).map_err(|e| invalid_base_url(&ctx, e.to_string()))?;
                url::Url::options()
                    .base_url(Some(&base_url))
                    .parse(&input)
                    .map_err(|e| invalid_url(&ctx, e.to_string()))?
            }
            None => url::Url::parse(&input).map_err(|e| invalid_url(&ctx, e.to_string()))?,
        };

        let inner = Rc::new(RefCell::new(parsed));
        let search_params = UrlSearchParams::from_url(inner.clone());

        Ok(UrlUrl {
            inner,
            search_params,
        })
    }

    #[qjs(get, rename = "href")]
    fn href(&self) -> String {
        String::from(self.inner.borrow().as_str())
    }

    #[qjs(set, rename = "href")]
    fn set_href<'js>(&self, ctx: Ctx<'js>, value: String) -> js::Result<()> {
        let new_url = url::Url::parse(&value).map_err(|e| invalid_url(&ctx, e.to_string()))?;
        *self.inner.borrow_mut() = new_url;
        let query = self.inner.borrow().query().unwrap_or("").to_string();
        *self.search_params.pairs.borrow_mut() = crate::search_params::parse_query_string(&query);
        Ok(())
    }

    #[qjs(get)]
    fn origin(&self) -> String {
        self.inner.borrow().origin().ascii_serialization()
    }

    #[qjs(get, rename = "protocol")]
    fn protocol(&self) -> String {
        format!("{}:", self.inner.borrow().scheme())
    }

    #[qjs(set, rename = "protocol")]
    fn set_protocol(&self, value: String) {
        let scheme = value.trim_end_matches(':');
        let _ = self.inner.borrow_mut().set_scheme(scheme);
    }

    #[qjs(get, rename = "username")]
    fn username(&self) -> String {
        self.inner.borrow().username().to_string()
    }

    #[qjs(set, rename = "username")]
    fn set_username(&self, value: String) {
        let _ = self.inner.borrow_mut().set_username(&value);
    }

    #[qjs(get, rename = "password")]
    fn password(&self) -> String {
        self.inner.borrow().password().unwrap_or("").to_string()
    }

    #[qjs(set, rename = "password")]
    fn set_password(&self, value: String) {
        let _ = self.inner.borrow_mut().set_password(Some(&value));
    }

    #[qjs(get, rename = "host")]
    fn host(&self) -> String {
        self.inner.borrow().host_str().unwrap_or("").to_string()
    }

    #[qjs(set, rename = "host")]
    fn set_host(&self, value: String) {
        let _ = self.inner.borrow_mut().set_host(Some(&value));
    }

    #[qjs(get, rename = "hostname")]
    fn hostname(&self) -> String {
        self.inner.borrow().host_str().unwrap_or("").to_string()
    }

    #[qjs(set, rename = "hostname")]
    fn set_hostname(&self, value: String) {
        let _ = self.inner.borrow_mut().set_host(Some(&value));
    }

    #[qjs(get, rename = "port")]
    fn port(&self) -> String {
        match self.inner.borrow().port() {
            Some(p) => p.to_string(),
            None => String::new(),
        }
    }

    #[qjs(set, rename = "port")]
    fn set_port(&self, value: String) {
        if value.is_empty() {
            self.inner.borrow_mut().set_port(None).ok();
        } else if let Ok(port) = value.parse::<u16>() {
            self.inner.borrow_mut().set_port(Some(port)).ok();
        }
    }

    #[qjs(get, rename = "pathname")]
    fn pathname(&self) -> String {
        self.inner.borrow().path().to_string()
    }

    #[qjs(set, rename = "pathname")]
    fn set_pathname(&self, value: String) {
        self.inner.borrow_mut().set_path(&value);
    }

    #[qjs(get, rename = "search")]
    fn search(&self) -> String {
        match self.inner.borrow().query() {
            Some(q) if !q.is_empty() => {
                let mut s = String::with_capacity(q.len() + 1);
                s.push('?');
                s.push_str(q);
                s
            }
            _ => String::new(),
        }
    }

    #[qjs(set, rename = "search")]
    fn set_search(&self, value: String) {
        let q = value.strip_prefix('?').unwrap_or(&value);
        *self.search_params.pairs.borrow_mut() = crate::search_params::parse_query_string(q);
        self.search_params.sync_to_url();
    }

    #[qjs(get, rename = "hash")]
    fn hash(&self) -> String {
        match self.inner.borrow().fragment() {
            Some(f) if !f.is_empty() => {
                let mut s = String::with_capacity(f.len() + 1);
                s.push('#');
                s.push_str(f);
                s
            }
            _ => String::new(),
        }
    }

    #[qjs(set, rename = "hash")]
    fn set_hash(&self, value: String) {
        let f = value.strip_prefix('#').unwrap_or(&value);
        self.inner
            .borrow_mut()
            .set_fragment(if f.is_empty() { None } else { Some(f) });
    }

    #[qjs(get, rename = "searchParams")]
    fn search_params(&self) -> UrlSearchParams {
        self.search_params.clone()
    }

    #[qjs(rename = "toString")]
    fn to_string_js(&self) -> String {
        String::from(self.inner.borrow().as_str())
    }

    fn to_json(&self) -> String {
        String::from(self.inner.borrow().as_str())
    }

    #[qjs(static, rename = "canParse")]
    fn can_parse(input: String, base: Opt<String>) -> bool {
        match base.0 {
            Some(b) => url::Url::options()
                .base_url(url::Url::parse(&b).ok().as_ref())
                .parse(&input)
                .is_ok(),
            None => url::Url::parse(&input).is_ok(),
        }
    }

    #[qjs(static)]
    fn parse<'js>(ctx: Ctx<'js>, input: String, base: Opt<String>) -> js::Result<UrlUrl> {
        Self::construct(ctx, input, base)
    }
}
