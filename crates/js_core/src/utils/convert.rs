use super::string::JsString;
use crate::js::{Array, Ctx, JsIterator, Object, Promise, Type, Value};
use crate::object::ObjectMethodExt;

pub fn convert_to_string<'js>(
    ctx: &Ctx<'js>,
    args: &[Value<'js>],
    depth: usize,
    quote_str: bool,
) -> String {
    let mut out = String::new();
    for arg in args {
        format_one(ctx, arg, depth, quote_str, &mut out);
        out.push(' ');
    }
    out.trim_end().to_string()
}

fn format_one<'js>(
    ctx: &Ctx<'js>,
    arg: &Value<'js>,
    depth: usize,
    quote_str: bool,
    out: &mut String,
) {
    match arg.type_of() {
        Type::Undefined => out.push_str("undefined"),
        Type::Null => out.push_str("null"),
        Type::Bool => format_bool(arg, out),
        Type::Int | Type::Float => format_number(arg, out),
        Type::String => format_string(arg.as_string().unwrap(), quote_str, out),
        Type::Function => format_function(arg.as_function().unwrap(), out),
        Type::Array => format_array(ctx, arg.as_array().unwrap(), depth, out),
        Type::Object | Type::Constructor | Type::Promise | Type::Proxy | Type::Exception => {
            let obj = arg.as_object().unwrap();
            if let Some(class) = crate::js::Class::<crate::rs_string::RsString>::from_object(obj) {
                let borrowed = class.borrow();
                let s = borrowed.get_slice().to_string();
                if quote_str {
                    out.push('"');
                    out.push_str(&s);
                    out.push('"');
                } else {
                    out.push_str(&s);
                }
                return;
            }
            format_object(ctx, obj, depth, out)
        }
        _ => out.push_str("[object/unknown]"),
    }
}

fn format_bool(arg: &Value<'_>, out: &mut String) {
    if let Some(val) = arg.as_bool() {
        out.push_str(if val { "true" } else { "false" });
    }
}

fn format_number(arg: &Value<'_>, out: &mut String) {
    if let Some(val) = arg.as_int() {
        out.push_str(&val.to_string());
    } else if let Some(val) = arg.as_float() {
        out.push_str(&val.to_string());
    }
}

fn format_function(f: &Object<'_>, out: &mut String) {
    let name: String = f.get("name").unwrap_or_else(|_| "anonymous".to_string());
    out.push_str(&format!("Function {name}"));
}

fn format_string(js_str: &crate::js::String<'_>, quote_str: bool, out: &mut String) {
    if let Ok(rust_str) = js_str.to_string() {
        if quote_str {
            out.push('"');
            out.push_str(&rust_str.js_string());
            out.push('"');
        } else {
            out.push_str(&rust_str);
        }
    }
}

fn format_array<'js>(ctx: &Ctx<'js>, arr: &Array<'js>, depth: usize, out: &mut String) {
    if depth == 0 {
        out.push_str("[ ... ]");
        return;
    }
    out.push_str("[ ");
    let mut first = true;
    for val in arr.iter::<Value<'_>>().flatten() {
        if !first {
            out.push_str(", ");
        }
        format_one(ctx, &val, depth - 1, true, out);
        first = false;
    }
    out.push_str(" ]");
}

fn format_object<'js>(ctx: &Ctx<'js>, obj: &Object<'js>, depth: usize, out: &mut String) {
    if let Ok(map_ctor) = ctx.globals().get::<_, Object>("Map")
        && obj.is_instance_of(&map_ctor)
    {
        format_map(ctx, obj, depth, out);
        return;
    }
    if let Ok(set_ctor) = ctx.globals().get::<_, Object>("Set")
        && obj.is_instance_of(&set_ctor)
    {
        format_set(ctx, obj, depth, out);
        return;
    }
    if let Ok(date_ctor) = ctx.globals().get::<_, Object>("Date")
        && obj.is_instance_of(&date_ctor)
    {
        format_date(obj, out);
        return;
    }
    if let Ok(error_ctor) = ctx.globals().get::<_, Object>("Error")
        && obj.is_instance_of(&error_ctor)
    {
        format_error(obj, out);
        return;
    }
    if let Ok(promise_ctor) = ctx.globals().get::<_, Object>("Promise")
        && obj.is_instance_of(&promise_ctor)
    {
        format_promise(ctx, obj, depth, out);
        return;
    }
    format_plain(ctx, obj, depth, out);
}

