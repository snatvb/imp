js_core::impl_module!(HashModule,
    "md5" => crate::md5::js_md5,
    "sha1" => crate::sha1::js_sha1,
    "sha256" => crate::sha2::js_sha256,
    "sha512" => crate::sha2::js_sha512,
    "blake3" => crate::blake3::js_blake3,
);
