use std::collections::BTreeMap;

use js_core::error::JsError as _;
use js_core::js;
use js_core::js::function::Opt;
use js_core::js::{Ctx, FromJs, Object, String as JsString, Value};
use js_core::utils::StringArg;

use crate::error::EnvError;
use crate::expand::expand_inner;

#[derive(Debug, Clone)]
pub struct DotenvOptions {
    expand: bool,
    vars: std::collections::HashMap<String, String>,
}

impl Default for DotenvOptions {
    fn default() -> Self {
        DotenvOptions {
            expand: true,
            vars: std::collections::HashMap::new(),
        }
    }
}

impl<'js> FromJs<'js> for DotenvOptions {
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> js_core::js::Result<Self> {
        let mut opts = DotenvOptions::default();
        if let Some(obj) = value.as_object() {
            if let Ok(Some(b)) = obj.get::<_, Option<bool>>("expand") {
                opts.expand = b;
            }
            for item in obj.props::<String, Value>() {
                let (k, v) = item?;
                if k == "expand" {
                    continue;
                }
                if let Some(s) = v.as_string() {
                    let s = s.to_string()?;
                    opts.vars.insert(k, s);
                } else if let Some(i) = v.as_int() {
                    opts.vars.insert(k, i.to_string());
                } else if let Some(f) = v.as_float() {
                    opts.vars.insert(k, f.to_string());
                } else if let Some(b) = v.as_bool() {
                    opts.vars.insert(k, b.to_string());
                }
            }
        }
        let _ = ctx;
        Ok(opts)
    }
}

#[derive(Debug, Clone)]
enum RawPart {
    Lit(String),
    Var(String),
    BraceVar(String),
}

fn tokenize_unquoted(s: &str) -> Vec<RawPart> {
    let bytes = s.as_bytes();
    let mut parts = Vec::new();
    let mut lit = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            let next = bytes[i + 1];
            if next == b'$' {
                if !lit.is_empty() {
                    parts.push(RawPart::Lit(std::mem::take(&mut lit)));
                }
                parts.push(RawPart::Var("$".to_string()));
                i += 2;
                continue;
            }
            lit.push('\\');
            lit.push(next as char);
            i += 2;
            continue;
        }
        if b == b'$' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                let after = &s[i + 2..];
                if let Some(end) = after.find('}') {
                    if !lit.is_empty() {
                        parts.push(RawPart::Lit(std::mem::take(&mut lit)));
                    }
                    let name = after[..end].to_string();
                    if name.is_empty() {
                        lit.push('$');
                        lit.push('{');
                        lit.push('}');
                    } else {
                        parts.push(RawPart::BraceVar(name));
                    }
                    i += 2 + end + 1;
                    continue;
                }
            }
            let after = &s[i + 1..];
            let name: String = after
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if name.is_empty() {
                lit.push('$');
                i += 1;
                continue;
            }
            if !lit.is_empty() {
                parts.push(RawPart::Lit(std::mem::take(&mut lit)));
            }
            let name_len = name.len();
            parts.push(RawPart::Var(name));
            i += 1 + name_len;
            continue;
        }
        if b < 0x80 {
            lit.push(b as char);
            i += 1;
        } else {
            let rest = &s[i..];
            let ch = rest.chars().next().unwrap();
            lit.push(ch);
            i += ch.len_utf8();
        }
    }
    if !lit.is_empty() {
        parts.push(RawPart::Lit(lit));
    }
    parts
}

fn unescape_double(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'n' => {
                    out.push('\n');
                    i += 2;
                }
                b'r' => {
                    out.push('\r');
                    i += 2;
                }
                b't' => {
                    out.push('\t');
                    i += 2;
                }
                b'\\' => {
                    out.push('\\');
                    i += 2;
                }
                b'"' => {
                    out.push('"');
                    i += 2;
                }
                b'$' => {
                    out.push('$');
                    i += 2;
                }
                b'\'' => {
                    out.push('\'');
                    i += 2;
                }
                c => {
                    out.push('\\');
                    out.push(c as char);
                    i += 2;
                }
            }
        } else if b < 0x80 {
            out.push(b as char);
            i += 1;
        } else {
            let rest = &s[i..];
            let ch = rest.chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn unescape_single_keep(s: &str) -> String {
    s.replace("\\'", "'").replace("\\\\", "\\")
}

fn parse_value(raw: &str) -> (String, bool) {
    let s = raw;
    if s.starts_with('"') {
        if let Some(end) = find_unescaped(s, 1, '"') {
            let inner = &s[1..end];
            return (unescape_double(inner), true);
        }
        return (s.to_string(), false);
    }
    if s.starts_with('\'') {
        let rest = s.strip_prefix('\'').unwrap();
        if let Some(end) = rest.find('\'') {
            let inner = &rest[..end];
            return (unescape_single_keep(inner), true);
        }
        return (s.to_string(), false);
    }
    (unescape_unquoted(&parse_unquoted(s)), false)
}

fn parse_unquoted(raw: &str) -> String {
    let s = raw.trim();
    if let Some(index) = s.find(['#', ';']) {
        s[..index].trim_end().to_string()
    } else {
        s.to_string()
    }
}

fn unescape_unquoted(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            let next = bytes[i + 1];
            if next == b'$' || next == b'"' || next == b'\\' {
                out.push(next as char);
                i += 2;
                continue;
            }
            out.push('\\');
            i += 1;
            continue;
        }
        if b < 0x80 {
            out.push(b as char);
            i += 1;
        } else {
            let rest = &s[i..];
            let ch = rest.chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn find_unescaped(s: &str, start: usize, target: char) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = start;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            i += 2;
            continue;
        }
        if b < 0x80 {
            if b as char == target {
                return Some(i);
            }
            i += 1;
        } else {
            let rest = &s[i..];
            let ch = rest.chars().next().unwrap();
            if ch == target {
                return Some(i);
            }
            i += ch.len_utf8();
        }
    }
    None
}

