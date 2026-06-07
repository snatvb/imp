use chrono::{NaiveDate, Weekday};

use crate::js;
use crate::object::ObjectMethodExt;

pub fn parse_date_field<'js>(opts: &js::Object<'js>, field: &str) -> js::Result<Option<NaiveDate>> {
    opts.get::<_, Option<js::Value>>(field)?
        .as_ref()
        .and_then(js::Value::as_object)
        .map(|obj| {
            obj.call_method::<_, f64>("getTime", ())
                .map(timestamp_to_naive_date)
        })
        .transpose()
}

pub fn weekday_from_u8(n: u8) -> Option<Weekday> {
    match n {
        0 => Some(Weekday::Sun),
        1 => Some(Weekday::Mon),
        2 => Some(Weekday::Tue),
        3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu),
        5 => Some(Weekday::Fri),
        6 => Some(Weekday::Sat),
        _ => None,
    }
}

pub fn timestamp_to_naive_date(ms: f64) -> NaiveDate {
    let secs = (ms / 1000.0) as i64;
    chrono::DateTime::from_timestamp(secs, 0)
        .unwrap_or_default()
        .naive_utc()
        .date()
}

pub fn naive_date_to_js_date<'js>(
    ctx: &js::Ctx<'js>,
    date: &NaiveDate,
) -> js::Result<js::Object<'js>> {
    let date_ctor: js::function::Constructor = ctx.globals().get("Date")?;
    let timestamp_ms = date
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis() as f64;
    date_ctor.construct((timestamp_ms,))
}
