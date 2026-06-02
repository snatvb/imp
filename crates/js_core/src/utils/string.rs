use crate::js::Ctx;

pub fn extract_trace(ctx: &Ctx) -> String {
    ctx.eval("new Error().stack")
        .map(|s: String| s.lines().skip(1).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|_| "No stack trace available".to_string())
}

pub trait JsString {
    fn js_string(&self) -> String;
}

impl<T: AsRef<str>> JsString for T {
    fn js_string(&self) -> String {
        self.as_ref()
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
}
