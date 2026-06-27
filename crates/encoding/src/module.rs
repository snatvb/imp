js_core::impl_module!(EncodingModule,
    "base64" => { encode => crate::base64::js_encode, decode => crate::base64::js_decode },
    "hex"    => { encode => crate::hex::js_encode, decode => crate::hex::js_decode },
    "utf8"   => { encode => crate::utf8::js_encode, decode => crate::utf8::js_decode },
    "uri"    => { encode => crate::uri::js_encode, decode => crate::uri::js_decode },
);
