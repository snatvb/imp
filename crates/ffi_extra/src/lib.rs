pub mod js_helpers;
pub mod module_meta;
pub mod os_info;

use rquickjs as js;

const _: () = assert!(
    std::mem::size_of::<js::module::Module<'_, ()>>() == 2 * std::mem::size_of::<usize>(),
    "Module layout changed — transmute in module_meta is broken"
);
