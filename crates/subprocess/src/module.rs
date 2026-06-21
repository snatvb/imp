use crate::run;

js_core::impl_module!(SubprocessModule,
    "run" => run::js_run,
);
