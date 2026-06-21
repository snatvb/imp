use colored::*;
use js_core::utils::*;
use rquickjs as js;

#[js::function]
pub fn log<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    println!("{}", convert_to_string(&ctx, args.as_slice(), 3, false));
}

#[js::function]
pub fn trace<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    println!(
        "{}\n{}",
        convert_to_string(&ctx, args.as_slice(), 3, false),
        extract_trace(&ctx)
    );
}

#[js::function]
pub fn warn<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    println!(
        "{}\n{}",
        convert_to_string(&ctx, args.as_slice(), 3, false).yellow(),
        extract_trace(&ctx)
    );
}

#[js::function]
pub fn error<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    let error_msg = convert_to_string(&ctx, args.as_slice(), 3, false);
    let mut stack_trace = String::new();

    if let Some(js_stack) = args
        .first()
        .filter(|x| x.is_object())
        .and_then(|x| x.as_object())
        .and_then(|x| x.get::<_, String>("stack").ok())
    {
        stack_trace = js_stack;
    }

    if stack_trace.is_empty() {
        stack_trace = extract_trace(&ctx);
    }

    eprintln!("Error: {}\n{}", error_msg, stack_trace);
}

pub fn create<'a>(ctx: &js::Ctx<'a>) -> js::Result<js::Object<'a>> {
    let console_obj = js::Object::new(ctx.clone())?;
    console_obj.set("log", js_log)?;
    console_obj.set("trace", js_trace)?;
    console_obj.set("error", js_error)?;
    Ok(console_obj)
}
