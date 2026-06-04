use std::sync::Arc;

use rquickjs::{
    self as js, Array, Class, Ctx, Function, JsLifetime, Object, Persistent, Result, Value,
    atom::PredefinedAtom,
    class::{Trace, Tracer},
    function::{Opt, Rest},
};

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct RsString {
    inner: Arc<str>,
    start: usize,
    end: usize,
}

impl<'js> Trace<'js> for RsString {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl RsString {
    fn get_slice(&self) -> &str {
        &self.inner[self.start..self.end]
    }

    fn byte_len(&self) -> usize {
        self.end - self.start
    }

    fn owned(s: String) -> Self {
        let len = s.len();
        RsString {
            inner: Arc::from(s.into_boxed_str()),
            start: 0,
            end: len,
        }
    }

    fn char_to_byte(&self, char_index: usize) -> Option<usize> {
        self.get_slice()
            .char_indices()
            .nth(char_index)
            .map(|(i, _)| i)
    }

    fn empty(&self) -> Self {
        RsString {
            inner: self.inner.clone(),
            start: self.start,
            end: self.start,
        }
    }

    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[js::methods]
impl RsString {
    #[qjs(get, rename = "length")]
    fn length(&self) -> usize {
        self.get_slice().chars().count()
    }

    #[qjs(rename = "at")]
    fn at_method<'js>(&self, ctx: Ctx<'js>, index: Opt<isize>) -> Result<Class<'js, RsString>> {
        let index = index.0.unwrap_or(0);
        let len = self.get_slice().chars().count() as isize;
        let mut idx = index;
        if idx < 0 {
            idx += len;
        }
        if idx < 0 || idx >= len {
            return Class::instance(ctx, self.empty());
        }
        if let Some(byte_idx) = self.char_to_byte(idx as usize) {
            let c = self.get_slice()[byte_idx..].chars().next().unwrap();
            Class::instance(
                ctx,
                RsString {
                    inner: self.inner.clone(),
                    start: self.start + byte_idx,
                    end: self.start + byte_idx + c.len_utf8(),
                },
            )
        } else {
            Class::instance(ctx, self.empty())
        }
    }