fn format_map<'js>(ctx: &Ctx<'js>, obj: &Object<'js>, depth: usize, out: &mut String) {
    let size: u32 = obj.get("size").unwrap_or(0);
    out.push_str(&format!("Map({})", size));
    if depth == 0 {
        out.push_str(" { ... }");
        return;
    }
    out.push_str(" { ");
    if let Ok(entries) = obj.call_method::<_, JsIterator<Array>>("entries", ()) {
        let mut first = true;
        for entry in entries.flatten() {
            if let (Ok(key), Ok(val)) = (entry.get::<Value>(0), entry.get::<Value>(1)) {
                if !first {
                    out.push_str(", ");
                }
                format_one(ctx, &key, depth - 1, true, out);
                out.push_str(" => ");
                format_one(ctx, &val, depth - 1, true, out);
                first = false;
            }
        }
    }
    out.push_str(" }");
}

fn format_set<'js>(ctx: &Ctx<'js>, obj: &Object<'js>, depth: usize, out: &mut String) {
    let size: u32 = obj.get("size").unwrap_or(0);
    out.push_str(&format!("Set({})", size));
    if depth == 0 {
        out.push_str(" { ... }");
        return;
    }
    out.push_str(" { ");
    if let Ok(values) = obj.call_method::<_, JsIterator<Value>>("values", ()) {
        let mut first = true;
        for val in values.flatten() {
            if !first {
                out.push_str(", ");
            }
            format_one(ctx, &val, depth - 1, true, out);
            first = false;
        }
    }
    out.push_str(" }");
}

fn format_date(obj: &Object<'_>, out: &mut String) {
    if let Ok(iso) = obj.call_method::<_, String>("toISOString", ()) {
        out.push_str(&format!("Date({})", iso));
    } else {
        out.push_str("Date(Invalid)");
    }
}

fn format_error(obj: &Object<'_>, out: &mut String) {
    if let Ok(msg) = obj.call_method::<_, String>("toString", ()) {
        out.push_str(&msg);
    } else {
        out.push_str("Error");
    }
}

fn format_promise<'js>(ctx: &Ctx<'js>, obj: &Object<'js>, depth: usize, out: &mut String) {
    let promise = match Promise::from_value(obj.clone().into_value()) {
        Ok(p) => p,
        Err(_) => {
            out.push_str("Promise { }");
            return;
        }
    };

    match promise.state() {
        crate::js::promise::PromiseState::Pending => {
            out.push_str("Promise { 'pending' }");
        }
        crate::js::promise::PromiseState::Resolved => {
            if depth == 0 {
                out.push_str("Promise { ... }");
            } else if let Some(Ok(val)) = promise.result::<Value>() {
                out.push_str("Promise { ");
                format_one(ctx, &val, depth - 1, true, out);
                out.push_str(" }");
            } else {
                out.push_str("Promise { }");
            }
        }
        crate::js::promise::PromiseState::Rejected => {
            if depth == 0 {
                out.push_str("Promise { ... }");
            } else if promise.result::<Value>().is_some() {
                out.push_str("Promise { <rejected> ");
                let reason = ctx.catch();
                format_one(ctx, &reason, depth - 1, true, out);
                out.push_str(" }");
            } else {
                out.push_str("Promise { <rejected> }");
            }
        }
    }
}

fn format_plain<'js>(ctx: &Ctx<'js>, obj: &Object<'js>, depth: usize, out: &mut String) {
    if depth == 0 {
        out.push_str("{ ... }");
        return;
    }
    out.push_str("{ ");
    let mut first = true;
    for (key, val) in obj.props::<String, Value>().flatten() {
        if !first {
            out.push_str(", ");
        }
        out.push_str(&key);
        out.push_str(": ");
        format_one(ctx, &val, depth - 1, true, out);
        first = false;
    }
    out.push_str(" }");
}
