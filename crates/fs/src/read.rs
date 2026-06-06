use base64::Engine;
use js_core::error::SystemError;
use js_core::js::IntoJs;
use js_core::js::function;
use tokio::fs;

use crate::{encoding::Encoding, error::Error, prelude::*};

// to have possibility to update
#[inline(always)]
#[tracing::instrument(level = "trace", skip(path), fields(path = %path))]
pub async fn read(path: &str) -> std::io::Result<Vec<u8>> {
    fs::read(path).await
}

#[function]
#[tracing::instrument(level = "debug", skip(ctx, path, encoding), fields(path = %path))]
pub async fn read_file<'js>(
    ctx: js::Ctx<'js>,
    path: String,
    encoding: function::Opt<String>,
) -> js::Result<js::Value<'js>> {
    let encoding = Encoding::from_opt(encoding.as_deref()).map_err(|e| e.into_exception(&ctx))?;
    let raw = read(&path)
        .await
        .map_err(|e| SystemError::from_io(e, "read", Some(path.clone())).into_exception(&ctx))?;
    tracing::debug!(bytes = raw.len(), "read_file raw bytes");

    let value = match encoding {
        Encoding::Buffer => js::ArrayBuffer::new(ctx.clone(), raw)?.into_value(),
        Encoding::Utf8 => {
            let s = String::from_utf8(raw)
                .map_err(|e| Error::Encoding(e.to_string()).into_exception(&ctx))?;
            s.into_js(&ctx)?
        }
        Encoding::Ascii => {
            let decoder = encoding_rs::Encoding::for_label(b"ascii").unwrap();
            let (s, _, _) = decoder.decode(&raw);
            s.into_js(&ctx)?
        }
        Encoding::Latin1 | Encoding::Binary => {
            let (s, _, _) = encoding_rs::WINDOWS_1252.decode(&raw);
            s.into_js(&ctx)?
        }
        Encoding::Base64 => base64::engine::general_purpose::STANDARD
            .encode(&raw)
            .into_js(&ctx)?,
        Encoding::Base64Url => base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(&raw)
            .into_js(&ctx)?,
        Encoding::Hex => hex::encode(&raw).into_js(&ctx)?,
    };

    Ok(value)
}
