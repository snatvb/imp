macro_rules! make_path_wrappers {
    ($B:ty, $sep:expr, $delim:expr) => {
        #[function]
        pub fn resolve<'js>(
            ctx: js::Ctx<'js>,
            args: js::prelude::Rest<js::Value<'js>>,
        ) -> js::Result<String> {
            crate::path_impl::resolve::<$B>(&ctx, args)
        }

        #[function]
        pub fn join<'js>(
            ctx: js::Ctx<'js>,
            args: js::prelude::Rest<js::Value<'js>>,
        ) -> js::Result<String> {
            crate::path_impl::join::<$B>(&ctx, args, $sep)
        }

        #[function]
        pub fn basename<'js>(
            ctx: js::Ctx<'js>,
            path: js::Value<'js>,
            suffix: js::prelude::Opt<js::Value<'js>>,
        ) -> js::Result<String> {
            crate::path_impl::basename::<$B>(&ctx, path, suffix)
        }

        #[function]
        pub fn dirname<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
            crate::path_impl::dirname::<$B>(&ctx, path)
        }

        #[function]
        pub fn extname<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
            crate::path_impl::extname::<$B>(&ctx, path)
        }

        #[function]
        pub fn normalize<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<String> {
            crate::path_impl::normalize::<$B>(&ctx, path)
        }

        #[function]
        pub fn is_absolute<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<bool> {
            crate::path_impl::is_absolute::<$B>(&ctx, path)
        }

        #[function]
        pub fn format<'js>(ctx: js::Ctx<'js>, arg: js::Value<'js>) -> js::Result<String> {
            crate::path_impl::format::<$B>(&ctx, arg, $sep)
        }

        #[function]
        pub fn parse<'js>(ctx: js::Ctx<'js>, path: js::Value<'js>) -> js::Result<js::Object<'js>> {
            crate::path_impl::parse::<$B>(&ctx, path)
        }

        #[function]
        pub fn to_namespaced_path<'js>(
            ctx: js::Ctx<'js>,
            path: js::Value<'js>,
        ) -> js::Result<js::Value<'js>> {
            crate::path_impl::to_namespaced_path::<$B>(&ctx, path)
        }

        #[function]
        pub fn relative<'js>(
            ctx: js::Ctx<'js>,
            from: js::Value<'js>,
            to: js::Value<'js>,
        ) -> js::Result<String> {
            crate::path_impl::relative::<$B>(&ctx, from, to)
        }

        pub fn populate<'js>(ns: &js::Object<'js>) -> js::Result<()> {
            ns.set("resolve", js_resolve)?;
            ns.set("join", js_join)?;
            ns.set("basename", js_basename)?;
            ns.set("dirname", js_dirname)?;
            ns.set("extname", js_extname)?;
            ns.set("normalize", js_normalize)?;
            ns.set("isAbsolute", js_is_absolute)?;
            ns.set("format", js_format)?;
            ns.set("parse", js_parse)?;
            ns.set("relative", js_relative)?;
            ns.set("toNamespacedPath", js_to_namespaced_path)?;
            ns.set("sep", $sep)?;
            ns.set("delimiter", $delim)?;
            Ok(())
        }
    };
}
pub(crate) use make_path_wrappers;
