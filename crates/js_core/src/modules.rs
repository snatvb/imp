#[macro_export]
macro_rules! register_native_modules {
    ($loader:expr, $resolver:expr, $(($name:expr, $mod_val:expr)),+ $(,)?) => {
        $(
            $loader.add_module($name, $mod_val);
            $resolver.add_module($name);
        )*
    };
}

#[macro_export]
macro_rules! impl_module {
    ($name:ident, $($js_name:literal => $func:expr),+ $(,)?) => {
        pub struct $name;

        impl $crate::js::module::ModuleDef for $name {
            fn declare<'js>(decl: &$crate::js::module::Declarations<'js>) -> $crate::js::Result<()> {
                $(decl.declare($js_name)?;)+
                decl.declare("default")?;
                Ok(())
            }

            fn evaluate<'js>(ctx: &$crate::js::Ctx<'js>, exports: &$crate::js::module::Exports<'js>) -> $crate::js::Result<()> {
                let ns = $crate::js::Object::new(ctx.clone())?;
                $(
                    let val = $crate::js::IntoJs::into_js($func, ctx)?;
                    ns.set($js_name, val.clone())?;
                    exports.export($js_name, val)?;
                )+
                exports.export("default", ns)?;
                Ok(())
            }
        }
    };

    ($name:ident,
     evaluate: |$eval_ctx:ident, $eval_exports:ident, $eval_all:ident| $eval_body:tt,
     $($js_name:literal => $func:expr),* $(,)?) => {
        pub struct $name;

        impl $crate::js::module::ModuleDef for $name {
            fn declare<'js>(decl: &$crate::js::module::Declarations<'js>) -> $crate::js::Result<()> {
                $(decl.declare($js_name)?;)*
                decl.declare("default")?;
                Ok(())
            }

            fn evaluate<'js>(ctx: &$crate::js::Ctx<'js>, exports: &$crate::js::module::Exports<'js>) -> $crate::js::Result<()> {
                fn export_all<'js>(ctx: &$crate::js::Ctx<'js>, exports: &$crate::js::module::Exports<'js>) -> $crate::js::Result<$crate::js::Object<'js>> {
                    let ns = $crate::js::Object::new(ctx.clone())?;
                    $(
                        let val = $crate::js::IntoJs::into_js($func, ctx)?;
                        ns.set($js_name, val.clone())?;
                        exports.export($js_name, val)?;
                    )*
                    Ok(ns)
                }
                let $eval_ctx = ctx;
                let $eval_exports = exports;
                $eval_body
            }
        }
    };

    ($name:ident,
     declare: |$decl_decl:ident, $decl_all:ident| $decl_body:tt,
     evaluate: |$eval_ctx:ident, $eval_exports:ident, $eval_all:ident| $eval_body:tt,
     $($js_name:literal => $func:expr),* $(,)?) => {
        pub struct $name;

        impl $crate::js::module::ModuleDef for $name {
            fn declare<'js>(decl: &$crate::js::module::Declarations<'js>) -> $crate::js::Result<()> {
                fn $decl_all(decl: &$crate::js::module::Declarations<'_>) -> $crate::js::Result<()> {
                    $(decl.declare($js_name)?;)*
                    Ok(())
                }
                let $decl_decl = decl;
                $decl_body
            }

            fn evaluate<'js>(ctx: &$crate::js::Ctx<'js>, exports: &$crate::js::module::Exports<'js>) -> $crate::js::Result<()> {
                fn export_all<'js>(ctx: &$crate::js::Ctx<'js>, exports: &$crate::js::module::Exports<'js>) -> $crate::js::Result<$crate::js::Object<'js>> {
                    let ns = $crate::js::Object::new(ctx.clone())?;
                    $(
                        let val = $crate::js::IntoJs::into_js($func, ctx)?;
                        ns.set($js_name, val.clone())?;
                        exports.export($js_name, val)?;
                    )*
                    Ok(ns)
                }
                let $eval_ctx = ctx;
                let $eval_exports = exports;
                $eval_body
            }
        }
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
