use crate::prelude::*;
use js_core::js::{Class, JsIterator};
use js_core::object::ObjectMethodExt;
use js_core::rs_string::RsString;
use serde_json::{Map, Number};
use std::collections::HashSet;

use crate::error::Error;

pub fn value_to_js_ex<'js>(
    ctx: &Ctx<'js>,
    val: serde_json::Value,
    native_strings: bool,
) -> js::Result<Value<'js>> {
    match val {
        serde_json::Value::Null => Ok(Value::new_null(ctx.clone())),
        serde_json::Value::Bool(b) => Ok(Value::new_bool(ctx.clone(), b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    Ok(Value::new_int(ctx.clone(), i as i32))
                } else {
                    Ok(Value::new_float(ctx.clone(), i as f64))
                }
            } else if let Some(f) = n.as_f64() {
                Ok(Value::new_float(ctx.clone(), f))
            } else {
                Ok(Value::new_null(ctx.clone()))
            }
        }
        serde_json::Value::String(s) => {
            if native_strings {
                let js_str = JsString::from_str(ctx.clone(), &s)?;
                Ok(js_str.into_value())
            } else {
                let rs = Class::instance(ctx.clone(), RsString::owned(s))?;
                Ok(rs.into_value())
            }
        }
        serde_json::Value::Array(arr) => {
            let js_arr = Array::new(ctx.clone())?;
            for (i, item) in arr.into_iter().enumerate() {
                let js_item = value_to_js_ex(ctx, item, native_strings)?;
                js_arr.set(i, js_item)?;
            }
            Ok(js_arr.into_value())
        }
        serde_json::Value::Object(map) => {
            let js_obj = Object::new(ctx.clone())?;
            for (k, v) in map {
                let js_v = value_to_js_ex(ctx, v, native_strings)?;
                js_obj.set(k.as_str(), js_v)?;
            }
            Ok(js_obj.into_value())
        }
    }
}

pub fn value_to_js<'js>(ctx: &Ctx<'js>, val: serde_json::Value) -> js::Result<Value<'js>> {
    value_to_js_ex(ctx, val, true)
}

pub fn js_to_value<'js>(ctx: &Ctx<'js>, val: Value<'js>) -> Result<serde_json::Value, Error> {
    let mut visited = HashSet::new();
    js_to_value_inner(ctx, val, &mut visited)
}

fn js_to_value_inner<'js>(
    ctx: &Ctx<'js>,
    val: Value<'js>,
    visited: &mut HashSet<js::Value<'js>>,
) -> Result<serde_json::Value, Error> {
    match val.type_of() {
        js::Type::Null | js::Type::Undefined => Ok(serde_json::Value::Null),
        js::Type::Bool => val
            .as_bool()
            .map(serde_json::Value::Bool)
            .ok_or_else(|| Error::Parse("invalid bool".into())),
        js::Type::Int => val
            .as_int()
            .map(|i| serde_json::Value::Number(Number::from(i as i64)))
            .ok_or_else(|| Error::Parse("invalid int".into())),
        js::Type::Float => val
            .as_float()
            .and_then(Number::from_f64)
            .map(serde_json::Value::Number)
            .ok_or_else(|| Error::Parse("invalid float".into())),
        js::Type::String => {
            let s = StringArg::from_js(ctx, val).map_err(|e| Error::Parse(e.to_string()))?;
            Ok(serde_json::Value::String(s.as_str().to_string()))
        }
        js::Type::Array => {
            let arr = val
                .as_array()
                .ok_or_else(|| Error::Parse("invalid array".into()))?;
            let mut result = Vec::with_capacity(arr.len());
            for item in arr.iter::<Value>() {
                let item = item.map_err(|e| Error::Parse(e.to_string()))?;
                result.push(js_to_value_inner(ctx, item, visited)?);
            }
            Ok(serde_json::Value::Array(result))
        }
        js::Type::Function => Ok(serde_json::Value::Null),
        js::Type::Object
        | js::Type::Constructor
        | js::Type::Promise
        | js::Type::Proxy
        | js::Type::Exception => {
            let obj = val
                .as_object()
                .ok_or_else(|| Error::Parse("invalid object".into()))?;
            convert_object_inner(ctx, obj, visited)
        }
        _ => Err(Error::Unsupported(format!(
            "cannot convert {:?} to JSON",
            val.type_of()
        ))),
    }
}