    #[qjs(rename = "charAt")]
    fn char_at<'js>(&self, ctx: Ctx<'js>, index: usize) -> Result<Class<'js, RsString>> {
        if let Some(byte_idx) = self.char_to_byte(index) {
            let c = self.get_slice()[byte_idx..].chars().next().unwrap();
            Class::instance(
                ctx,
                RsString {
                    inner: self.inner.clone(),
                    start: self.start + byte_idx,
                    end: self.start + byte_idx + c.len_utf8(),
                },
            )
        } else {
            Class::instance(ctx, self.empty())
        }
    }

    #[qjs(rename = "charCodeAt")]
    fn char_code_at(&self, index: usize) -> i32 {
        self.get_slice()
            .chars()
            .nth(index)
            .map(|c| c as u32 as i32)
            .unwrap_or(-1)
    }

    #[qjs(rename = "codePointAt")]
    fn code_point_at(&self, index: usize) -> i32 {
        self.get_slice()
            .chars()
            .nth(index)
            .map(|c| c as i32)
            .unwrap_or(-1)
    }

    fn substring<'js>(
        &self,
        ctx: Ctx<'js>,
        start: usize,
        end: Opt<usize>,
    ) -> Result<Class<'js, RsString>> {
        let end = end.0.unwrap_or(self.byte_len());
        let bound_end = end.min(self.byte_len());
        let bound_start = start.min(bound_end);
        Class::instance(
            ctx,
            RsString {
                inner: self.inner.clone(),
                start: self.start + bound_start,
                end: self.start + bound_end,
            },
        )
    }

    fn slice<'js>(
        &self,
        ctx: Ctx<'js>,
        start: isize,
        end: Opt<isize>,
    ) -> Result<Class<'js, RsString>> {
        let mut start = start;
        let len = self.byte_len() as isize;

        if start < 0 {
            start += len;
        }
        if start < 0 {
            start = 0;
        }
        if start > len {
            start = len;
        }

        let mut real_end_pos = len;
        if let Some(e) = end.0 {
            let mut e = e;
            if e < 0 {
                e += len;
            }
            if e < 0 {
                e = 0;
            }
            if e > len {
                e = len;
            }
            real_end_pos = e;
        }

        if real_end_pos < start {
            real_end_pos = start;
        }

        Class::instance(
            ctx,
            RsString {
                inner: self.inner.clone(),
                start: self.start + start as usize,
                end: self.start + real_end_pos as usize,
            },
        )
    }

    fn substr<'js>(
        &self,
        ctx: Ctx<'js>,
        start: isize,
        length: Opt<isize>,
    ) -> Result<Class<'js, RsString>> {
        let char_count = self.get_slice().chars().count() as isize;
        let mut s = start;
        if s < 0 {
            s += char_count;
        }
        if s < 0 {
            s = 0;
        }
        if s > char_count {
            s = char_count;
        }

        let len = if let Some(l) = length.0 {
            if l <= 0 {
                return Class::instance(ctx, self.empty());
            }
            l.min(char_count - s) as usize
        } else {
            (char_count - s) as usize
        };

        if len == 0 {
            return Class::instance(ctx, self.empty());
        }

        let s_usize = s as usize;
        let end_char = s_usize + len;
        let byte_start = self.char_to_byte(s_usize).unwrap_or(self.byte_len());
        let byte_end = self.char_to_byte(end_char).unwrap_or(self.byte_len());

        Class::instance(
            ctx,
            RsString {
                inner: self.inner.clone(),
                start: self.start + byte_start,
                end: self.start + byte_end,
            },
        )
    }

    fn trim<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        let slice = self.get_slice();

        let start_offset = slice.len() - slice.trim_start().len();
        let end_offset = slice.len() - slice.trim_end().len();

        Class::instance(
            ctx,
            RsString {
                inner: self.inner.clone(),
                start: self.start + start_offset,
                end: self.end - end_offset,
            },
        )
    }

    #[qjs(rename = "trimStart")]
    fn trim_start_method<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        let slice = self.get_slice();
        let start_offset = slice.len() - slice.trim_start().len();
        Class::instance(
            ctx,
            RsString {
                inner: self.inner.clone(),
                start: self.start + start_offset,
                end: self.end,
            },
        )
    }

    #[qjs(rename = "trimEnd")]
    fn trim_end_method<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        let slice = self.get_slice();
        let end_offset = slice.len() - slice.trim_end().len();
        Class::instance(
            ctx,
            RsString {
                inner: self.inner.clone(),
                start: self.start,
                end: self.end - end_offset,
            },
        )
    }

    // --- search methods ---

    #[qjs(rename = "indexOf")]
    fn index_of(&self, search: String, from_index: Opt<usize>) -> isize {
        let start_pos = from_index.0.unwrap_or(0);
        let slice = self.get_slice();

        if start_pos >= slice.len() {
            return -1;
        }

        if let Some(pos) = slice[start_pos..].find(&search) {
            (start_pos + pos) as isize
        } else {
            -1
        }
    }

    #[qjs(rename = "lastIndexOf")]
    fn last_index_of(&self, search: String, from_index: Opt<usize>) -> isize {
        let slice = self.get_slice();
        let max_pos = slice.len();
        let from = from_index.0.unwrap_or(max_pos);
        let from = from.min(max_pos);

        if from == 0 {
            if slice[..0].find(&search).is_some() {
                return 0;
            }
            return -1;
        }

        let search_end = from.min(slice.len());
        let search_area = &slice[..search_end];
        if let Some(pos) = search_area.rfind(&search) {
            pos as isize
        } else {
            -1
        }
    }

    fn includes(&self, search: String) -> bool {
        self.get_slice().contains(&search)
    }

    #[qjs(rename = "startsWith")]
    fn starts_with(&self, search: String) -> bool {
        self.get_slice().starts_with(&search)
    }

    #[qjs(rename = "endsWith")]
    fn ends_with(&self, search: String) -> bool {
        self.get_slice().ends_with(&search)
    }

    // --- transform / alloc methods ---

    fn concat<'js>(&self, ctx: Ctx<'js>, str1: js::String<'js>) -> Result<Class<'js, RsString>> {
        let s1 = str1.to_string()?;
        let mut result = self.get_slice().to_owned();
        result.push_str(&s1);
        Class::instance(ctx, RsString::owned(result))
    }

    fn repeat<'js>(&self, ctx: Ctx<'js>, count: usize) -> Result<Class<'js, RsString>> {
        if count == 0 || self.is_empty() {
            return Class::instance(ctx, self.empty());
        }
        let result = self.get_slice().repeat(count);
        Class::instance(ctx, RsString::owned(result))
    }

    #[qjs(rename = "padStart")]
    fn pad_start<'js>(
        &self,
        ctx: Ctx<'js>,
        target_length: usize,
        pad_string: Opt<js::String<'js>>,
    ) -> Result<Class<'js, RsString>> {
        let slice = self.get_slice();
        let char_len = slice.chars().count();
        if target_length <= char_len {
            return Class::instance(
                ctx,
                RsString {
                    inner: self.inner.clone(),
                    start: self.start,
                    end: self.end,
                },
            );
        }

        let pad = if let Some(ref s) = pad_string.0 {
            s.to_string().unwrap_or_else(|_| " ".to_string())
        } else {
            " ".to_string()
        };

        let pad_needed = target_length - char_len;
        let pad_chars: Vec<char> = pad.chars().collect();
        if pad_chars.is_empty() {
            return Class::instance(
                ctx,
                RsString {
                    inner: self.inner.clone(),
                    start: self.start,
                    end: self.end,
                },
            );
        }

        let mut result = String::with_capacity(target_length * 4);
        for i in 0..pad_needed {
            result.push(pad_chars[i % pad_chars.len()]);
        }
        result.push_str(slice);
        Class::instance(ctx, RsString::owned(result))
    }

    #[qjs(rename = "padEnd")]
    fn pad_end<'js>(
        &self,
        ctx: Ctx<'js>,
        target_length: usize,
        pad_string: Opt<js::String<'js>>,
    ) -> Result<Class<'js, RsString>> {
        let slice = self.get_slice();
        let char_len = slice.chars().count();
        if target_length <= char_len {
            return Class::instance(
                ctx,
                RsString {
                    inner: self.inner.clone(),
                    start: self.start,
                    end: self.end,
                },
            );
        }

        let pad = if let Some(ref s) = pad_string.0 {
            s.to_string().unwrap_or_else(|_| " ".to_string())
        } else {
            " ".to_string()
        };

        let pad_needed = target_length - char_len;
        let pad_chars: Vec<char> = pad.chars().collect();
        if pad_chars.is_empty() {
            return Class::instance(
                ctx,
                RsString {
                    inner: self.inner.clone(),
                    start: self.start,
                    end: self.end,
                },
            );
        }

        let mut result = String::with_capacity(target_length * 4);
        result.push_str(slice);
        for i in 0..pad_needed {
            result.push(pad_chars[i % pad_chars.len()]);
        }
        Class::instance(ctx, RsString::owned(result))
    }

    #[qjs(rename = "toLowerCase")]
    fn to_lower_case<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        Class::instance(ctx, RsString::owned(self.get_slice().to_lowercase()))
    }

    #[qjs(rename = "toUpperCase")]
    fn to_upper_case<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        Class::instance(ctx, RsString::owned(self.get_slice().to_uppercase()))
    }

    #[qjs(rename = "toLocaleLowerCase")]
    fn to_locale_lower_case<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        self.to_lower_case(ctx)
    }

    #[qjs(rename = "toLocaleUpperCase")]
    fn to_locale_upper_case<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, RsString>> {
        self.to_upper_case(ctx)
    }

    #[qjs(rename = "localeCompare")]
    fn locale_compare(&self, other: String) -> i32 {
        let a = self.get_slice();
        if a == other {
            0
        } else if a < other.as_str() {
            -1
        } else {
            1
        }
    }

    fn normalize<'js>(
        &self,
        ctx: Ctx<'js>,
        form: Opt<js::String<'js>>,
    ) -> Result<Class<'js, RsString>> {
        use unicode_normalization::UnicodeNormalization;

        let form_str = form.0.and_then(|s| s.to_string().ok()).unwrap_or_default();

        let result = match form_str.as_str() {
            "NFC" | "" => self.get_slice().chars().nfc().collect(),
            "NFD" => self.get_slice().chars().nfd().collect(),
            "NFKC" => self.get_slice().chars().nfkc().collect(),
            "NFKD" => self.get_slice().chars().nfkd().collect(),
            _ => self.get_slice().chars().nfc().collect(),
        };

        Class::instance(ctx, RsString::owned(result))
    }

    fn replace<'js>(
        &self,
        ctx: Ctx<'js>,
        search: Value<'js>,
        replacement: Value<'js>,
    ) -> Result<Class<'js, RsString>> {
        let func: Function =
            ctx.eval::<Function, _>("(s, search, replacement) => s.replace(search, replacement)")?;
        let self_str = js::String::from_str(ctx.clone(), self.get_slice())?;
        let result: js::String = func.call((self_str, search, replacement))?;
        let result_str = result.to_string()?;
        Class::instance(ctx, RsString::owned(result_str))
    }

    #[qjs(rename = "replaceAll")]
    fn replace_all<'js>(
        &self,
        ctx: Ctx<'js>,
        search: Value<'js>,
        replacement: Value<'js>,
    ) -> Result<Class<'js, RsString>> {
        let func: Function = ctx
            .eval::<Function, _>("(s, search, replacement) => s.replaceAll(search, replacement)")?;
        let self_str = js::String::from_str(ctx.clone(), self.get_slice())?;
        let result: js::String = func.call((self_str, search, replacement))?;
        let result_str = result.to_string()?;
        Class::instance(ctx, RsString::owned(result_str))
    }

    fn search<'js>(&self, ctx: Ctx<'js>, regexp: Value<'js>) -> Result<i32> {
        let func: Function = ctx.eval::<Function, _>("(s, r) => s.search(r)")?;
        let self_str = js::String::from_str(ctx, self.get_slice())?;
        func.call((self_str, regexp))
    }

    #[qjs(rename = "match")]
    fn match_method<'js>(&self, ctx: Ctx<'js>, regexp: Value<'js>) -> Result<Value<'js>> {
        let func: Function = ctx.eval::<Function, _>("(s, r) => s.match(r)")?;
        let self_str = js::String::from_str(ctx, self.get_slice())?;
        func.call((self_str, regexp))
    }

    #[qjs(rename = "matchAll")]
    fn match_all_method<'js>(&self, ctx: Ctx<'js>, regexp: Value<'js>) -> Result<Value<'js>> {
        let func: Function = ctx.eval::<Function, _>("(s, r) => s.matchAll(r)")?;
        let self_str = js::String::from_str(ctx, self.get_slice())?;
        func.call((self_str, regexp))
    }

    fn split<'js>(
        &self,
        ctx: Ctx<'js>,
        separator: Value<'js>,
        limit: Opt<Value<'js>>,
    ) -> Result<Array<'js>> {
        let self_str = js::String::from_str(ctx.clone(), self.get_slice())?;

        let limit_num: Option<usize> = limit.0.and_then(|v| {
            if v.is_undefined() {
                None
            } else {
                js::FromJs::from_js(&ctx, v).ok()
            }
        });

        let js_arr: Array = if let Some(lim) = limit_num {
            let func: Function = ctx.eval::<Function, _>("(s, sep, lim) => s.split(sep, lim)")?;
            func.call((self_str, separator, lim))?
        } else {
            let func: Function = ctx.eval::<Function, _>("(s, sep) => s.split(sep)")?;
            func.call((self_str, separator))?
        };

        Ok(js_arr)
    }

    // --- primitives + symbols ---

    #[qjs(rename = "toString")]
    fn to_string_method<'js>(&self, ctx: Ctx<'js>) -> Result<js::String<'js>> {
        js::String::from_str(ctx, self.get_slice())
    }

    #[qjs(rename = PredefinedAtom::ValueOf)]
    fn value_of<'js>(&self, ctx: Ctx<'js>) -> Result<js::String<'js>> {
        js::String::from_str(ctx, self.get_slice())
    }

    #[qjs(rename = PredefinedAtom::SymbolToPrimitive)]
    fn to_primitive<'js>(&self, ctx: Ctx<'js>, _hint: js::String<'js>) -> Result<js::String<'js>> {
        js::String::from_str(ctx, self.get_slice())
    }

    #[qjs(rename = PredefinedAtom::ToJSON)]
    fn to_json<'js>(&self, ctx: Ctx<'js>) -> Result<js::String<'js>> {
        js::String::from_str(ctx, self.get_slice())
    }

    #[qjs(rename = PredefinedAtom::SymbolIterator)]
    fn symbol_iterator<'js>(&self, ctx: Ctx<'js>) -> Result<Object<'js>> {
        let self_str = js::String::from_str(ctx.clone(), self.get_slice())?;
        let func: Function = ctx.eval::<Function, _>("(s) => s[Symbol.iterator]()")?;
        func.call((self_str,))
    }
}

