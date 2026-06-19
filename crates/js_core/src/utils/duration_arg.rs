use std::fmt;

use crate::coerce::{js_type_of, throw_type_error};
use crate::js::{self, Ctx, FromJs, Type, Value};
use crate::object::ObjectMethodExt;

/// A value extracted from JS that represents a duration.
///
/// Accepts either a `number` (interpreted as milliseconds) or a JS object
/// exposing an `asMillis()` method (the contract for `Duration` class in
/// `imp:time`).
///
/// String values are **not** accepted here — callers must use an explicit
/// `Duration.parse(s)` first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationArg {
    Millis(u64),
}

impl DurationArg {
    pub fn as_millis(&self) -> u64 {
        match self {
            DurationArg::Millis(m) => *m,
        }
    }
}

impl From<DurationArg> for std::time::Duration {
    fn from(arg: DurationArg) -> Self {
        std::time::Duration::from_millis(arg.as_millis())
    }
}

fn throw_range<'js>(ctx: &Ctx<'js>, msg: impl AsRef<str>) -> js::Error {
    let make = || -> js::Result<js::Error> {
        let ctor: js::function::Constructor = ctx.globals().get("RangeError")?;
        let err: js::Object = ctor.construct((msg.as_ref(),))?;
        err.set("code", "ERR_INVALID_ARG_VALUE")?;
        Ok(ctx.throw(err.into()))
    };
    make().unwrap_or_else(|e| e)
}

pub trait JsDurationArg<'js>: Sized {
    fn to_duration_arg(self, ctx: &Ctx<'js>) -> js::Result<DurationArg>;

    #[inline(always)]
    fn coerce_js(
        ctx: &Ctx<'js>,
        val: &Value<'js>,
        name: impl fmt::Display,
    ) -> js::Result<DurationArg> {
        val.clone().to_duration_arg(ctx).map_err(|_| {
            let received = js_type_of(val);
            let msg = format!(
                "The \"{name}\" argument must be of type Duration or number. Received {received}"
            );
            throw_type_error(ctx, "ERR_INVALID_ARG_TYPE", &msg)
        })
    }
}

fn check_finite_ms<'js>(ctx: &Ctx<'js>, n: f64) -> js::Result<DurationArg> {
    if !n.is_finite() || n < 0.0 {
        return Err(throw_range(
            ctx,
            "duration must be a non-negative finite number of milliseconds",
        ));
    }
    Ok(DurationArg::Millis(n as u64))
}

impl<'js> JsDurationArg<'js> for f64 {
    fn to_duration_arg(self, ctx: &Ctx<'js>) -> js::Result<DurationArg> {
        check_finite_ms(ctx, self)
    }
}

impl<'js> JsDurationArg<'js> for u64 {
    fn to_duration_arg(self, _: &Ctx<'js>) -> js::Result<DurationArg> {
        Ok(DurationArg::Millis(self))
    }
}

impl<'js> JsDurationArg<'js> for i64 {
    fn to_duration_arg(self, ctx: &Ctx<'js>) -> js::Result<DurationArg> {
        if self < 0 {
            return Err(throw_range(
                ctx,
                "duration must be a non-negative integer (milliseconds)",
            ));
        }
        Ok(DurationArg::Millis(self as u64))
    }
}

impl<'js> JsDurationArg<'js> for u32 {
    fn to_duration_arg(self, _: &Ctx<'js>) -> js::Result<DurationArg> {
        Ok(DurationArg::Millis(self as u64))
    }
}

impl<'js> JsDurationArg<'js> for i32 {
    fn to_duration_arg(self, ctx: &Ctx<'js>) -> js::Result<DurationArg> {
        if self < 0 {
            return Err(throw_range(
                ctx,
                "duration must be a non-negative integer (milliseconds)",
            ));
        }
        Ok(DurationArg::Millis(self as u64))
    }
}

impl<'js> JsDurationArg<'js> for Value<'js> {
    fn to_duration_arg(self, ctx: &Ctx<'js>) -> js::Result<DurationArg> {
        match self.type_of() {
            Type::Int => {
                let n = self.as_int().unwrap_or(0);
                if n < 0 {
                    return Err(throw_range(
                        ctx,
                        "duration must be a non-negative integer (milliseconds)",
                    ));
                }
                Ok(DurationArg::Millis(n as u64))
            }
            Type::Float => check_finite_ms(ctx, self.as_float().unwrap_or(0.0)),
            Type::Object | Type::Constructor | Type::Promise | Type::Proxy => {
                let obj = self.as_object().ok_or_else(|| {
                    let received = js_type_of(&self);
                    throw_type_error(
                        ctx,
                        "ERR_INVALID_ARG_TYPE",
                        &format!(
                            "The \"duration\" argument must be of type Duration or number. Received {received}"
                        ),
                    )
                })?;
                let ms = obj.call_method::<_, f64>("asMillis", ()).map_err(|_| {
                    let received = js_type_of(&self);
                    throw_type_error(
                        ctx,
                        "ERR_INVALID_ARG_TYPE",
                        &format!(
                            "The \"duration\" argument must be of type Duration or number. Received {received}"
                        ),
                    )
                })?;
                check_finite_ms(ctx, ms)
            }
            _ => {
                let received = js_type_of(&self);
                Err(throw_type_error(
                    ctx,
                    "ERR_INVALID_ARG_TYPE",
                    &format!(
                        "The \"duration\" argument must be of type Duration or number. Received {received}"
                    ),
                ))
            }
        }
    }
}

impl<'js> FromJs<'js> for DurationArg {
    fn from_js(ctx: &Ctx<'js>, value: Value<'js>) -> js::Result<Self> {
        value.to_duration_arg(ctx)
    }
}

impl<'js> JsDurationArg<'js> for DurationArg {
    fn to_duration_arg(self, _: &Ctx<'js>) -> js::Result<DurationArg> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_millis_passthrough() {
        assert_eq!(DurationArg::Millis(42).as_millis(), 42);
    }

    #[test]
    fn into_std_duration() {
        let d: std::time::Duration = DurationArg::Millis(250).into();
        assert_eq!(d, std::time::Duration::from_millis(250));
    }

    #[test]
    fn zero_millis() {
        assert_eq!(DurationArg::Millis(0).as_millis(), 0);
    }

    #[test]
    fn large_millis() {
        assert_eq!(DurationArg::Millis(u64::MAX).as_millis(), u64::MAX);
    }
}
