use crate::prelude::*;
use js_core::js::{Class, JsIterator};
use js_core::object::ObjectMethodExt;
use js_core::rs_string::RsString;
use toml::value::{Datetime, Table};

use crate::error::Error;

pub fn js_to_toml_value<'js>(ctx: &Ctx<'js>, val: Value<'js>) -> Result<toml::Value, Error> {
    js_to_toml_value_depth(ctx, val, 0)
}

fn js_to_toml_value_depth<'js>(
    ctx: &Ctx<'js>,
    val: Value<'js>,
    depth: usize,
) -> Result<toml::Value, Error> {
    const MAX_DEPTH: usize = 256;
    if depth > MAX_DEPTH {
        return Err(Error::Parse("maximum nesting depth exceeded".into()));
    }

    match val.type_of() {
        js::Type::Null | js::Type::Undefined => Ok(toml::Value::String("null".to_string())),
        js::Type::Bool => val
            .as_bool()
            .map(toml::Value::Boolean)
            .ok_or_else(|| Error::Parse("invalid bool".into())),
        js::Type::Int => val
            .as_int()
            .map(|i| toml::Value::Integer(i as i64))
            .ok_or_else(|| Error::Parse("invalid int".into())),
        js::Type::Float => val
            .as_float()
            .map(toml::Value::Float)
            .ok_or_else(|| Error::Parse("invalid float".into())),
        js::Type::String => {
            let s = StringArg::from_js(ctx, val).map_err(|e| Error::Parse(e.to_string()))?;
            Ok(toml::Value::String(s.as_str().to_string()))
        }
        js::Type::Array => {
            let arr = val
                .as_array()
                .ok_or_else(|| Error::Parse("invalid array".into()))?;
            let mut result = Vec::with_capacity(arr.len());
            for item in arr.iter::<Value>() {
                let item = item.map_err(|e| Error::Parse(e.to_string()))?;
                result.push(js_to_toml_value_depth(ctx, item, depth + 1)?);
            }
            Ok(toml::Value::Array(result))
        }
        js::Type::Function => Ok(toml::Value::String("null".to_string())),
        js::Type::Object
        | js::Type::Constructor
        | js::Type::Promise
        | js::Type::Proxy
        | js::Type::Exception => {
            let obj = val
                .as_object()
                .ok_or_else(|| Error::Parse("invalid object".into()))?;
            convert_toml_object(ctx, obj, depth)
        }
        _ => Err(Error::Unsupported(format!(
            "cannot convert {:?} to TOML",
            val.type_of()
        ))),
    }
}

fn convert_toml_object<'js>(
    ctx: &Ctx<'js>,
    obj: &Object<'js>,
    depth: usize,
) -> Result<toml::Value, Error> {
    if let Some(class) = Class::<RsString>::from_object(obj) {
        let borrowed = class.borrow();
        return Ok(toml::Value::String(borrowed.get_slice().to_string()));
    }

    if let Ok(date_ctor) = ctx.globals().get::<_, Object>("Date")
        && obj.is_instance_of(&date_ctor)
    {
        if let Ok(iso_str) = obj.call_method::<_, String>("toISOString", ()) {
            if let Ok(dt) = iso_str.parse::<Datetime>() {
                return Ok(toml::Value::Datetime(dt));
            }
            return Ok(toml::Value::String(iso_str));
        }
        return Ok(toml::Value::String("null".to_string()));
    }

    if let Ok(regexp_ctor) = ctx.globals().get::<_, Object>("RegExp")
        && obj.is_instance_of(&regexp_ctor)
    {
        if let Ok(str_repr) = obj.call_method::<_, String>("toString", ()) {
            return Ok(toml::Value::String(str_repr));
        }
        return Ok(toml::Value::String("null".to_string()));
    }

    if let Ok(set_ctor) = ctx.globals().get::<_, Object>("Set")
        && obj.is_instance_of(&set_ctor)
    {
        let mut result = Vec::new();
        if let Ok(values) = obj.call_method::<_, JsIterator<Value>>("values", ()) {
            for val in values.flatten() {
                result.push(js_to_toml_value_depth(ctx, val, depth + 1)?);
            }
        }
        return Ok(toml::Value::Array(result));
    }

    if let Ok(map_ctor) = ctx.globals().get::<_, Object>("Map")
        && obj.is_instance_of(&map_ctor)
    {
        let mut table = Table::new();
        if let Ok(entries) = obj.call_method::<_, JsIterator<Array>>("entries", ()) {
            for entry in entries.flatten() {
                if let (Ok(key), Ok(val)) = (entry.get::<Value>(0), entry.get::<Value>(1)) {
                    let key_str = key_to_string(ctx, key)?;
                    table.insert(key_str, js_to_toml_value_depth(ctx, val, depth + 1)?);
                }
            }
        }
        return Ok(toml::Value::Table(table));
    }

    let mut table = Table::new();
    for item in obj.props::<String, Value>() {
        let (k, v) = item.map_err(|e| Error::Parse(e.to_string()))?;
        if v.type_of() == js::Type::Function {
            continue;
        }
        table.insert(k, js_to_toml_value_depth(ctx, v, depth + 1)?);
    }
    Ok(toml::Value::Table(table))
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

pub fn toml_value_to_js<'js>(ctx: &Ctx<'js>, val: toml::Value) -> js::Result<Value<'js>> {
    match val {
        toml::Value::String(s) => {
            let js_str = JsString::from_str(ctx.clone(), &s)?;
            Ok(js_str.into_value())
        }
        toml::Value::Integer(i) => Ok(Value::new_int(ctx.clone(), i as i32)),
        toml::Value::Float(f) => Ok(Value::new_float(ctx.clone(), f)),
        toml::Value::Boolean(b) => Ok(Value::new_bool(ctx.clone(), b)),
        toml::Value::Datetime(dt) => {
            let s = dt.to_string();
            let js_str = JsString::from_str(ctx.clone(), &s)?;
            Ok(js_str.into_value())
        }
        toml::Value::Array(arr) => {
            let js_arr = Array::new(ctx.clone())?;
            for (i, item) in arr.into_iter().enumerate() {
                let js_item = toml_value_to_js(ctx, item)?;
                js_arr.set(i, js_item)?;
            }
            Ok(js_arr.into_value())
        }
        toml::Value::Table(table) => {
            let js_obj = Object::new(ctx.clone())?;
            for (k, v) in table {
                let js_v = toml_value_to_js(ctx, v)?;
                js_obj.set(k.as_str(), js_v)?;
            }
            Ok(js_obj.into_value())
        }
    }
}