pub(crate) fn parse_with_vars(
    input: &str,
    do_expand: bool,
    extra_vars: &std::collections::HashMap<String, String>,
) -> Result<BTreeMap<String, String>, EnvError> {
    let mut out = parse_raw(input)?;
    if do_expand {
        let mut combined: std::collections::HashMap<String, String> = extra_vars.clone();
        for (k, v) in &out {
            combined.insert(k.clone(), v.clone());
        }
        let mut resolved: BTreeMap<String, String> = BTreeMap::new();
        for (k, v) in &out {
            let mut visiting = Vec::new();
            let expanded = expand_value(v, &combined, &mut visiting)?;
            resolved.insert(k.clone(), expanded);
        }
        out = resolved;
    }
    Ok(out)
}

fn expand_value(
    raw: &str,
    vars: &std::collections::HashMap<String, String>,
    visiting: &mut Vec<String>,
) -> Result<String, EnvError> {
    if raw.starts_with('"') {
        if let Some(end) = find_unescaped(raw, 1, '"') {
            let inner = &raw[1..end];
            let unescaped = unescape_double(inner);
            return expand_inner(&unescaped, vars, visiting);
        }
        return Ok(raw.to_string());
    }
    if raw.starts_with('\'') {
        return Ok(raw.to_string());
    }
    let parts = tokenize_unquoted(raw);
    if parts.iter().all(|p| matches!(p, RawPart::Lit(_))) {
        let mut s = String::new();
        for p in parts {
            if let RawPart::Lit(l) = p {
                s.push_str(&l);
            }
        }
        return Ok(s);
    }
    let mut buf = String::new();
    for p in parts {
        match p {
            RawPart::Lit(l) => buf.push_str(&l),
            RawPart::Var(name) | RawPart::BraceVar(name) => {
                let val = resolve_dotenv_var(&name, vars, visiting)?;
                buf.push_str(&val);
            }
        }
    }
    Ok(buf)
}

fn parse_raw(input: &str) -> Result<BTreeMap<String, String>, EnvError> {
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");

    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for raw_line in normalized.split('\n') {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        let mut body = line;
        if let Some(rest) = body.strip_prefix("export ") {
            body = rest.trim_start();
        } else if let Some(rest) = body.strip_prefix("export\t") {
            body = rest.trim_start();
        }
        let Some(eq) = body.find('=') else { continue };
        let key = body[..eq].trim().to_string();
        if key.is_empty() {
            continue;
        }
        let value_part = body[eq + 1..].trim_start();
        let (raw_value, _quoted) = parse_value(value_part);
        out.insert(key, raw_value);
    }
    Ok(out)
}

fn resolve_dotenv_var(
    name: &str,
    vars: &std::collections::HashMap<String, String>,
    visiting: &mut Vec<String>,
) -> Result<String, EnvError> {
    if visiting.iter().any(|v| v == name) {
        return Err(EnvError::Expand(format!(
            "circular variable reference involving '{name}'"
        )));
    }
    let Some(val) = vars.get(name) else {
        return Ok(format!("${name}"));
    };
    visiting.push(name.to_string());
    let parts = tokenize_unquoted(val);
    let mut buf = String::new();
    for p in parts {
        match p {
            RawPart::Lit(l) => buf.push_str(&l),
            RawPart::Var(n) | RawPart::BraceVar(n) => {
                let v = resolve_dotenv_var(&n, vars, visiting)?;
                buf.push_str(&v);
            }
        }
    }
    visiting.pop();
    Ok(buf)
}

#[js::function]
pub fn parse_dotenv<'js>(
    ctx: Ctx<'js>,
    input: StringArg,
    options: Opt<DotenvOptions>,
) -> js::Result<Object<'js>> {
    let s = input.as_str();
    let opts = options.0.unwrap_or_default();
    let map = parse_with_vars(s, opts.expand, &opts.vars).map_err(|e| e.into_exception(&ctx))?;
    let obj = Object::new(ctx.clone())?;
    for (k, v) in &map {
        obj.set(k.as_str(), JsString::from_str(ctx.clone(), v)?)?;
    }
    Ok(obj)
}
