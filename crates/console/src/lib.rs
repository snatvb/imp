use colored::*;
use js_core::utils::*;
use rquickjs as js;

pub fn log(_ctx: js::Ctx<'_>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'_>>) {
    println!("{}", convert_to_string(args.as_slice()));
}

pub fn trace(ctx: js::Ctx<'_>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'_>>) {
    println!(
        "{}\n{}",
        convert_to_string(args.as_slice()),
        extract_trace(&ctx)
    );
}

pub fn warn(ctx: js::Ctx<'_>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'_>>) {
    println!(
        "{}\n{}",
        convert_to_string(args.as_slice()).yellow(),
        extract_trace(&ctx)
    );
}

pub fn error(ctx: js::Ctx<'_>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'_>>) {
    let error_msg = convert_to_string(args.as_slice());
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
    console_obj.set("log", js::Function::new(ctx.clone(), log)?)?;
    console_obj.set("trace", js::Function::new(ctx.clone(), trace)?)?;
    console_obj.set("error", js::Function::new(ctx.clone(), error)?)?;
    Ok(console_obj)
}
