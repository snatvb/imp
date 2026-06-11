use crate::prelude::*;
use serde_json::{Map, Number};

use crate::error::Error;

pub fn value_to_js<'js>(ctx: &Ctx<'js>, val: serde_json::Value) -> js::Result<Value<'js>> {
    match val {
        serde_json::Value::Null => Ok(Value::new_null(ctx.clone())),
        serde_json::Value::Bool(b) => Ok(Value::new_bool(ctx.clone(), b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::new_int(ctx.clone(), i as i32))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::new_float(ctx.clone(), f))
            } else {
                Ok(Value::new_null(ctx.clone()))
            }
        }
        serde_json::Value::String(s) => {
            let js_str = JsString::from_str(ctx.clone(), &s)?;
            Ok(js_str.into_value())
        }
        serde_json::Value::Array(arr) => {
            let js_arr = Array::new(ctx.clone())?;
            for (i, item) in arr.into_iter().enumerate() {
                let js_item = value_to_js(ctx, item)?;
                js_arr.set(i, js_item)?;
            }
            Ok(js_arr.into_value())
        }
        serde_json::Value::Object(map) => {
            let js_obj = Object::new(ctx.clone())?;
            for (k, v) in map {
                let js_v = value_to_js(ctx, v)?;
                js_obj.set(k.as_str(), js_v)?;
            }
            Ok(js_obj.into_value())
        }
    }
}

pub fn js_to_value<'js>(_ctx: &Ctx<'js>, val: Value<'js>) -> Result<serde_json::Value, Error> {
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
            let s = StringArg::from_js(_ctx, val).map_err(|e| Error::Parse(e.to_string()))?;
            Ok(serde_json::Value::String(s.as_str().to_string()))
        }
        js::Type::Array => {
            let arr = val
                .as_array()
                .ok_or_else(|| Error::Parse("invalid array".into()))?;
            let mut result = Vec::with_capacity(arr.len());
            for item in arr.iter::<Value>() {
                let item = item.map_err(|e| Error::Parse(e.to_string()))?;
                result.push(js_to_value(_ctx, item)?);
            }
            Ok(serde_json::Value::Array(result))
        }
        js::Type::Object
        | js::Type::Constructor
        | js::Type::Promise
        | js::Type::Proxy
        | js::Type::Exception => {
            let obj = val
                .as_object()
                .ok_or_else(|| Error::Parse("invalid object".into()))?;
            let mut map = Map::new();
            for item in obj.props::<String, Value>() {
                let (k, v) = item.map_err(|e| Error::Parse(e.to_string()))?;
                map.insert(k, js_to_value(_ctx, v)?);
            }
            Ok(serde_json::Value::Object(map))
        }
        _ => Err(Error::Unsupported(format!(
            "cannot convert {:?} to JSON",
            val.type_of()
        ))),
    }
}
