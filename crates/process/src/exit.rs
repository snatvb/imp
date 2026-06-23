use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use rquickjs as js;

struct ExitState {
    requested: AtomicBool,
    exit_code: AtomicI32,
    listeners: RefCell<Vec<js::Persistent<js::Function<'static>>>>,
}

#[derive(Clone)]
pub struct ExitHandle(Rc<ExitState>);

impl Default for ExitHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl ExitHandle {
    pub fn new() -> Self {
        Self(Rc::new(ExitState {
            requested: AtomicBool::new(false),
            exit_code: AtomicI32::new(0),
            listeners: RefCell::new(Vec::new()),
        }))
    }

    pub fn is_requested(&self) -> bool {
        self.0.requested.load(Ordering::Relaxed)
    }

    pub fn exit_code(&self) -> i32 {
        self.0.exit_code.load(Ordering::Relaxed)
    }

    pub fn request_exit(&self, code: i32) {
        self.0.requested.store(true, Ordering::Relaxed);
        self.0.exit_code.store(code, Ordering::Relaxed);
    }

    pub fn set_exit_code(&self, code: i32) {
        self.0.exit_code.store(code, Ordering::Relaxed);
    }

    pub fn add_listener(&self, cb: js::Persistent<js::Function<'static>>) {
        self.0.listeners.borrow_mut().push(cb);
    }

    pub fn run_listeners(&self, ctx: &js::Ctx<'_>) {
        let listeners: Vec<_> = self.0.listeners.borrow_mut().drain(..).collect();
        let code = self.exit_code();
        for listener in listeners {
            if let Ok(cb) = listener.clone().restore(ctx) {
                let _ = cb.call::<_, ()>((code,));
            }
        }
    }
}
