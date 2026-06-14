use crate::js;
use crate::js::JsLifetime;
use crate::js::class::{Trace, Tracer};

#[js::class]
#[derive(JsLifetime, Clone)]
pub struct AbortSignal {
    aborted: bool,
}

impl<'js> Trace<'js> for AbortSignal {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

#[js::methods]
impl AbortSignal {
    #[qjs(get)]
    fn aborted(&self) -> bool {
        self.aborted
    }
}

impl Default for AbortSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl AbortSignal {
    pub fn new() -> Self {
        AbortSignal { aborted: false }
    }

    pub fn abort(&mut self) {
        self.aborted = true;
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
        self.signal.abort();
    }
}
