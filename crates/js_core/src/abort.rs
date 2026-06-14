use crate::js;
use crate::js::JsLifetime;
use crate::js::class::{Trace, Tracer};
use tokio_util::sync::CancellationToken;

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct AbortSignal {
    aborted: bool,
    reason: String,
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
            aborted: false,
            reason: String::new(),
            token: CancellationToken::new(),
        }
    }

    pub fn abort(&mut self, reason: &str) {
        self.aborted = true;
        self.reason = reason.to_string();
        self.token.cancel();
    }

    pub fn token(&self) -> &CancellationToken {
        &self.token
    }
}

#[js::methods]
impl AbortSignal {
    #[qjs(get)]
    fn aborted(&self) -> bool {
        self.aborted
    }

    #[qjs(get)]
    fn reason(&self) -> String {
        self.reason.clone()
    }
}

impl AbortSignal {
    pub fn is_aborted(&self) -> bool {
        self.aborted
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

    fn abort(&mut self) {
        self.signal.abort("The operation was aborted");
    }
}
