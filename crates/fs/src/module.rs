use crate::imp::{expand_home, write};
use crate::read;

js_core::impl_module!(FsPromisesModule,
    "readFile" => read::js_read_file,
    "writeFile" => write::js_write_file,
    "expandHome" => expand_home::js_expand_home,
);
