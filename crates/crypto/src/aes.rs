use js_core::error::JsError;
use js_core::js;

use aes_gcm::{
    Aes128Gcm, Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use rand::RngExt;

use crate::common::extract_bytes;
use crate::error::CryptoError;

const NONCE_LEN: usize = 12;

enum AeadCipher {
    Aes128(Box<Aes128Gcm>),
    Aes256(Box<Aes256Gcm>),
}

fn make_cipher(algo: &str, key: &[u8]) -> Result<AeadCipher, CryptoError> {
    match algo {
        "aes-128-gcm" => {
            if key.len() != 16 {
                return Err(CryptoError::invalid_value(format!(
                    "aes-128-gcm requires a 16-byte key, got {}",
                    key.len()
                )));
            }
            let cipher = Aes128Gcm::new_from_slice(key)
                .map_err(|e| CryptoError::invalid_value(format!("cipher init error: {e}")))?;
            Ok(AeadCipher::Aes128(Box::new(cipher)))
        }
        "aes-256-gcm" => {
            if key.len() != 32 {
                return Err(CryptoError::invalid_value(format!(
                    "aes-256-gcm requires a 32-byte key, got {}",
                    key.len()
                )));
            }
            let cipher = Aes256Gcm::new_from_slice(key)
                .map_err(|e| CryptoError::invalid_value(format!("cipher init error: {e}")))?;
            Ok(AeadCipher::Aes256(Box::new(cipher)))
        }
        _ => Err(CryptoError::invalid_value(format!(
            "unsupported AEAD algorithm: \"{algo}\". Supported: aes-128-gcm, aes-256-gcm"
        ))),
    }
}

fn encrypt_bytes(algo: &str, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = make_cipher(algo, key)?;
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = match cipher {
        AeadCipher::Aes128(c) => c.encrypt(nonce, plaintext),
        AeadCipher::Aes256(c) => c.encrypt(nonce, plaintext),
    }
    .map_err(|e| CryptoError::invalid_value(format!("encryption failed: {e}")))?;

    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn decrypt_bytes(algo: &str, key: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if data.len() < NONCE_LEN {
        return Err(CryptoError::invalid_value(
            "ciphertext too short: missing nonce",
        ));
    }
    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = make_cipher(algo, key)?;

    match cipher {
        AeadCipher::Aes128(c) => c.decrypt(nonce, ciphertext),
        AeadCipher::Aes256(c) => c.decrypt(nonce, ciphertext),
    }
    .map_err(|e| {
        CryptoError::invalid_value(format!(
            "decryption failed (wrong key or tampered data): {e}"
        ))
    })
}

#[js::function]
pub fn aes_encrypt<'js>(
    ctx: js::Ctx<'js>,
    algo: String,
    key: js::Value<'js>,
    plaintext: js::Value<'js>,
) -> js::Result<js::Value<'js>> {
    let key_bytes = extract_bytes(&ctx, &key, "key")?;
    let pt_bytes = extract_bytes(&ctx, &plaintext, "plaintext")?;
    let result = encrypt_bytes(&algo, &key_bytes, &pt_bytes).map_err(|e| e.into_exception(&ctx))?;
    Ok(js::Class::instance(
        ctx.clone(),
        js_core::byte_buffer::ByteBuffer::new(ctx.clone(), result)?,
    )?
    .into_value())
}

#[js::function]
pub fn aes_decrypt<'js>(
    ctx: js::Ctx<'js>,
    algo: String,
    key: js::Value<'js>,
    ciphertext: js::Value<'js>,
) -> js::Result<js::Value<'js>> {
    let key_bytes = extract_bytes(&ctx, &key, "key")?;
    let ct_bytes = extract_bytes(&ctx, &ciphertext, "ciphertext")?;
    let result = decrypt_bytes(&algo, &key_bytes, &ct_bytes).map_err(|e| e.into_exception(&ctx))?;
    Ok(js::Class::instance(
        ctx.clone(),
        js_core::byte_buffer::ByteBuffer::new(ctx.clone(), result)?,
    )?
    .into_value())
}
