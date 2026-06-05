use crate::read;

js_core::impl_module!(FsPromisesModule,
    "readFile" => read::js_read_file,
);
