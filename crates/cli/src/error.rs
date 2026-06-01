use js_core::js::{self, CatchResultExt};

pub fn expect_js<'js, T>(ctx: &js::Ctx<'js>, result: js::Result<T>, context: &str) -> T {
    match result.catch(ctx) {
        Ok(val) => val,
        Err(e) => panic!("{context}: {e}"),
    }
}
