use std::cell::RefCell;
use std::rc::Rc;

use rquickjs::class::{Trace, Tracer};
use rquickjs::{self as js, Ctx, JsLifetime, Result};

#[js::class]
#[qjs(rename = "URLSearchParams")]
#[derive(JsLifetime, Clone)]
pub struct UrlSearchParams {
    #[qjs(skip_trace)]
    pub pairs: Rc<RefCell<Vec<(String, String)>>>,
    #[qjs(skip_trace)]
    url_inner: Option<Rc<RefCell<url::Url>>>,
}

impl<'js> Trace<'js> for UrlSearchParams {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl UrlSearchParams {
    pub fn from_query_string(query: &str) -> Self {
        let pairs = parse_query_string(query);
        UrlSearchParams {
            pairs: Rc::new(RefCell::new(pairs)),
            url_inner: None,
        }
    }

    pub fn from_url(url_inner: Rc<RefCell<url::Url>>) -> Self {
        let query = url_inner.borrow().query().unwrap_or("").to_string();
        let pairs = parse_query_string(&query);
        UrlSearchParams {
            pairs: Rc::new(RefCell::new(pairs)),
            url_inner: Some(url_inner),
        }
    }

    pub fn serialize(&self) -> String {
        serialize_pairs(&self.pairs.borrow())
    }

    pub(crate) fn sync_to_url(&self) {
        if let Some(ref url) = self.url_inner {
            let pairs = self.pairs.borrow();
            {
                let mut u = url.borrow_mut();
                let mut qp = u.query_pairs_mut();
                qp.clear();
                for (k, v) in pairs.iter() {
                    qp.append_pair(k, v);
                }
            }
        }
    }
}

pub fn parse_query_string(query: &str) -> Vec<(String, String)> {
    let q = query.strip_prefix('?').unwrap_or(query);
    if q.is_empty() {
        return Vec::new();
    }
    let mut pairs = Vec::with_capacity(q.matches('&').count() + 1);
    for pair in q.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").to_string();
        pairs.push((key, value));
    }
    pairs
}

fn serialize_pairs(pairs: &[(String, String)]) -> String {
    if pairs.is_empty() {
        return String::new();
    }
    let pairs_sum = pairs
        .iter()
        .map(|(k, v)| k.len() + v.len() + 1)
        .sum::<usize>();
    let estimated = pairs_sum + pairs.len() - 1;
    let mut buf = String::with_capacity(estimated);
    for (i, (k, v)) in pairs.iter().enumerate() {
        if i > 0 {
            buf.push('&');
        }
        buf.push_str(k);
        buf.push('=');
        buf.push_str(v);
    }
    buf
}

#[js::methods]
impl UrlSearchParams {
    #[qjs(constructor)]
    fn construct(init: Option<String>) -> UrlSearchParams {
        match init {
            Some(s) => UrlSearchParams::from_query_string(&s),
            None => UrlSearchParams {
                pairs: Rc::new(RefCell::new(Vec::new())),
                url_inner: None,
            },
        }
    }

    fn append(&self, name: String, value: String) {
        self.pairs.borrow_mut().push((name, value));
        self.sync_to_url();
    }

    fn delete(&self, name: String) {
        self.pairs.borrow_mut().retain(|(k, _)| k != &name);
        self.sync_to_url();
    }

    fn get(&self, name: String) -> Option<String> {
        self.pairs
            .borrow()
            .iter()
            .find(|(k, _)| k == &name)
            .map(|(_, v)| v.clone())
    }

    fn get_all(&self, name: String) -> Vec<String> {
        self.pairs
            .borrow()
            .iter()
            .filter(|(k, _)| k == &name)
            .map(|(_, v)| v.clone())
            .collect()
    }

    fn has(&self, name: String) -> bool {
        self.pairs.borrow().iter().any(|(k, _)| k == &name)
    }

    fn set(&self, name: String, value: String) {
        let mut pairs = self.pairs.borrow_mut();
        if let Some(pos) = pairs.iter().position(|(k, _)| k == &name) {
            pairs[pos].1 = value;
            let mut i = pos + 1;
            while i < pairs.len() {
                if pairs[i].0 == name {
                    pairs.remove(i);
                } else {
                    i += 1;
                }
            }
        } else {
            pairs.push((name, value));
        }
        drop(pairs);
        self.sync_to_url();
    }

    fn sort(&self) {
        self.pairs.borrow_mut().sort_by(|a, b| a.0.cmp(&b.0));
        self.sync_to_url();
    }

    #[qjs(get, rename = "size")]
    fn size(&self) -> usize {
        self.pairs.borrow().len()
    }

    #[qjs(rename = "toString")]
    fn to_string_js(&self) -> String {
        self.serialize()
    }

    fn keys(&self) -> Vec<String> {
        self.pairs.borrow().iter().map(|(k, _)| k.clone()).collect()
    }

    fn values(&self) -> Vec<String> {
        self.pairs.borrow().iter().map(|(_, v)| v.clone()).collect()
    }

    fn entries(&self) -> Vec<Vec<String>> {
        self.pairs
            .borrow()
            .iter()
            .map(|(k, v)| vec![k.clone(), v.clone()])
            .collect()
    }

    fn for_each<'js>(&self, _ctx: Ctx<'js>, callback: js::Function<'js>) -> Result<()> {
        let pairs = self.pairs.borrow();
        for (k, v) in pairs.iter() {
            callback.call::<_, ()>((k.as_str(), v.as_str()))?;
        }
        Ok(())
    }
}
