use chrono::Datelike;
use rquickjs::class::Trace;
use rquickjs::class::Tracer;
use rquickjs::function::Constructor;
use rquickjs::{self as js, Class, Ctx, JsLifetime, Result, Value};

use js_core::error::JsError as _;
use js_core::utils::StringArg;

use crate::duration::Duration;
use crate::error::Error;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct ImpDate {
    #[qjs(skip_trace)]
    pub(crate) inner: chrono::NaiveDate,
}

impl<'js> Trace<'js> for ImpDate {
    fn trace<'a>(&self, _: Tracer<'a, 'js>) {}
}

impl ImpDate {
    pub fn from_chrono(d: chrono::NaiveDate) -> Self {
        Self { inner: d }
    }
}

fn extract_date<'js>(ctx: &Ctx<'js>, val: &Value<'js>, name: &str) -> Result<chrono::NaiveDate> {
    let class = Class::<ImpDate>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be an ImpDate")).into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}

fn check_date_range(ctx: &Ctx<'_>, y: i32, m: u32, d: u32) -> Result<chrono::NaiveDate> {
    let date = chrono::NaiveDate::from_ymd_opt(y, m, d).ok_or_else(|| {
        Error::OutOfRange(format!("invalid date: {y:04}-{m:02}-{d:02}")).into_exception(ctx)
    })?;
    Ok(date)
}

#[js::methods]
impl ImpDate {
    #[qjs(constructor)]
    fn js_new() -> Self {
        Self {
            inner: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        }
    }

    #[qjs(static, rename = "today")]
    fn js_today(_ctx: Ctx<'_>) -> Result<Self> {
        let local_now = chrono::Local::now();
        Ok(Self {
            inner: local_now.date_naive(),
        })
    }

    #[qjs(static, rename = "fromYmd")]
    fn js_from_ymd(ctx: Ctx<'_>, year: i32, month: u32, day: u32) -> Result<Self> {
        let date = check_date_range(&ctx, year, month, day)?;
        Ok(Self { inner: date })
    }

    #[qjs(static, rename = "fromIso")]
    fn js_from_iso<'js>(ctx: Ctx<'js>, input: StringArg) -> Result<Self> {
        let date = chrono::NaiveDate::parse_from_str(input.as_str(), "%Y-%m-%d")
            .map_err(|e| Error::Parse(format!("invalid ISO date: {e}")).into_exception(&ctx))?;
        Ok(Self { inner: date })
    }

    #[qjs(static, rename = "fromTimestamp")]
    fn js_from_timestamp(ctx: Ctx<'_>, ms: f64) -> Result<Self> {
        if !ms.is_finite() {
            return Err(
                Error::OutOfRange("timestamp must be finite".to_string()).into_exception(&ctx)
            );
        }
        let secs = (ms / 1000.0) as i64;
        let nsecs = ((ms % 1000.0) * 1e6) as u32;
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nsecs).ok_or_else(|| {
            Error::OutOfRange(format!("timestamp out of range: {ms}")).into_exception(&ctx)
        })?;
        Ok(Self {
            inner: dt.date_naive(),
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

    #[qjs(rename = "getDayOfWeek")]
    fn get_day_of_week(&self) -> u32 {
        self.inner.weekday().num_days_from_monday() + 1
    }

    #[qjs(rename = "getDayOfYear")]
    fn get_day_of_year(&self) -> u32 {
        self.inner.ordinal()
    }

    #[qjs(rename = "addDays")]
    fn add_days<'js>(&self, ctx: Ctx<'js>, d: Value<'js>) -> Result<Self> {
        let dur = extract_duration(&ctx, &d, "d")?;
        let new_date = self
            .inner
            .checked_add_signed(dur)
            .ok_or_else(|| Error::OutOfRange("date overflow".to_string()).into_exception(&ctx))?;
        Ok(Self { inner: new_date })
    }

    #[qjs(rename = "addWeeks")]
    fn add_weeks<'js>(&self, ctx: Ctx<'js>, d: Value<'js>) -> Result<Self> {
        let dur = extract_duration(&ctx, &d, "d")?;
        let new_date = self
            .inner
            .checked_add_signed(dur)
            .ok_or_else(|| Error::OutOfRange("date overflow".to_string()).into_exception(&ctx))?;
        Ok(Self { inner: new_date })
    }

    #[qjs(rename = "addMonths")]
    fn add_months<'js>(&self, ctx: Ctx<'js>, n: i32) -> Result<Self> {
        let new_date = if n >= 0 {
            self.inner.checked_add_months(chrono::Months::new(n as u32))
        } else {
            self.inner
                .checked_sub_months(chrono::Months::new(n.unsigned_abs()))
        }
        .ok_or_else(|| Error::OutOfRange("date overflow".to_string()).into_exception(&ctx))?;
        Ok(Self { inner: new_date })
    }

    #[qjs(rename = "addYears")]
    fn add_years<'js>(&self, ctx: Ctx<'js>, n: i32) -> Result<Self> {
        let months = n.unsigned_abs() * 12;
        let new_date = if n >= 0 {
            self.inner.checked_add_months(chrono::Months::new(months))
        } else {
            self.inner.checked_sub_months(chrono::Months::new(months))
        }
        .ok_or_else(|| Error::OutOfRange("date overflow".to_string()).into_exception(&ctx))?;
        Ok(Self { inner: new_date })
    }

    #[qjs(rename = "daysBetween")]
    fn days_between<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<Class<'js, Duration>> {
        let other = extract_date(&ctx, &other, "other")?;
        let diff = (other - self.inner).num_days();
        let dur = if diff >= 0 {
            chrono::Duration::days(diff)
        } else {
            -chrono::Duration::days(-diff)
        };
        Class::instance(ctx.clone(), Duration::from_chrono(dur))
    }

    #[qjs(rename = "toIso")]
    fn to_iso(&self) -> String {
        self.inner.format("%Y-%m-%d").to_string()
    }

    #[qjs(rename = "toJsDate")]
    fn to_js_date<'js>(&self, ctx: Ctx<'js>) -> Result<js::Object<'js>> {
        let ctor: Constructor = ctx.globals().get("Date")?;
        let ts = self
            .inner
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as f64;
        ctor.construct((ts,))
    }

    #[qjs(rename = "toString")]
    fn js_to_string(&self) -> String {
        self.to_iso()
    }

    #[qjs(rename = "equals")]
    fn equals<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_date(&ctx, &other, "other")?;
        Ok(self.inner == other)
    }
}

fn extract_duration<'js>(ctx: &Ctx<'js>, val: &Value<'js>, name: &str) -> Result<chrono::Duration> {
    let class = Class::<Duration>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be a Duration")).into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}
