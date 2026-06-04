use std::time::Instant;

use crate::js;

static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

fn now() -> f64 {
    let start = START.get_or_init(Instant::now);
    start.elapsed().as_secs_f64() * 1000.0
}

pub fn create<'a>(ctx: &js::Ctx<'a>) -> js::Result<js::Object<'a>> {
    let perf_obj = js::Object::new(ctx.clone())?;
    perf_obj.set("now", js::Function::new(ctx.clone(), now)?)?;
    Ok(perf_obj)
}
