js_core::impl_module!(SubprocessModule,
    declare: |decl, _all| { decl.declare("default")?; Ok(()) },
    evaluate: |_ctx, _exports, _export_all| { Ok(()) },
);
