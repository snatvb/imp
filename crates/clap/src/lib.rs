use js_core::js;

js_core::impl_module!(ClapModule,
    "parse" => js_parse,
);

#[js::function]
fn parse() -> js::Result<()> {
    todo!()
}

pub fn set_script_args<'js>(ctx: &js::Ctx<'js>, args: Vec<String>) -> js::Result<()> {
    let globals = ctx.globals();
    let js_args = js::Array::new(ctx.clone())?;
    for (i, arg) in args.into_iter().enumerate() {
        js_args.set(i, arg)?;
    }
    globals.set("__scriptArgs__", js_args)?;
    Ok(())
}
