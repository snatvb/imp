use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use rquickjs as js;
use tokio::sync::mpsc;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum SignalName {
    SigInt,
    SigTerm,
    SigHup,
    SigQuit,
    SigBreak,
}

impl SignalName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SigInt => "SIGINT",
            Self::SigTerm => "SIGTERM",
            Self::SigHup => "SIGHUP",
            Self::SigQuit => "SIGQUIT",
            Self::SigBreak => "SIGBREAK",
        }
    }

    pub fn from_js(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "SIGINT" => Some(Self::SigInt),
            "SIGTERM" => Some(Self::SigTerm),
            "SIGHUP" => Some(Self::SigHup),
            "SIGQUIT" => Some(Self::SigQuit),
            "SIGBREAK" => Some(Self::SigBreak),
            _ => None,
        }
    }

    pub fn all() -> &'static [SignalName] {
        &[
            Self::SigInt,
            Self::SigTerm,
            Self::SigHup,
            Self::SigQuit,
            Self::SigBreak,
        ]
    }
}

static HANDLER_ID: AtomicU64 = AtomicU64::new(1);

pub struct HandlerEntry {
    pub id: u64,
    pub handler: js::Persistent<js::Function<'static>>,
    pub once: bool,
}

struct SignalInner {
    handlers: HashMap<SignalName, Vec<HandlerEntry>>,
    active_listeners: HashSet<SignalName>,
    pending: Vec<SignalName>,
}

thread_local! {
    static SIGNAL_RX: RefCell<Option<mpsc::UnboundedReceiver<SignalName>>> = const { RefCell::new(None) };
}

pub struct SignalHandle {
    inner: Rc<RefCell<SignalInner>>,
    tx: mpsc::UnboundedSender<SignalName>,
}

impl Clone for SignalHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            tx: self.tx.clone(),
        }
    }
}

impl Default for SignalHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalHandle {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        SIGNAL_RX.with(|slot| {
            *slot.borrow_mut() = Some(rx);
        });
        Self {
            inner: Rc::new(RefCell::new(SignalInner {
                handlers: HashMap::new(),
                active_listeners: HashSet::new(),
                pending: Vec::new(),
            })),
            tx,
        }
    }

    pub fn add_handler(
        &self,
        name: SignalName,
        handler: js::Persistent<js::Function<'static>>,
        once: bool,
    ) -> u64 {
        let id = HANDLER_ID.fetch_add(1, Ordering::Relaxed);
        let mut inner = self.inner.borrow_mut();
        inner
            .handlers
            .entry(name)
            .or_default()
            .push(HandlerEntry { id, handler, once });
        id
    }

    pub fn remove_handler(&self, name: SignalName, id: u64) {
        let mut inner = self.inner.borrow_mut();
        if let Some(entries) = inner.handlers.get_mut(&name) {
            entries.retain(|e| e.id != id);
        }
    }

    pub fn remove_all(&self, name: Option<SignalName>) {
        let mut inner = self.inner.borrow_mut();
        match name {
            Some(n) => {
                inner.handlers.remove(&n);
            }
            None => {
                inner.handlers.clear();
            }
        }
    }

    pub fn should_listen(&self, name: SignalName) -> bool {
        let mut inner = self.inner.borrow_mut();
        if inner.active_listeners.contains(&name) {
            return false;
        }
        inner.active_listeners.insert(name);
        true
    }

    pub fn tx(&self) -> &mpsc::UnboundedSender<SignalName> {
        &self.tx
    }

    pub fn drain(&self, ctx: &js::Ctx<'_>) {
        let signals: Vec<SignalName> = SIGNAL_RX.with(|slot| {
            let mut slot = slot.borrow_mut();
            let rx = match slot.as_mut() {
                Some(rx) => rx,
                None => return Vec::new(),
            };
            let mut batch = Vec::new();
            while let Ok(name) = rx.try_recv() {
                batch.push(name);
            }
            batch
        });

        if signals.is_empty() {
            return;
        }

        let to_call: Vec<_> = {
            let mut inner = self.inner.borrow_mut();

            for name in &signals {
                inner.pending.push(*name);
            }

            let mut result = Vec::new();
            for name in &signals {
                if let Some(entries) = inner.handlers.get_mut(name) {
                    let mut remaining = Vec::new();
                    for entry in entries.drain(..) {
                        if entry.once {
                            result.push(entry.handler);
                        } else {
                            remaining.push(entry);
                        }
                    }
                    *entries = remaining;
                }
            }
            inner.pending.drain(..);
            result
        };

        for handler in to_call {
            if let Ok(cb) = handler.restore(ctx)
                && let Err(e) = cb.call::<_, ()>(())
            {
                eprintln!("signal: handler error: {e}");
            }
        }
    }

    pub fn pending_signals(&self) -> Vec<SignalName> {
        let inner = self.inner.borrow();
        inner.pending.clone()
    }
}

thread_local! {
    static HANDLE: RefCell<Option<SignalHandle>> = const { RefCell::new(None) };
}

pub fn set_handle(handle: SignalHandle) {
    HANDLE.with(|h| {
        *h.borrow_mut() = Some(handle);
    });
}

pub fn with_handle<R>(f: impl FnOnce(&SignalHandle) -> R) -> Option<R> {
    HANDLE.with(|h| {
        let borrow = h.borrow();
        borrow.as_ref().map(f)
    })
}
