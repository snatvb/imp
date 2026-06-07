use crate::prompts;

js_core::impl_module!(InquireModule,
    "prompt" => prompts::js_prompt,
    "select" => prompts::js_select,
    "multiSelect" => prompts::js_multi_select,
    "password" => prompts::js_password,
    "passwordWithConfirm" => prompts::js_password_with_confirm,
    "editor" => prompts::js_editor,
    "dateSelect" => prompts::js_date_select,
);
