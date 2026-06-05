use js_core::js::{self, CatchResultExt};

pub fn try_js<'js, T>(ctx: &js::Ctx<'js>, result: js::Result<T>, context: &str) -> Option<T> {
    match result.catch(ctx) {
        Ok(val) => Some(val),
        Err(e) => {
            eprintln!("{context}: {e}");
            None
        }
    }
}
