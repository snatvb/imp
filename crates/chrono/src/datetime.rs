use chrono::Datelike;
use chrono::Timelike;
use rquickjs::class::Trace;
use rquickjs::class::Tracer;
use rquickjs::function::Constructor;
use rquickjs::{self as js, Class, Ctx, JsLifetime, Result, Value};

use js_core::error::JsError as _;
use js_core::utils::StringArg;

use crate::date::ImpDate;
use crate::duration::Duration;
use crate::error::Error;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct ImpDateTime {
    #[qjs(skip_trace)]
    pub(crate) inner: chrono::DateTime<chrono::Utc>,
}

impl<'js> Trace<'js> for ImpDateTime {
    fn trace<'a>(&self, _: Tracer<'a, 'js>) {}
}

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct ImpLocalDateTime {
    #[qjs(skip_trace)]
    pub(crate) inner: chrono::DateTime<chrono::Local>,
}

impl<'js> Trace<'js> for ImpLocalDateTime {
    fn trace<'a>(&self, _: Tracer<'a, 'js>) {}
}

fn extract_duration<'js>(ctx: &Ctx<'js>, val: &Value<'js>, name: &str) -> Result<chrono::Duration> {
    let class = Class::<Duration>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be a Duration")).into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}

fn extract_utc<'js>(
    ctx: &Ctx<'js>,
    val: &Value<'js>,
    name: &str,
) -> Result<chrono::DateTime<chrono::Utc>> {
    let class = Class::<ImpDateTime>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be an ImpDateTime"))
            .into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}

fn extract_local<'js>(
    ctx: &Ctx<'js>,
    val: &Value<'js>,
    name: &str,
) -> Result<chrono::DateTime<chrono::Local>> {
    let class = Class::<ImpLocalDateTime>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!(
            "The \"{name}\" argument must be an ImpLocalDateTime"
        ))
        .into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}

fn timestamp_to_utc(ctx: &Ctx<'_>, ms: f64) -> Result<chrono::DateTime<chrono::Utc>> {
    if !ms.is_finite() {
        return Err(Error::OutOfRange("timestamp must be finite".to_string()).into_exception(ctx));
    }
    let secs = (ms / 1000.0) as i64;
    let nsecs = ((ms.rem_euclid(1000.0)) * 1e6) as u32;
    chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nsecs).ok_or_else(|| {
        Error::OutOfRange(format!("timestamp out of range: {ms}")).into_exception(ctx)
    })
}

#[js::methods]
impl ImpDateTime {
    #[qjs(constructor)]
    fn js_new() -> Self {
        Self {
            inner: chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).unwrap(),
        }
    }

    #[qjs(static, rename = "now")]
    fn js_now() -> Self {
        Self {
            inner: chrono::Utc::now(),
        }
    }

    #[qjs(static, rename = "fromTimestamp")]
    fn js_from_timestamp(ctx: Ctx<'_>, ms: f64) -> Result<Self> {
        Ok(Self {
            inner: timestamp_to_utc(&ctx, ms)?,
        })
    }

    #[qjs(static, rename = "fromIso")]
    fn js_from_iso<'js>(ctx: Ctx<'js>, input: StringArg) -> Result<Self> {
        let dt = chrono::DateTime::parse_from_rfc3339(input.as_str())
            .map_err(|e| Error::Parse(format!("invalid ISO 8601: {e}")).into_exception(&ctx))?;
        Ok(Self {
            inner: dt.with_timezone(&chrono::Utc),
        })
    }

    #[qjs(rename = "getYear")]
    fn get_year(&self) -> i32 {
        self.inner.year()
    }
    #[qjs(rename = "getMonth")]
    fn get_month(&self) -> u32 {
        self.inner.month()
    }
    #[qjs(rename = "getDay")]
    fn get_day(&self) -> u32 {
        self.inner.day()
    }
    #[qjs(rename = "getHour")]
    fn get_hour(&self) -> u32 {
        self.inner.hour()
    }
    #[qjs(rename = "getMinute")]
    fn get_minute(&self) -> u32 {
        self.inner.minute()
    }
    #[qjs(rename = "getSecond")]
    fn get_second(&self) -> u32 {
        self.inner.second()
    }
    #[qjs(rename = "getNano")]
    fn get_nano(&self) -> u32 {
        self.inner.nanosecond()
    }

    #[qjs(rename = "getDate")]
    fn get_date<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, ImpDate>> {
        Class::instance(ctx.clone(), ImpDate::from_chrono(self.inner.date_naive()))
    }

    #[qjs(rename = "add")]
    fn add<'js>(&self, ctx: Ctx<'js>, d: Value<'js>) -> Result<Self> {
        let dur = extract_duration(&ctx, &d, "d")?;
        let new_dt = self.inner.checked_add_signed(dur).ok_or_else(|| {
            Error::OutOfRange("datetime overflow".to_string()).into_exception(&ctx)
        })?;
        Ok(Self { inner: new_dt })
    }

    #[qjs(rename = "sub")]
    fn sub<'js>(&self, ctx: Ctx<'js>, d: Value<'js>) -> Result<Self> {
        let dur = extract_duration(&ctx, &d, "d")?;
        let new_dt = self.inner.checked_sub_signed(dur).ok_or_else(|| {
            Error::OutOfRange("datetime overflow".to_string()).into_exception(&ctx)
        })?;
        Ok(Self { inner: new_dt })
    }

    #[qjs(rename = "diff")]
    fn diff<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<Class<'js, Duration>> {
        let other = extract_utc(&ctx, &other, "other")?;
        let dur = other - self.inner;
        Class::instance(ctx.clone(), Duration::from_chrono(dur))
    }

    #[qjs(rename = "format")]
    fn format(&self, fmt: StringArg) -> String {
        self.inner.format(fmt.as_str()).to_string()
    }

    #[qjs(rename = "toIso")]
    fn to_iso(&self) -> String {
        self.inner.to_rfc3339()
    }

    #[qjs(rename = "toJsDate")]
    fn to_js_date<'js>(&self, ctx: Ctx<'js>) -> Result<js::Object<'js>> {
        let ctor: Constructor = ctx.globals().get("Date")?;
        ctor.construct((self.inner.timestamp_millis() as f64,))
    }

    #[qjs(rename = "toString")]
    fn js_to_string(&self) -> String {
        self.to_iso()
    }

    #[qjs(rename = "equals")]
    fn equals<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_utc(&ctx, &other, "other")?;
        Ok(self.inner == other)
    }
}

