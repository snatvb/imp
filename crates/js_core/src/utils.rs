use rquickjs::{Ctx, Value};

pub fn convert_to_string(args: &[Value<'_>]) -> String {
    let mut output = String::new();

    for arg in args {
        if let Some(js_str) = arg.as_string() {
            if let Ok(rust_str) = js_str.to_string() {
                output.push_str(&rust_str);
                output.push(' ');
                continue;
            }
        } else if arg.is_number() {
            if let Some(val) = arg.as_int() {
                output.push_str(&val.to_string());
            } else if let Some(val) = arg.as_float() {
                output.push_str(&val.to_string());
            }
            output.push(' ');
            continue;
        } else if arg.is_bool() {
            if let Some(val) = arg.as_bool() {
                output.push_str(&val.to_string());
            }
            output.push(' ');
            continue;
        } else if arg.is_null() {
            output.push_str("null");
            output.push(' ');
            continue;
        } else if let Some(f) = arg.as_function() {
            let name: String = f.get("name").unwrap_or_else(|_| "anonymous".to_string());
            output.push_str(&format!("Function {name}"));
            output.push(' ');
        } else if arg.is_undefined() {
            output.push_str("undefined");
            output.push(' ');
            continue;
        } else if let Some(arr) = arg.as_array().map(|arr| arr.iter::<Value<'_>>()) {
            output.push_str("[ ");
            for (i, item) in arr.flatten().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                let result = convert_to_string(&[item]);
                output.push_str(result.as_str());
            }
            output.push_str(" ]");
            output.push(' ');
            continue;
        } else if arg.as_object().is_some() {
            output.push_str("[object]");
            output.push(' ');
            continue;
        } else {
            output.push_str("[object/unknown] ");
        }
    }
    output.trim_end().to_string()
}

pub fn extract_trace(ctx: &Ctx) -> String {
    ctx.eval("new Error().stack")
        .map(|s: String| s.lines().skip(1).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|_| "No stack trace available".to_string())
}
