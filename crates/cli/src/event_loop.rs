use std::time::{Duration, Instant};

use js_core::timers::JsTimers;

use crate::prelude::*;

pub async fn run_event_loop<'js>(ctx: &js::Ctx<'js>, _rt: &js::AsyncRuntime, js_timers: JsTimers) {
    let mut last = Instant::now();
    let mut released = Vec::with_capacity(8);
    loop {
        let now = Instant::now();
        let dt = now.duration_since(last);
        last = now;

        {
            let mut timers = js_timers.borrow_mut();
            timers.tick(dt);
            while let Some(r) = timers.pop_one() {
                released.push(r);
            }
        }

        while let Some(r) = released.pop() {
            match r.cb.restore(ctx) {
                Ok(cb) => {
                    let _ = cb.call::<_, ()>(());
                }
                Err(e) => {
                    tracing::error!(?e, "failed to restore timer callback");
                }
            }
        }

        tokio::task::yield_now().await;

        {
            let timers = js_timers.borrow();
            if timers.timers.is_empty() && timers.released.is_empty() {
                break;
            }
        }

        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tracing::info!("event loop finished");
}