#[js::methods]
impl ImpLocalDateTime {
    #[qjs(constructor)]
    fn js_new() -> Self {
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0)
            .unwrap()
            .with_timezone(&chrono::Local);
        Self { inner: dt }
    }

    #[qjs(static, rename = "nowLocal")]
    fn js_now_local() -> Self {
        Self {
            inner: chrono::Local::now(),
        }
    }

    #[qjs(static, rename = "fromTimestamp")]
    fn js_from_timestamp(ctx: Ctx<'_>, ms: f64) -> Result<Self> {
        let utc = timestamp_to_utc(&ctx, ms)?;
        Ok(Self {
            inner: utc.with_timezone(&chrono::Local),
        })
    }

    #[qjs(static, rename = "fromIso")]
    fn js_from_iso<'js>(ctx: Ctx<'js>, input: StringArg) -> Result<Self> {
        let dt = chrono::DateTime::parse_from_rfc3339(input.as_str())
            .map_err(|e| Error::Parse(format!("invalid ISO 8601: {e}")).into_exception(&ctx))?;
        Ok(Self {
            inner: dt.with_timezone(&chrono::Local),
        })
    }

    #[qjs(rename = "getYear")]
    fn get_year(&self) -> i32 {
        self.inner.year()
    }
    #[qjs(rename = "getMonth")]
    fn get_month(&self) -> u32 {
        self.inner.month()
    }
    #[qjs(rename = "getDay")]
    fn get_day(&self) -> u32 {
        self.inner.day()
    }
    #[qjs(rename = "getHour")]
    fn get_hour(&self) -> u32 {
        self.inner.hour()
    }
    #[qjs(rename = "getMinute")]
    fn get_minute(&self) -> u32 {
        self.inner.minute()
    }
    #[qjs(rename = "getSecond")]
    fn get_second(&self) -> u32 {
        self.inner.second()
    }
    #[qjs(rename = "getNano")]
    fn get_nano(&self) -> u32 {
        self.inner.nanosecond()
    }

    #[qjs(rename = "add")]
    fn add<'js>(&self, ctx: Ctx<'js>, d: Value<'js>) -> Result<Self> {
        let dur = extract_duration(&ctx, &d, "d")?;
        let new_dt = self.inner.checked_add_signed(dur).ok_or_else(|| {
            Error::OutOfRange("datetime overflow".to_string()).into_exception(&ctx)
        })?;
        Ok(Self { inner: new_dt })
    }

    #[qjs(rename = "sub")]
    fn sub<'js>(&self, ctx: Ctx<'js>, d: Value<'js>) -> Result<Self> {
        let dur = extract_duration(&ctx, &d, "d")?;
        let new_dt = self.inner.checked_sub_signed(dur).ok_or_else(|| {
            Error::OutOfRange("datetime overflow".to_string()).into_exception(&ctx)
        })?;
        Ok(Self { inner: new_dt })
    }

    #[qjs(rename = "diff")]
    fn diff<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<Class<'js, Duration>> {
        let other = extract_local(&ctx, &other, "other")?;
        let dur = other - self.inner;
        Class::instance(ctx.clone(), Duration::from_chrono(dur))
    }

    #[qjs(rename = "format")]
    fn format(&self, fmt: StringArg) -> String {
        self.inner.format(fmt.as_str()).to_string()
    }

    #[qjs(rename = "toIso")]
    fn to_iso(&self) -> String {
        self.inner.to_rfc3339()
    }

    #[qjs(rename = "toUtc")]
    fn to_utc<'js>(&self, ctx: Ctx<'js>) -> Result<Class<'js, ImpDateTime>> {
        Class::instance(
            ctx.clone(),
            ImpDateTime {
                inner: self.inner.with_timezone(&chrono::Utc),
            },
        )
    }

    #[qjs(rename = "toJsDate")]
    fn to_js_date<'js>(&self, ctx: Ctx<'js>) -> Result<js::Object<'js>> {
        let ctor: Constructor = ctx.globals().get("Date")?;
        let ts = self.inner.with_timezone(&chrono::Utc).timestamp_millis() as f64;
        ctor.construct((ts,))
    }

    #[qjs(rename = "toString")]
    fn js_to_string(&self) -> String {
        self.to_iso()
    }

    #[qjs(rename = "equals")]
    fn equals<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_local(&ctx, &other, "other")?;
        Ok(self.inner == other)
    }
}
