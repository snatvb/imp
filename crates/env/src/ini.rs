use std::collections::BTreeMap;

use js_core::error::JsError as _;
use js_core::js;
use js_core::js::function::Opt;
use js_core::js::{Ctx, Object, String as JsString, Value as JsValue};
use js_core::utils::StringArg;

use crate::error::EnvError;

#[derive(Debug, Clone)]
pub(crate) enum IniValue {
    Str(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Object(BTreeMap<String, IniValue>),
}

fn apply_case(s: &str, case_sensitive: bool) -> String {
    if case_sensitive {
        s.to_string()
    } else {
        s.to_lowercase()
    }
}

fn parse_unquoted(raw: &str) -> String {
    let s = raw.trim();
    if let Some(idx) = s.find(['#', ';']) {
        s[..idx].trim_end().to_string()
    } else {
        s.to_string()
    }
}

fn coerce_scalar(raw: &str) -> IniValue {
    let s = raw.trim();
    if s.is_empty() {
        return IniValue::Str(String::new());
    }
    let lower = s.to_ascii_lowercase();
    if lower == "true" {
        return IniValue::Bool(true);
    }
    if lower == "false" {
        return IniValue::Bool(false);
    }
    if let Ok(i) = s.parse::<i64>() {
        return IniValue::Int(i);
    }
    if let Ok(f) = s.parse::<f64>()
        && f.is_finite()
    {
        return IniValue::Float(f);
    }
    IniValue::Str(s.to_string())
}

fn unescape_double_quoted(s: &str) -> Result<String, EnvError> {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'"' => {
                    out.push('"');
                    i += 2;
                }
                b'\\' => {
                    out.push('\\');
                    i += 2;
                }
                b'\'' => {
                    out.push('\'');
                    i += 2;
                }
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
                b'0' => {
                    out.push('\0');
                    i += 2;
                }
                b'b' => {
                    out.push('\u{0008}');
                    i += 2;
                }
                b'f' => {
                    out.push('\u{000C}');
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
            let s_rest = &s[i..];
            let ch = s_rest.chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    Ok(out)
}

#[derive(Debug)]
enum Token {
    Empty,
    Comment,
    Section(String),
    KeyValue {
        key: String,
        value: String,
        line_cont: bool,
        tri_open: bool,
    },
}

fn split_kv(trimmed: &str) -> Option<(String, &str)> {
    let eq = trimmed.find('=')?;
    let key = trimmed[..eq].trim().to_string();
    if key.is_empty() {
        return None;
    }
    let value = &trimmed[eq + 1..];
    Some((key, value))
}

fn classify_line(trimmed: &str) -> Token {
    if trimmed.is_empty() {
        return Token::Empty;
    }
    if trimmed.starts_with(';') || trimmed.starts_with('#') {
        return Token::Comment;
    }
    if trimmed.starts_with('[') {
        if let Some(end) = trimmed.find(']') {
            let name = trimmed[1..end].trim().to_string();
            if !name.is_empty() {
                return Token::Section(name);
            }
        }
        return Token::Comment;
    }
    let Some((key, value)) = split_kv(trimmed) else {
        return Token::Comment;
    };
    let v_trim = value.trim_start();
    if let Some(after) = v_trim.strip_prefix("\"\"\"") {
        if let Some(end) = after.rfind("\"\"\"") {
            let inner = after[..end].to_string();
            return Token::KeyValue {
                key,
                value: inner,
                line_cont: false,
                tri_open: false,
            };
        }
        return Token::KeyValue {
            key,
            value: String::new(),
            line_cont: false,
            tri_open: true,
        };
    }
    if v_trim.ends_with('\\') && !v_trim.ends_with("\\\\") {
        let body = v_trim[..v_trim.len() - 1].trim_end().to_string();
        return Token::KeyValue {
            key,
            value: body,
            line_cont: true,
            tri_open: false,
        };
    }
    Token::KeyValue {
        key,
        value: v_trim.trim_end().to_string(),
        line_cont: false,
        tri_open: false,
    }
}

fn finalize_value(raw: &str) -> IniValue {
    let s = raw.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        return match unescape_double_quoted(inner) {
            Ok(v) => IniValue::Str(v),
            Err(_) => IniValue::Str(s.to_string()),
        };
    }
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
        return IniValue::Str(s[1..s.len() - 1].to_string());
    }
    let trimmed = parse_unquoted(s);
    coerce_scalar(&trimmed)
}

fn insert_into(
    target: &mut BTreeMap<String, IniValue>,
    key: &str,
    value: IniValue,
) -> Result<(), EnvError> {
    let parts: Vec<&str> = key.split('.').collect();
    if parts.len() == 1 {
        target.insert(parts[0].to_string(), value);
        return Ok(());
    }
    let head = parts[0];
    let rest = parts[1..].join(".");
    let entry = target
        .entry(head.to_string())
        .or_insert_with(|| IniValue::Object(BTreeMap::new()));
    match entry {
        IniValue::Object(map) => insert_into(map, &rest, value),
        _ => {
            *entry = IniValue::Object(BTreeMap::new());
            if let IniValue::Object(map) = entry {
                insert_into(map, &rest, value)
            } else {
                unreachable!()
            }
        }
    }
}

