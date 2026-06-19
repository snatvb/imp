use chrono::Timelike;
use rquickjs::class::Trace;
use rquickjs::class::Tracer;
use rquickjs::function::Constructor;
use rquickjs::{self as js, Class, Ctx, JsLifetime, Result, Value};

use js_core::error::JsError as _;

use crate::duration::Duration;
use crate::error::Error;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct ImpTime {
    #[qjs(skip_trace)]
    pub(crate) inner: chrono::NaiveTime,
}

impl<'js> Trace<'js> for ImpTime {
    fn trace<'a>(&self, _: Tracer<'a, 'js>) {}
}

fn check_time_range(ctx: &Ctx<'_>, h: u32, m: u32, s: u32, n: u32) -> Result<chrono::NaiveTime> {
    chrono::NaiveTime::from_hms_nano_opt(h, m, s, n).ok_or_else(|| {
        Error::OutOfRange(format!("invalid time: {h:02}:{m:02}:{s:02}.{n:09}")).into_exception(ctx)
    })
}

fn extract_time<'js>(ctx: &Ctx<'js>, val: &Value<'js>, name: &str) -> Result<chrono::NaiveTime> {
    let class = Class::<ImpTime>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be an ImpTime")).into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}

fn extract_duration<'js>(ctx: &Ctx<'js>, val: &Value<'js>, name: &str) -> Result<chrono::Duration> {
    let class = Class::<Duration>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be a Duration")).into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}

#[js::methods]
impl ImpTime {
    #[qjs(constructor)]
    fn js_new() -> Self {
        Self {
            inner: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        }
    }

    #[qjs(static, rename = "fromHms")]
    fn js_from_hms(ctx: Ctx<'_>, hour: u32, minute: u32, second: u32) -> Result<Self> {
        let t = check_time_range(&ctx, hour, minute, second, 0)?;
        Ok(Self { inner: t })
    }

    #[qjs(static, rename = "fromHmsNano")]
    fn js_from_hms_nano(
        ctx: Ctx<'_>,
        hour: u32,
        minute: u32,
        second: u32,
        nano: u32,
    ) -> Result<Self> {
        let t = check_time_range(&ctx, hour, minute, second, nano)?;
        Ok(Self { inner: t })
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
        // NaiveTime has no checked_add_signed; convert to NaiveDateTime and back
        let base = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let dt = base.and_time(self.inner);
        let new_dt = dt
            .checked_add_signed(dur)
            .ok_or_else(|| Error::OutOfRange("time overflow".to_string()).into_exception(&ctx))?;
        Ok(Self {
            inner: new_dt.time(),
        })
    }

    #[qjs(rename = "toIso")]
    fn to_iso(&self) -> String {
        self.inner.format("%H:%M:%S%.f").to_string()
    }

    #[qjs(rename = "toJsDate")]
    fn to_js_date<'js>(&self, ctx: Ctx<'js>) -> Result<js::Object<'js>> {
        let ctor: Constructor = ctx.globals().get("Date")?;
        let secs = self.inner.num_seconds_from_midnight() as f64;
        let ts = secs * 1000.0;
        ctor.construct((ts,))
    }

    #[qjs(rename = "toString")]
    fn js_to_string(&self) -> String {
        self.to_iso()
    }

    #[qjs(rename = "equals")]
    fn equals<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_time(&ctx, &other, "other")?;
        Ok(self.inner == other)
    }
}
