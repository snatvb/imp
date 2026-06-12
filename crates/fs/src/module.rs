use crate::read;
use crate::imp::write;

js_core::impl_module!(FsPromisesModule,
    "readFile" => read::js_read_file,
    "writeFile" => write::js_write_file,
);