#[rquickjs::function]
fn from_char_code<'js>(ctx: Ctx<'js>, codes: Rest<u32>) -> Result<Class<'js, RsString>> {
    let s: String = codes.0.iter().filter_map(|&c| char::from_u32(c)).collect();
    Class::instance(ctx, RsString::owned(s))
}

#[rquickjs::function]
fn from_code_point<'js>(ctx: Ctx<'js>, points: Rest<u32>) -> Result<Class<'js, RsString>> {
    let s: String = points.0.iter().filter_map(|&c| char::from_u32(c)).collect();
    Class::instance(ctx, RsString::owned(s))
}

// ── InternPool ──

pub struct InternPool {
    get_or_create: Persistent<Function<'static>>,
}

// SAFETY: contains only Persistent (type-erased, context-independent)
unsafe impl<'js> JsLifetime<'js> for InternPool {
    type Changed<'to> = InternPool;
}

#[rquickjs::function]
fn from_string_impl<'js>(ctx: Ctx<'js>, v: Opt<Value<'js>>) -> Result<Class<'js, RsString>> {
    let val = match v.0 {
        None => {
            return Class::instance(ctx, RsString::owned(String::new()));
        }
        Some(val) => val,
    };

    let factory_ctx = ctx.clone();
    let pool_fn: Function = {
        let guard = factory_ctx
            .userdata::<InternPool>()
            .expect("InternPool not initialized in context userdata");
        guard.get_or_create.clone().restore(&factory_ctx)?
    };

    let factory = Function::new(
        factory_ctx.clone(),
        move |key: String| -> Result<Class<'js, RsString>> {
            Class::instance(factory_ctx.clone(), RsString::owned(key))
        },
    )?;

    pool_fn.call((val, factory))
}

