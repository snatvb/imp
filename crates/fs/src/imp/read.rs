use base64::Engine;
use js_core::RsString;
use js_core::error::SystemError;
use js_core::js::Class;
use js_core::js::function;
use tokio::fs;

use crate::{encoding::Encoding, error::Error, prelude::*};

#[function]
pub async fn read_file<'js>(
    ctx: js::Ctx<'js>,
    path: String,
    encoding: function::Opt<String>,
) -> js::Result<js::Value<'js>> {
    let encoding = Encoding::from_opt(encoding.as_deref()).map_err(|e| e.into_exception(&ctx))?;
    let raw = fs::read(&path)
        .await
        .map_err(|e| SystemError::from_io(e, "read", Some(path.clone())).into_exception(&ctx))?;
    tracing::debug!(bytes = raw.len(), "imp:fs readFile raw bytes");

    match encoding {
        Encoding::Buffer => Ok(js::ArrayBuffer::new(ctx.clone(), raw)?.into_value()),
        Encoding::Utf8 => {
            let s = String::from_utf8(raw)
                .map_err(|e| Error::Encoding(e.to_string()).into_exception(&ctx))?;
            Ok(Class::instance(ctx.clone(), RsString::owned(s))?.into_value())
        }
        Encoding::Ascii => {
            let decoder = encoding_rs::Encoding::for_label(b"ascii").unwrap();
            let (s, _, _) = decoder.decode(&raw);
            Ok(Class::instance(ctx.clone(), RsString::owned(s.into_owned()))?.into_value())
        }
        Encoding::Latin1 | Encoding::Binary => {
            let (s, _, _) = encoding_rs::WINDOWS_1252.decode(&raw);
            Ok(Class::instance(ctx.clone(), RsString::owned(s.into_owned()))?.into_value())
        }
        Encoding::Base64 => {
            let s = base64::engine::general_purpose::STANDARD.encode(&raw);
            Ok(Class::instance(ctx.clone(), RsString::owned(s))?.into_value())
        }
        Encoding::Base64Url => {
            let s = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&raw);
            Ok(Class::instance(ctx.clone(), RsString::owned(s))?.into_value())
        }
        Encoding::Hex => {
            let s = hex::encode(&raw);
            Ok(Class::instance(ctx.clone(), RsString::owned(s))?.into_value())
        }
    }
}
