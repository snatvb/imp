use rquickjs::class::{Trace, Tracer};
use rquickjs::{self as js, Class, Ctx, JsLifetime, Result, Value};

use js_core::error::JsError as _;
use js_core::utils::StringArg;

use crate::error::Error;
use crate::human_parse;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct Duration {
    #[qjs(skip_trace)]
    pub(crate) inner: chrono::Duration,
}

impl<'js> Trace<'js> for Duration {
    fn trace<'a>(&self, _: Tracer<'a, 'js>) {}
}

impl Duration {
    pub fn from_chrono(d: chrono::Duration) -> Self {
        Self { inner: d }
    }

    pub fn to_chrono(&self) -> chrono::Duration {
        self.inner
    }

    fn from_f64_secs(ctx: &Ctx<'_>, n: f64) -> Result<Self> {
        if !n.is_finite() {
            return Err(
                Error::OutOfRange("duration must be a finite number".to_string())
                    .into_exception(ctx),
            );
        }

        // chrono::Duration::from_secs_f64 may panic on NaN/Inf — guarded above
        match chrono::Duration::from_std(std::time::Duration::from_secs_f64(n.abs())) {
            Ok(d) => {
                if n < 0.0 {
                    Ok(Self { inner: -d })
                } else {
                    Ok(Self { inner: d })
                }
            }
            Err(_) => {
                // overflow — build manually from nanos
                if n < 0.0 {
                    Ok(Self {
                        inner: -chrono::Duration::nanoseconds((n * -1e9) as i64),
                    })
                } else {
                    Ok(Self {
                        inner: chrono::Duration::nanoseconds((n * 1e9) as i64),
                    })
                }
            }
        }
    }

    fn from_f64_millis(ctx: &Ctx<'_>, n: f64) -> Result<Self> {
        if !n.is_finite() {
            return Err(
                Error::OutOfRange("duration must be a finite number".to_string())
                    .into_exception(ctx),
            );
        }
        if n == 0.0 {
            return Ok(Self {
                inner: chrono::Duration::zero(),
            });
        }
        let nanos = (n.abs() * 1e6) as i64;
        let d = chrono::Duration::nanoseconds(nanos);
        Ok(if n < 0.0 {
            Self { inner: -d }
        } else {
            Self { inner: d }
        })
    }

    fn from_f64_nanos(ctx: &Ctx<'_>, n: f64) -> Result<Self> {
        if !n.is_finite() {
            return Err(
                Error::OutOfRange("duration must be a finite number".to_string())
                    .into_exception(ctx),
            );
        }
        Ok(Self {
            inner: chrono::Duration::nanoseconds(n as i64),
        })
    }
}

#[js::methods]
impl Duration {
    #[qjs(constructor)]
    fn js_new() -> Self {
        Self {
            inner: chrono::Duration::zero(),
        }
    }

    #[qjs(static, rename = "zero")]
    fn js_zero() -> Self {
        Self {
            inner: chrono::Duration::zero(),
        }
    }

    #[qjs(static, rename = "nanos")]
    fn js_nanos(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_nanos(&ctx, n)
    }

    #[qjs(static, rename = "micros")]
    fn js_micros(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_millis(&ctx, n / 1e3)
    }

    #[qjs(static, rename = "millis")]
    fn js_millis(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_millis(&ctx, n)
    }

    #[qjs(static, rename = "seconds")]
    fn js_seconds(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_secs(&ctx, n)
    }

    #[qjs(static, rename = "minutes")]
    fn js_minutes(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_secs(&ctx, n * 60.0)
    }

    #[qjs(static, rename = "hours")]
    fn js_hours(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_secs(&ctx, n * 3600.0)
    }

    #[qjs(static, rename = "days")]
    fn js_days(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_secs(&ctx, n * 86_400.0)
    }

    #[qjs(static, rename = "weeks")]
    fn js_weeks(ctx: Ctx<'_>, n: f64) -> Result<Self> {
        Self::from_f64_secs(&ctx, n * 604_800.0)
    }

