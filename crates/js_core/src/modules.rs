#[macro_export]
macro_rules! register_native_modules {
    ($loader:expr, $resolver:expr, $(($name:expr, $mod_val:expr)),+ $(,)?) => {
        $(
            $loader.add_module($name, $mod_val);
            $resolver.add_module($name);
        )*
    };
}

pub fn export_ns<'js>(
    ctx: &crate::js::Ctx<'js>,
    exports: &crate::js::module::Exports<'js>,
    entries: &[(&str, crate::js::Value<'js>)],
) -> crate::js::Result<()> {
    let ns = crate::js::Object::new(ctx.clone())?;
    for (name, value) in entries {
        ns.set(*name, value.clone())?;
        exports.export(*name, value.clone())?;
    }
    exports.export("default", ns)?;
    Ok(())
}
