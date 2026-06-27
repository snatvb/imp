use js_core::error::JsError;
use js_core::js;
use js_core::js::function::Opt;

use hmac::{Hmac, KeyInit, Mac};
use sha2::{Sha256, Sha512};

use crate::common::{encode_output, extract_bytes, parse_encoding};
use crate::error::CryptoError;

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;

fn hmac_sign(key: &[u8], data: &[u8], algo: &str) -> Result<Vec<u8>, CryptoError> {
    match algo {
        "sha256" => {
            let mut mac = HmacSha256::new_from_slice(key)
                .map_err(|e| CryptoError::invalid_value(format!("hmac key error: {e}")))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "sha512" => {
            let mut mac = HmacSha512::new_from_slice(key)
                .map_err(|e| CryptoError::invalid_value(format!("hmac key error: {e}")))?;
            mac.update(data);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        _ => Err(CryptoError::invalid_value(format!(
            "unsupported hmac algorithm: \"{algo}\". Supported: sha256, sha512"
        ))),
    }
}

#[js::function]
pub fn hmac<'js>(
    ctx: js::Ctx<'js>,
    algo: String,
    key: js::Value<'js>,
    data: js::Value<'js>,
    encoding: Opt<String>,
) -> js::Result<js::Value<'js>> {
    let key_bytes = extract_bytes(&ctx, &key, "key")?;
    let data_bytes = extract_bytes(&ctx, &data, "data")?;
    let result = hmac_sign(&key_bytes, &data_bytes, &algo).map_err(|e| e.into_exception(&ctx))?;
    let enc = parse_encoding(encoding.0.as_deref());
    encode_output(&ctx, &result, enc)
}
