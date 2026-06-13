use js_core::rs_string::RsString;

use crate::prelude::*;

use crate::convert::{js_to_value, value_to_js};
use crate::error::Error;

fn extract_headers(rows: &[serde_json::Value]) -> Vec<String> {
    rows.first()
        .and_then(|v| v.as_object())
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default()
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
pub fn parse<'js>(ctx: Ctx<'js>, input: StringArg) -> js::Result<Value<'js>> {
    let s = input.as_str();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(s.as_bytes());
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: serde_json::Value =
            result.map_err(|e| Error::Parse(e.to_string()).into_exception(&ctx))?;
        records.push(record);
    }
    value_to_js(&ctx, serde_json::Value::Array(records))
}

#[js::function]
pub fn stringify<'js>(ctx: Ctx<'js>, value: Value<'js>) -> js::Result<js::Class<'js, RsString>> {
    let val = js_to_value(&ctx, value).map_err(|e| e.into_exception(&ctx))?;
    let mut writer = csv::Writer::from_writer(Vec::new());
    if let serde_json::Value::Array(rows) = &val {
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
    }
    let s = String::from_utf8(
        writer
            .into_inner()
            .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?,
    )
    .map_err(|e| Error::Serialize(e.to_string()).into_exception(&ctx))?;
    js::Class::instance(ctx, RsString::owned(s))
}
