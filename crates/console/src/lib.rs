use colored::*;
use js_core::utils::*;
use rquickjs as js;

pub fn log<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    println!("{}", convert_to_string(&ctx, args.as_slice(), 3, false));
}

pub fn trace<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    println!(
        "{}\n{}",
        convert_to_string(&ctx, args.as_slice(), 3, false),
        extract_trace(&ctx)
    );
}

pub fn warn<'js>(ctx: js::Ctx<'js>, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    println!(
        "{}\n{}",
        convert_to_string(&ctx, args.as_slice(), 3, false).yellow(),
        extract_trace(&ctx)
    );
}

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

pub fn assert<'js>(ctx: js::Ctx<'js>, condition: bool, js::prelude::Rest(args): js::prelude::Rest<js::Value<'js>>) {
    if !condition {
        let msg = if args.is_empty() {
            "console.assert".to_string()
        } else {
            convert_to_string(&ctx, args.as_slice(), 3, false)
        };
        println!("{}", msg.yellow());
    }
}

pub fn create<'a>(ctx: &js::Ctx<'a>) -> js::Result<js::Object<'a>> {
    let console_obj = js::Object::new(ctx.clone())?;
    console_obj.set("log", js::Function::new(ctx.clone(), log)?)?;
    console_obj.set("trace", js::Function::new(ctx.clone(), trace)?)?;
    console_obj.set("error", js::Function::new(ctx.clone(), error)?)?;
    console_obj.set("assert", js::Function::new(ctx.clone(), assert)?)?;
    Ok(console_obj)
}
