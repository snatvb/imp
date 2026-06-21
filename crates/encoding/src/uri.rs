use js_core::error::JsError;
use js_core::js;
use js_core::utils::StringArg;

use crate::error::EncodingError;

fn is_unreserved(b: u8) -> bool {
    matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~')
}

fn hex_upper(b: u8) -> [u8; 2] {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    [HEX[(b >> 4) as usize], HEX[(b & 0x0F) as usize]]
}

fn encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    for &b in bytes {
        if is_unreserved(b) {
            out.push(b as char);
        } else {
            let h = hex_upper(b);
            out.push('%');
            out.push(h[0] as char);
            out.push(h[1] as char);
        }
    }
    out
}

fn decode<'js>(ctx: &js::Ctx<'js>, input: &str) -> js::Result<String> {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'%' {
            if i + 2 >= bytes.len() {
                return Err(EncodingError::InvalidUri(format!(
                    "incomplete percent escape at position {i}"
                ))
                .into_exception(ctx));
            }
            let h = (bytes[i + 1] as char).to_digit(16).ok_or_else(|| {
                EncodingError::InvalidUri(format!("invalid percent escape at position {i}"))
                    .into_exception(ctx)
            })?;
            let l = (bytes[i + 2] as char).to_digit(16).ok_or_else(|| {
                EncodingError::InvalidUri(format!("invalid percent escape at position {i}"))
                    .into_exception(ctx)
            })?;
            out.push((h * 16 + l) as u8);
            i += 3;
        } else {
            out.push(b);
            i += 1;
        }
    }
    String::from_utf8(out).map_err(|e| EncodingError::InvalidUri(e.to_string()).into_exception(ctx))
}

#[js::function]
pub fn js_encode<'js>(ctx: js::Ctx<'js>, input: StringArg) -> js::Result<js::String<'js>> {
    let s = encode(input.as_str());
    js::String::from_str(ctx, &s)
}

#[js::function]
pub fn js_decode<'js>(ctx: js::Ctx<'js>, input: StringArg) -> js::Result<js::String<'js>> {
    let s = decode(&ctx, input.as_str())?;
    js::String::from_str(ctx, &s)
}

pub fn make_module<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Object<'js>> {
    let obj = js::Object::new(ctx.clone())?;
    obj.set("encode", js_js_encode)?;
    obj.set("decode", js_js_decode)?;
    Ok(obj)
}
