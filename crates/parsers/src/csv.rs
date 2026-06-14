use js_core::rs_string::RsString;

use crate::prelude::*;

use crate::convert::{js_to_value, value_to_js_ex};
use crate::error::Error;

fn extract_headers(rows: &[serde_json::Value]) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    for row in rows {
        if let Some(obj) = row.as_object() {
            for key in obj.keys() {
                seen.insert(key.clone());
            }
        }
    }
    seen.into_iter().collect()
}

fn value_to_field(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn row_to_record(row: &serde_json::Value, headers: &[String]) -> Vec<String> {
    row.as_object()
        .map(|obj| {
            headers
                .iter()
                .map(|h| obj.get(h).map(value_to_field).unwrap_or_default())
                .collect()
        })
        .unwrap_or_default()
}

#[js::function]
pub fn parse<'js>(
    ctx: Ctx<'js>,
    input: StringArg,
    options: Opt<Object<'js>>,
) -> js::Result<Value<'js>> {
    let native_strings = options
        .into_inner()
        .and_then(|o| o.get::<_, Option<bool>>("nativeStrings").ok())
        .flatten()
        .unwrap_or(true);
    let s = input.as_str();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(s.as_bytes());
    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| Error::Parse(e.to_string()).into_exception(&ctx))?
        .iter()
        .map(|h| h.to_string())
        .collect();
    let mut records = Vec::new();
    for result in reader.records() {
        let row = result.map_err(|e| Error::Parse(e.to_string()).into_exception(&ctx))?;
        let mut map = serde_json::Map::new();
        for (i, field) in row.iter().enumerate() {
            if let Some(key) = headers.get(i) {
                map.insert(key.clone(), serde_json::Value::String(field.to_string()));
            }
        }
        records.push(serde_json::Value::Object(map));
    }
    value_to_js_ex(&ctx, serde_json::Value::Array(records), native_strings)
}

#[js::function]
pub fn stringify<'js>(ctx: Ctx<'js>, value: Value<'js>) -> js::Result<js::Class<'js, RsString>> {
    let val = js_to_value(&ctx, value).map_err(|e| e.into_exception(&ctx))?;
    let rows = match &val {
        serde_json::Value::Array(rows) => rows,
        _ => {
            return Err(
                Error::Serialize("csv.stringify expects an array of objects".into())
                    .into_exception(&ctx),
            );
        }
    };
    let mut writer = csv::Writer::from_writer(Vec::new());
    let headers = extract_headers(rows);
    if !headers.is_empty() {
        writer
            .write_record(&headers)
            .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?;
        for row in rows {
            let record = row_to_record(row, &headers);
            writer
                .write_record(&record)
                .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?;
        }
    }
    let s = String::from_utf8(
        writer
            .into_inner()
            .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?,
    )
    .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?;
    js::Class::instance(ctx, RsString::owned(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_headers_sorted() {
        let rows = vec![json!({"z": 1, "a": 2, "m": 3}), json!({"b": 4, "a": 5})];
        let headers = extract_headers(&rows);
        assert_eq!(headers, vec!["a", "b", "m", "z"]);
    }

    #[test]
    fn test_extract_headers_empty() {
        let rows: Vec<serde_json::Value> = vec![];
        let headers = extract_headers(&rows);
        assert!(headers.is_empty());
    }

    #[test]
    fn test_value_to_field_string() {
        assert_eq!(value_to_field(&json!("hello")), "hello");
    }

    #[test]
    fn test_value_to_field_null() {
        assert_eq!(value_to_field(&serde_json::Value::Null), "");
    }

    #[test]
    fn test_value_to_field_number() {
        assert_eq!(value_to_field(&json!(42)), "42");
    }

    #[test]
    fn test_value_to_field_bool() {
        assert_eq!(value_to_field(&json!(true)), "true");
    }

    #[test]
    fn test_row_to_record() {
        let row = json!({"a": "1", "b": "2"});
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let record = row_to_record(&row, &headers);
        assert_eq!(record, vec!["1", "2", ""]);
    }

    #[test]
    fn test_row_to_record_non_object() {
        let row = json!("not an object");
        let headers = vec!["a".to_string()];
        let record = row_to_record(&row, &headers);
        assert!(record.is_empty());
    }

    #[test]
    fn test_extract_headers_partial_rows() {
        let rows = vec![
            json!({"name": "Alice"}),
            json!({"name": "Bob", "email": "bob@test.com"}),
        ];
        let headers = extract_headers(&rows);
        assert_eq!(headers, vec!["email", "name"]);
    }
}
