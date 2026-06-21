use std::collections::HashMap;

use js_core::error::JsError as _;
use js_core::js;
use js_core::js::function::Opt;
use js_core::js::{Ctx, Object, String as JsString, Value};
use js_core::utils::StringArg;

use crate::error::EnvError;

fn collect_vars<'js>(obj: &Object<'js>) -> js::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for item in obj.props::<String, Value>() {
        let (k, v) = item?;
        if let Some(s) = v.as_string() {
            let s = s.to_string()?;
            map.insert(k, s);
        } else if let Some(i) = v.as_int() {
            map.insert(k, i.to_string());
        } else if let Some(f) = v.as_float() {
            map.insert(k, f.to_string());
        } else if let Some(b) = v.as_bool() {
            map.insert(k, b.to_string());
        }
    }
    Ok(map)
}

pub(crate) fn expand_inner(
    input: &str,
    vars: &HashMap<String, String>,
    visiting: &mut Vec<String>,
) -> Result<String, EnvError> {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' && i + 1 < bytes.len() {
            let next = bytes[i + 1];
            if next == b'$' || next == b'"' {
                out.push(next as char);
                i += 2;
                continue;
            }
            out.push('\\');
            i += 1;
            continue;
        }
        if b == b'$' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                let after = &input[i + 2..];
                if let Some(end) = after.find('}') {
                    let name = &after[..end];
                    out.push_str(&resolve_var(name, vars, visiting)?);
                    i += 2 + end + 1;
                    continue;
                }
            }
            let after = &input[i + 1..];
            let name: String = after
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if name.is_empty() {
                out.push('$');
                i += 1;
                continue;
            }
            out.push_str(&resolve_var(&name, vars, visiting)?);
            i += 1 + name.len();
            continue;
        }
        if b < 0x80 {
            out.push(b as char);
            i += 1;
        } else {
            let rest = &input[i..];
            let ch = rest.chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    Ok(out)
}

fn resolve_var(
    name: &str,
    vars: &HashMap<String, String>,
    visiting: &mut Vec<String>,
) -> Result<String, EnvError> {
    if visiting.iter().any(|v| v == name) {
        return Err(EnvError::Expand(format!(
            "circular variable reference: {} -> {}",
            visiting.join(" -> "),
            name
        )));
    }
    let Some(val) = vars.get(name) else {
        return Ok(format!("${name}"));
    };
    visiting.push(name.to_string());
    let result = expand_inner(val, vars, visiting);
    visiting.pop();
    result
}

#[js::function]
pub fn expand<'js>(
    ctx: Ctx<'js>,
    input: StringArg,
    vars: Opt<Object<'js>>,
) -> js::Result<JsString<'js>> {
    let s = input.as_str();
    let map = match vars.into_inner() {
        Some(o) => collect_vars(&o)?,
        None => HashMap::new(),
    };
    let mut visiting = Vec::new();
    let result = expand_inner(s, &map, &mut visiting).map_err(|e| e.into_exception(&ctx))?;
    JsString::from_str(ctx, &result)
}
