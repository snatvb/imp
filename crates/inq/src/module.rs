use crate::prompts;

js_core::impl_module!(InquireModule,
    "prompt" => prompts::js_prompt,
    "select" => prompts::js_select,
);
