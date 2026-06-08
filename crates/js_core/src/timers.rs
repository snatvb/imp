use std::{cell::RefCell, collections::VecDeque, rc::Rc, time::Duration};

use crate::js;

#[derive(Debug, Clone)]
pub enum TimerKind {
    Once,
    Repeating { interval: Duration },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TimerId(u64);

impl TimerId {
    fn take_next(&mut self) -> Self {
        self.0 += 1;
        Self(self.0)
    }
}

impl From<u64> for TimerId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<TimerId> for u64 {
    fn from(id: TimerId) -> Self {
        id.0
    }
}

impl From<f64> for TimerId {
    fn from(id: f64) -> Self {
        Self(id as u64)
    }
}

impl From<TimerId> for f64 {
    fn from(id: TimerId) -> Self {
        id.0 as f64
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub id: TimerId,
    pub callback: js::Persistent<js::Function<'static>>,
    pub kind: TimerKind,
    pub remaining: Duration,
}

#[derive(Debug, Clone)]
pub struct ReleasedTimer {
    pub id: TimerId,
    pub cb: js::Persistent<js::Function<'static>>,
}

#[derive(Debug, Default)]
pub struct Timers {
    pub timers: Vec<Timer>,
    pub released: VecDeque<ReleasedTimer>,
    next_id: TimerId,
}

impl Timers {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(&mut self, dt: Duration) {
        let mut i = 0;
        while i < self.timers.len() {
            let t = &mut self.timers[i];
            t.remaining = t.remaining.saturating_sub(dt);
            if t.remaining > Duration::ZERO {
                i += 1;
                continue;
            }

            match t.kind {
                TimerKind::Once => {
                    let t = self.timers.swap_remove(i);
                    self.released.push_back(ReleasedTimer {
                        id: t.id,
                        cb: t.callback,
                    });
                }
                TimerKind::Repeating { interval } => {
                    self.released.push_back(ReleasedTimer {
                        id: t.id,
                        cb: t.callback.clone(),
                    });
                    t.remaining = interval;
                    i += 1;
                }
            }
        }
    }

    pub fn set_timeout<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        callback: js::Function<'js>,
        delay: Duration,
    ) -> TimerId {
        let id = self.next_id.take_next();
        self.timers.push(Timer {
            id,
            callback: js::Persistent::save(ctx, callback),
            kind: TimerKind::Once,
            remaining: delay,
        });
        id
    }

    pub fn set_interval<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        callback: js::Function<'js>,
        delay: Duration,
    ) -> TimerId {
        let id = self.next_id.take_next();
        self.timers.push(Timer {
            id,
            callback: js::Persistent::save(ctx, callback),
            kind: TimerKind::Repeating { interval: delay },
            remaining: delay,
        });
        id
    }

    pub fn remove(&mut self, id: TimerId) {
        self.timers.retain(|t| t.id != id);
        self.released.retain(|t| t.id != id);
    }

    pub fn clear(&mut self) {
        self.timers.clear();
        self.released.clear();
    }

    pub fn pop_one(&mut self) -> Option<ReleasedTimer> {
        self.released.pop_front()
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsTimers(Rc<RefCell<Timers>>);

impl std::ops::Deref for JsTimers {
    type Target = RefCell<Timers>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsTimers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bind_to<'js>(&self, ctx: &js::Ctx<'js>, obj: &js::Object<'js>) -> js::Result<()> {
        let timers = self.clone();
        obj.set(
            "setTimeout",
            js::Function::new(
                ctx.clone(),
                move |cb: js::Function<'js>, delay: f64| -> f64 {
                    let ctx = cb.ctx().clone();
                    let id = timers.borrow_mut().set_timeout(
                        &ctx,
                        cb,
                        Duration::from_millis(delay as u64),
                    );
                    id.into()
                },
            )?,
        )?;

        let timers = self.clone();
        obj.set(
            "setInterval",
            js::Function::new(
                ctx.clone(),
                move |cb: js::Function<'js>, delay: f64| -> f64 {
                    let ctx = cb.ctx().clone();
                    let id = timers.borrow_mut().set_interval(
                        &ctx,
                        cb,
                        Duration::from_millis(delay as u64),
                    );
                    id.into()
                },
            )?,
        )?;

        let timers = self.clone();
        let clear_timer = js::Function::new(ctx.clone(), move |id: f64| {
            timers.borrow_mut().remove(id.into());
        })?;

        obj.set("clearInterval", clear_timer.clone())?;
        obj.set("clearTimeout", clear_timer)?;

        Ok(())
    }
}
