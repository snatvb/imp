#[macro_export]
macro_rules! register_native_modules {
    ($loader:expr, $resolver:expr, $(($name:expr, $mod_val:expr)),+ $(,)?) => {
        $(
            $loader.add_module($name, $mod_val);
            $resolver.add_module($name);
        )*
    };
}