fn convert_object_inner<'js>(
    ctx: &Ctx<'js>,
    obj: &Object<'js>,
    visited: &mut HashSet<js::Value<'js>>,
) -> Result<serde_json::Value, Error> {
    if let Some(class) = Class::<RsString>::from_object(obj) {
        let borrowed = class.borrow();
        return Ok(serde_json::Value::String(borrowed.get_slice().to_string()));
    }

    if let Ok(date_ctor) = ctx.globals().get::<_, Object>("Date")
        && obj.is_instance_of(&date_ctor)
    {
        if let Ok(json_str) = obj.call_method::<_, String>("toJSON", ()) {
            return Ok(serde_json::Value::String(json_str));
        }
        return Ok(serde_json::Value::Null);
    }

    if let Ok(regexp_ctor) = ctx.globals().get::<_, Object>("RegExp")
        && obj.is_instance_of(&regexp_ctor)
    {
        if let Ok(str_repr) = obj.call_method::<_, String>("toString", ()) {
            return Ok(serde_json::Value::String(str_repr));
        }
        return Ok(serde_json::Value::Null);
    }

    let raw = obj.as_value().clone();
    if !visited.insert(raw.clone()) {
        return Err(Error::Parse("circular reference detected".into()));
    }

    if let Ok(set_ctor) = ctx.globals().get::<_, Object>("Set")
        && obj.is_instance_of(&set_ctor)
    {
        let mut result = Vec::new();
        if let Ok(values) = obj.call_method::<_, JsIterator<Value>>("values", ()) {
            for val in values {
                let v = val.map_err(|e| Error::Parse(e.to_string()))?;
                result.push(js_to_value_inner(ctx, v, visited)?);
            }
        }
        visited.remove(&raw);
        return Ok(serde_json::Value::Array(result));
    }

    if let Ok(map_ctor) = ctx.globals().get::<_, Object>("Map")
        && obj.is_instance_of(&map_ctor)
    {
        let mut map = Map::new();
        if let Ok(entries) = obj.call_method::<_, JsIterator<Array>>("entries", ()) {
            for entry in entries {
                let entry = entry.map_err(|e| Error::Parse(e.to_string()))?;
                if let (Ok(key), Ok(val)) = (entry.get::<Value>(0), entry.get::<Value>(1)) {
                    let key_str = key_to_string(ctx, key)?;
                    map.insert(key_str, js_to_value_inner(ctx, val, visited)?);
                }
            }
        }
        visited.remove(&raw);
        return Ok(serde_json::Value::Object(map));
    }

    let mut map = Map::new();
    for item in obj.props::<String, Value>() {
        let (k, v) = item.map_err(|e| Error::Parse(e.to_string()))?;
        if v.type_of() == js::Type::Function {
            continue;
        }
        map.insert(k, js_to_value_inner(ctx, v, visited)?);
    }
    visited.remove(&raw);
    Ok(serde_json::Value::Object(map))
}

fn key_to_string<'js>(ctx: &Ctx<'js>, val: Value<'js>) -> Result<String, Error> {
    match val.type_of() {
        js::Type::String => {
            let s = StringArg::from_js(ctx, val).map_err(|e| Error::Parse(e.to_string()))?;
            Ok(s.as_str().to_string())
        }
        js::Type::Int => Ok(val.as_int().unwrap().to_string()),
        js::Type::Float => Ok(val.as_float().unwrap().to_string()),
        _ => Err(Error::Unsupported(
            "Map key must be string or number".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Number;

    #[test]
    fn test_nan_not_serializable() {
        let nan = Number::from_f64(f64::NAN);
        assert!(
            nan.is_none(),
            "NaN should not be representable as JSON Number"
        );
    }

    #[test]
    fn test_infinity_not_serializable() {
        assert!(Number::from_f64(f64::INFINITY).is_none());
        assert!(Number::from_f64(f64::NEG_INFINITY).is_none());
    }
}
