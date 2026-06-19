//! Module registration for `imp:time`.
//!
//! All chrono classes are also registered as JS globals in `create_globals`,
//! so users can call `Duration.seconds(2)` without importing. The `imp:time`
//! module re-exports them for explicit imports.

use rquickjs::Class;
use rquickjs::Ctx;
use rquickjs::Result;

use crate::date::ImpDate;
use crate::datetime::ImpDateTime;
use crate::datetime::ImpLocalDateTime;
use crate::duration::Duration;
use crate::time::ImpTime;

js_core::impl_module!(
    TimeModule,
    declare: |decl, _all| {
        decl.declare("Duration")?;
        decl.declare("ImpDate")?;
        decl.declare("ImpTime")?;
        decl.declare("ImpDateTime")?;
        decl.declare("ImpLocalDateTime")?;
        decl.declare("default")?;
        Ok(())
    },
    evaluate: |ctx, exports, _all| {
        let duration = class_ctor::<Duration>(ctx)?;
        let imp_date = class_ctor::<ImpDate>(ctx)?;
        let imp_time = class_ctor::<ImpTime>(ctx)?;
        let imp_datetime = class_ctor::<ImpDateTime>(ctx)?;
        let imp_local = class_ctor::<ImpLocalDateTime>(ctx)?;

        let ns = rquickjs::Object::new(ctx.clone())?;
        ns.set("Duration", duration.clone())?;
        ns.set("ImpDate", imp_date.clone())?;
        ns.set("ImpTime", imp_time.clone())?;
        ns.set("ImpDateTime", imp_datetime.clone())?;
        ns.set("ImpLocalDateTime", imp_local.clone())?;

        exports.export("Duration", duration)?;
        exports.export("ImpDate", imp_date)?;
        exports.export("ImpTime", imp_time)?;
        exports.export("ImpDateTime", imp_datetime)?;
        exports.export("ImpLocalDateTime", imp_local)?;
        exports.export("default", ns)?;
        Ok(())
    },
);

fn class_ctor<'js, T: rquickjs::class::JsClass<'js>>(
    ctx: &Ctx<'js>,
) -> Result<rquickjs::Value<'js>> {
    let ctor = Class::<T>::create_constructor(ctx)?.ok_or(rquickjs::Error::Exception)?;
    Ok(ctor.into_value())
}

/// Register all chrono classes on the JS global object so they can be used
/// without an explicit import (`Duration.seconds(30)` works directly).
pub fn create_globals<'js>(ctx: &Ctx<'js>) -> Result<()> {
    Class::<Duration>::define(&ctx.globals())?;
    Class::<ImpDate>::define(&ctx.globals())?;
    Class::<ImpTime>::define(&ctx.globals())?;
    Class::<ImpDateTime>::define(&ctx.globals())?;
    Class::<ImpLocalDateTime>::define(&ctx.globals())?;
    Ok(())
}