    #[qjs(static, rename = "parse")]
    fn js_parse<'js>(ctx: Ctx<'js>, input: StringArg) -> Result<Self> {
        human_parse::parse(input.as_str())
            .map(Self::from_chrono)
            .map_err(|e| Error::Parse(e.to_string()).into_exception(&ctx))
    }

    #[qjs(rename = "asNanos")]
    fn as_nanos(&self) -> f64 {
        self.inner.num_nanoseconds().unwrap_or(0) as f64
    }

    #[qjs(rename = "asMicros")]
    fn as_micros(&self) -> f64 {
        self.inner.num_microseconds().unwrap_or(0) as f64
    }

    #[qjs(rename = "asMillis")]
    fn as_millis(&self) -> f64 {
        self.inner.num_milliseconds() as f64
    }

    #[qjs(rename = "asSeconds")]
    fn as_seconds(&self) -> f64 {
        self.inner.num_seconds() as f64
    }

    #[qjs(rename = "asMinutes")]
    fn as_minutes(&self) -> f64 {
        self.inner.num_minutes() as f64
    }

    #[qjs(rename = "asHours")]
    fn as_hours(&self) -> f64 {
        self.inner.num_hours() as f64
    }

    #[qjs(rename = "asDays")]
    fn as_days(&self) -> f64 {
        self.inner.num_days() as f64
    }

    #[qjs(rename = "add")]
    fn js_add<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<Self> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(Self {
            inner: self.inner.checked_add(&other).ok_or_else(|| {
                Error::OutOfRange("duration overflow".to_string()).into_exception(&ctx)
            })?,
        })
    }

    #[qjs(rename = "sub")]
    fn js_sub<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<Self> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(Self {
            inner: self.inner.checked_sub(&other).ok_or_else(|| {
                Error::OutOfRange("duration overflow".to_string()).into_exception(&ctx)
            })?,
        })
    }

    #[qjs(rename = "mul")]
    fn js_mul<'js>(&self, ctx: Ctx<'js>, n: f64) -> Result<Self> {
        if !n.is_finite() {
            return Err(
                Error::OutOfRange("multiplier must be finite".to_string()).into_exception(&ctx)
            );
        }
        let nanos = (self.inner.num_nanoseconds().unwrap_or(0) as f64 * n) as i64;
        Ok(Self {
            inner: chrono::Duration::nanoseconds(nanos),
        })
    }

    #[qjs(rename = "neg")]
    fn js_neg(&self) -> Self {
        Self { inner: -self.inner }
    }

    #[qjs(rename = "abs")]
    fn js_abs(&self) -> Self {
        Self {
            inner: self.inner.abs(),
        }
    }

    #[qjs(rename = "isZero")]
    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    #[qjs(rename = "isNegative")]
    fn is_negative(&self) -> bool {
        self.inner < chrono::Duration::zero()
    }

    #[qjs(rename = "eq")]
    fn js_eq<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(self.inner == other)
    }

    #[qjs(rename = "lt")]
    fn js_lt<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(self.inner < other)
    }

    #[qjs(rename = "lte")]
    fn js_lte<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(self.inner <= other)
    }

    #[qjs(rename = "gt")]
    fn js_gt<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(self.inner > other)
    }

    #[qjs(rename = "gte")]
    fn js_gte<'js>(&self, ctx: Ctx<'js>, other: Value<'js>) -> Result<bool> {
        let other = extract_inner(&ctx, &other, "other")?;
        Ok(self.inner >= other)
    }

    #[qjs(rename = "toString")]
    fn js_to_string(&self) -> String {
        self.inner.to_string()
    }
}

fn extract_inner<'js>(ctx: &Ctx<'js>, val: &Value<'js>, name: &str) -> Result<chrono::Duration> {
    let class = Class::<Duration>::from_value(val).map_err(|_| {
        Error::InvalidArg(format!("The \"{name}\" argument must be a Duration")).into_exception(ctx)
    })?;
    Ok(class.borrow().inner)
}
