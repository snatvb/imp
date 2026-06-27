js_core::impl_module!(ParsersModule,
    "json"    => { parse => crate::json::js_parse,       stringify => crate::json::js_stringify },
    "yaml"    => { parse => crate::yaml::js_parse,       stringify => crate::yaml::js_stringify },
    "xml"     => { parse => crate::xml::js_parse,        stringify => crate::xml::js_stringify },
    "toml"    => { parse => crate::toml::js_parse,       stringify => crate::toml::js_stringify },
    "ron"     => { parse => crate::ron::js_parse,        stringify => crate::ron::js_stringify },
    "csv"     => { parse => crate::csv::js_parse,        stringify => crate::csv::js_stringify },
    "msgpack" => { parse => crate::msgpack::js_parse,    stringify => crate::msgpack::js_stringify },
);
