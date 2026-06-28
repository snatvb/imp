use crate::js;

#[derive(Default)]
pub struct PipeOptions {
    pub prevent_close: bool,
    pub prevent_abort: bool,
    pub prevent_cancel: bool,
}

impl PipeOptions {
    pub fn from_js_object(options: &js::Object<'_>) -> js::Result<Self> {
        let prevent_close: bool = options.get("preventClose").unwrap_or(false);
        let prevent_abort: bool = options.get("preventAbort").unwrap_or(false);
        let prevent_cancel: bool = options.get("preventCancel").unwrap_or(false);

        Ok(Self {
            prevent_close,
            prevent_abort,
            prevent_cancel,
        })
    }
}
