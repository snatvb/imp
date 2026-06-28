use std::io::Write;
use std::time::{Duration, Instant};

use js_core::timers::JsTimers;

use crate::prelude::*;
use js::Exception;
use js::promise::PromiseState;

#[allow(clippy::collapsible_if)]
pub async fn run_event_loop<'js>(
    ctx: &js::Ctx<'js>,
    js_timers: JsTimers,
    exit_handle: Option<process::ExitHandle>,
    early_exit: Option<js::Promise<'js>>,
) {
    let mut last = Instant::now();
    let mut released = Vec::with_capacity(8);
    #[cfg(debug_assertions)]
    let mut idle_rounds: u32 = 0;
    let mut rejection_handled = false;

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

        signal::with_handle(|h| h.drain(ctx));

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

        if early_done && !rejection_handled {
            rejection_handled = true;
            if let Some(ref p) = early_exit {
                if matches!(p.state(), PromiseState::Rejected) {
                    let _ = p.result::<js::Value>();
                    let reason = ctx.catch();
                    let msg = reason
                        .clone()
                        .into_object()
                        .and_then(Exception::from_object)
                        .map(|exc| {
                            let msg = exc.message().unwrap_or_default();
                            let stack = exc.stack().unwrap_or_default();
                            format!("{msg}\n{stack}")
                        })
                        .unwrap_or_else(|| format!("{reason:?}"));
                    eprintln!("Uncaught (in module) {msg}");
                    if let Some(ref h) = exit_handle {
                        h.set_exit_code(1);
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        let idle = {
            let timers = js_timers.borrow();
            (early_done || early_exit.is_none())
                && timers.timers.is_empty()
                && timers.released.is_empty()
        };

        #[cfg(debug_assertions)]
        if idle {
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
#[cfg(debug_assertions)]
fn is_job_pending(ctx: &js::Ctx<'_>) -> bool {
    ffi_extra::js_helpers::is_job_pending(ctx)
}