fn set_at(
    root: &mut BTreeMap<String, IniValue>,
    section: Option<&str>,
    key: &str,
    value: IniValue,
) -> Result<(), EnvError> {
    match section {
        None => insert_into(root, key, value),
        Some(s) => {
            let entry = root
                .entry(s.to_string())
                .or_insert_with(|| IniValue::Object(BTreeMap::new()));
            match entry {
                IniValue::Object(map) => insert_into(map, key, value),
                _ => {
                    *entry = IniValue::Object(BTreeMap::new());
                    if let IniValue::Object(map) = entry {
                        insert_into(map, key, value)
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }
}

pub(crate) fn parse_public(
    input: &str,
    case_sensitive: bool,
) -> Result<BTreeMap<String, IniValue>, EnvError> {
    parse(input, case_sensitive)
}

fn parse(input: &str, case_sensitive: bool) -> Result<BTreeMap<String, IniValue>, EnvError> {
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");
    let lines: Vec<&str> = normalized.split('\n').collect();

    let mut root: BTreeMap<String, IniValue> = BTreeMap::new();
    let mut section: Option<String> = None;
    let mut pending_key: Option<String> = None;
    let mut pending_buffer: String = String::new();
    let mut pending_triple = false;

    let mut i = 0;
    while i < lines.len() {
        let raw_line = lines[i];
        let trimmed = raw_line.trim();

        if pending_triple {
            if trimmed == "\"\"\"" {
                set_at(
                    &mut root,
                    section.as_deref(),
                    &pending_key.take().unwrap(),
                    IniValue::Str(pending_buffer.clone()),
                )?;
                pending_buffer.clear();
                pending_triple = false;
            } else {
                pending_buffer.push_str(raw_line);
                pending_buffer.push('\n');
            }
            i += 1;
            continue;
        }

        if let Some(key) = pending_key.clone() {
            let leading_ws_len = raw_line.len() - raw_line.trim_start().len();
            let body = &raw_line[leading_ws_len..];
            if body.trim_end().ends_with('\\') && !body.trim_end().ends_with("\\\\") {
                let s = body.trim_end();
                pending_buffer.push_str(s[..s.len() - 1].trim_end());
                pending_buffer.push(' ');
            } else {
                pending_buffer.push_str(body);
                set_at(
                    &mut root,
                    section.as_deref(),
                    &key,
                    finalize_value(&pending_buffer),
                )?;
                pending_buffer.clear();
                pending_key = None;
            }
            i += 1;
            continue;
        }

        let token = classify_line(trimmed);
        match token {
            Token::Empty | Token::Comment => {}
            Token::Section(name) => {
                section = Some(apply_case(&name, case_sensitive));
            }
            Token::KeyValue {
                key,
                value,
                line_cont,
                tri_open,
            } => {
                let k = apply_case(&key, case_sensitive);
                if tri_open {
                    pending_key = Some(k);
                    pending_buffer.clear();
                    pending_triple = true;
                } else if line_cont {
                    pending_key = Some(k);
                    pending_buffer.clear();
                    pending_buffer.push_str(&value);
                    pending_buffer.push(' ');
                } else {
                    set_at(&mut root, section.as_deref(), &k, finalize_value(&value))?;
                }
            }
        }
        i += 1;
    }

    if let Some(key) = pending_key.take() {
        if pending_triple {
            return Err(EnvError::Format(format!(
                "unterminated triple-quoted value for key '{key}'"
            )));
        }
        set_at(
            &mut root,
            section.as_deref(),
            &key,
            finalize_value(&pending_buffer),
        )?;
    }

    Ok(root)
}

pub(crate) fn value_to_js_public<'js>(ctx: &Ctx<'js>, val: &IniValue) -> js::Result<JsValue<'js>> {
    value_to_js(ctx, val)
}

fn value_to_js<'js>(ctx: &Ctx<'js>, val: &IniValue) -> js::Result<JsValue<'js>> {
    match val {
        IniValue::Str(s) => Ok(JsString::from_str(ctx.clone(), s)?.into_value()),
        IniValue::Bool(b) => Ok(JsValue::new_bool(ctx.clone(), *b)),
        IniValue::Int(i) => {
            if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                Ok(JsValue::new_int(ctx.clone(), *i as i32))
            } else {
                Ok(JsValue::new_float(ctx.clone(), *i as f64))
            }
        }
        IniValue::Float(f) => Ok(JsValue::new_float(ctx.clone(), *f)),
        IniValue::Object(map) => {
            let obj = Object::new(ctx.clone())?;
            for (k, v) in map {
                let js_v = value_to_js(ctx, v)?;
                obj.set(k.as_str(), js_v)?;
            }
            Ok(obj.into_value())
        }
    }
}

#[inline]
fn get_bool(obj: &Object<'_>, key: &str, default: bool) -> bool {
    obj.get::<_, Option<bool>>(key)
        .ok()
        .flatten()
        .unwrap_or(default)
}

#[js::function]
pub fn parse_ini<'js>(
    ctx: Ctx<'js>,
    input: StringArg,
    options: Opt<Object<'js>>,
) -> js::Result<Object<'js>> {
    let s = input.as_str();
    let case_sensitive = options
        .0
        .as_ref()
        .map(|o| get_bool(o, "caseSensitive", false))
        .unwrap_or(false);

    let map = parse(s, case_sensitive).map_err(|e| e.into_exception(&ctx))?;
    let obj = Object::new(ctx.clone())?;
    for (k, v) in &map {
        let js_v = value_to_js(&ctx, v)?;
        obj.set(k.as_str(), js_v)?;
    }
    Ok(obj)
}
