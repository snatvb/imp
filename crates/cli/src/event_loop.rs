use std::io::Write;
use std::time::{Duration, Instant};

use js_core::timers::JsTimers;

use crate::prelude::*;
use js::promise::PromiseState;

pub async fn run_event_loop<'js>(
    ctx: &js::Ctx<'js>,
    js_timers: JsTimers,
    exit_handle: Option<process::ExitHandle>,
    early_exit: Option<js::Promise<'js>>,
) {
    let mut last = Instant::now();
    let mut released = Vec::with_capacity(8);
    let mut idle_rounds: u32 = 0;

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

        if let Some(ref handle) = exit_handle
            && handle.is_requested()
        {
            handle.run_listeners(ctx);
            let _ = std::io::stdout().flush();
            let _ = std::io::stderr().flush();
            break;
        }

        let early_done = early_exit
            .as_ref()
            .map(|p| !matches!(p.state(), PromiseState::Pending))
            .unwrap_or(false);

        let idle = {
            let timers = js_timers.borrow();
            (early_done || early_exit.is_none())
                && timers.timers.is_empty()
                && timers.released.is_empty()
        };

        if idle {
            #[cfg(not(debug_assertions))]
            break;

            let has_jobs = is_job_pending(ctx);
            if has_jobs {
                idle_rounds = 0;
            } else {
                idle_rounds += 1;
                const GUARD_CYCLES: u32 = 100;
                if idle_rounds >= GUARD_CYCLES {
                    break;
                }
            }
        } else {
            idle_rounds = 0;
        }

        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tracing::info!("event loop finished");
}

#[inline]
fn is_job_pending(ctx: &js::Ctx<'_>) -> bool {
    // unsafe neccesary to get rt ptr because using rt has_jobs leads to deadlock
    // loop in signle thread, no datarace
    unsafe {
        let rt_ptr = js::qjs::JS_GetRuntime(ctx.as_raw().as_ptr());
        js::qjs::JS_IsJobPending(rt_ptr)
    }
}
