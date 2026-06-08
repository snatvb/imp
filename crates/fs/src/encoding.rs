use crate::error::Error;

pub enum Encoding {
    Utf8,
    Ascii,
    Latin1,
    Binary,
    Base64,
    Base64Url,
    Hex,
    Buffer,
}

impl Encoding {
    pub fn from_opt(s: Option<&str>) -> Result<Self, Error> {
        let Some(str) = s else {
            return Ok(Encoding::Buffer);
        };
        let res = match str {
            "buffer" | "null" => Encoding::Buffer,
            "utf-8" | "utf8" => Encoding::Utf8,
            "ascii" => Encoding::Ascii,
            "latin1" | "binary" => Encoding::Latin1,
            "base64" => Encoding::Base64,
            "base64url" => Encoding::Base64Url,
            "hex" => Encoding::Hex,
            other => return Err(Error::Encoding(format!("Unsupported encoding: {other}"))),
        };
        Ok(res)
    }
}