pub fn init_rs_string<'js>(ctx: &Ctx<'js>) -> Result<()> {
    Class::<RsString>::define(&ctx.globals())?;

    let proto =
        Class::<RsString>::prototype(ctx)?.expect("RsString prototype should exist after define");

    let ctor = Object::new(ctx.clone())?;
    ctor.set("prototype", proto)?;
    ctor.set("fromString", js_from_string_impl)?;
    ctor.set("fromCharCode", js_from_char_code)?;
    ctor.set("fromCodePoint", js_from_code_point)?;

    ctx.globals().set("RsString", ctor)?;

    let pool_fn: Function = ctx.eval::<Function, _>(
        r#"(function() {
            const map = new Map();
            const registry = new FinalizationRegistry((key) => map.delete(key));
            return (val, factory) => {
                const key = String(val);
                const ref = map.get(key);
                if (ref) {
                    const obj = ref.deref();
                    if (obj) return obj;
                    map.delete(key);
                }
                const obj = factory(key);
                map.set(key, new WeakRef(obj));
                registry.register(obj, key);
                return obj;
            };
        })()"#,
    )?;

    ctx.store_userdata(InternPool {
        get_or_create: Persistent::save(ctx, pool_fn),
    })
    .ok();

    tracing::info!("RsString class defined");
    Ok(())
}
