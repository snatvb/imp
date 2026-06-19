use crate::js;
use crate::js::JsLifetime;
use crate::js::class::{Trace, Tracer};
use crate::js::function::Opt;
use crate::utils::DurationArg;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio_util::sync::CancellationToken;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct AbortSignal {
    #[qjs(skip_trace)]
    aborted: Arc<AtomicBool>,
    #[qjs(skip_trace)]
    reason: Arc<Mutex<String>>,
    #[qjs(skip_trace)]
    token: CancellationToken,
}

impl<'js> Trace<'js> for AbortSignal {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl Default for AbortSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl AbortSignal {
    pub fn new() -> Self {
        AbortSignal {
            aborted: Arc::new(AtomicBool::new(false)),
            reason: Arc::new(Mutex::new(String::new())),
            token: CancellationToken::new(),
        }
    }

    pub fn abort(&mut self, reason: &str) {
        self.aborted.store(true, Ordering::Relaxed);
        *self.reason.lock().unwrap() = reason.to_string();
        self.token.cancel();
    }

    pub fn token(&self) -> &CancellationToken {
        &self.token
    }
}

#[js::methods]
impl AbortSignal {
    #[qjs(constructor)]
    fn constructor() -> Self {
        AbortSignal::new()
    }

    #[qjs(get)]
    fn aborted(&self) -> bool {
        self.aborted.load(Ordering::Relaxed)
    }

    #[qjs(get)]
    fn reason(&self) -> String {
        self.reason.lock().unwrap().clone()
    }

    #[qjs(static)]
    fn timeout(d: DurationArg) -> AbortSignal {
        let signal = AbortSignal::new();
        let token = signal.token().clone();
        let a = signal.aborted.clone();
        let r = signal.reason.clone();
        let delay: std::time::Duration = d.into();

        tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            a.store(true, Ordering::Relaxed);
            *r.lock().unwrap() = "The operation timed out".to_string();
            token.cancel();
        });

        signal
    }
}

impl AbortSignal {
    pub fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::Relaxed)
    }
}

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct AbortController {
    signal: AbortSignal,
}

impl<'js> Trace<'js> for AbortController {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::methods]
impl AbortController {
    #[qjs(constructor)]
    fn new() -> Self {
        AbortController {
            signal: AbortSignal::new(),
        }
    }

    #[qjs(get)]
    fn signal(&self) -> AbortSignal {
        self.signal.clone()
    }

    fn abort(&mut self, reason: Opt<String>) {
        let msg = match reason.0 {
            Some(val) => val,
            None => "The operation was aborted".to_string(),
        };
        self.signal.abort(&msg);
    }
}
