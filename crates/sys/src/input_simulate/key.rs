use crate::prelude::*;

pub struct Key {
    pub vk_code: u16,
    pub unicode_char: u16,
}

impl Key {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "enter" | "return" => Ok(Self {
                vk_code: 0x0D,
                unicode_char: 13,
            }),
            "escape" | "esc" => Ok(Self {
                vk_code: 0x1B,
                unicode_char: 0,
            }),
            "tab" => Ok(Self {
                vk_code: 0x09,
                unicode_char: 9,
            }),
            "up" => Ok(Self {
                vk_code: 0x26,
                unicode_char: 0,
            }),
            "down" => Ok(Self {
                vk_code: 0x28,
                unicode_char: 0,
            }),
            "left" => Ok(Self {
                vk_code: 0x25,
                unicode_char: 0,
            }),
            "right" => Ok(Self {
                vk_code: 0x27,
                unicode_char: 0,
            }),
            "backspace" => Ok(Self {
                vk_code: 0x08,
                unicode_char: 0,
            }),
            "delete" | "del" => Ok(Self {
                vk_code: 0x2E,
                unicode_char: 0,
            }),
            "space" => Ok(Self {
                vk_code: 0x20,
                unicode_char: 32,
            }),
            "home" => Ok(Self {
                vk_code: 0x24,
                unicode_char: 0,
            }),
            "end" => Ok(Self {
                vk_code: 0x23,
                unicode_char: 0,
            }),
            "pageup" => Ok(Self {
                vk_code: 0x21,
                unicode_char: 0,
            }),
            "pagedown" => Ok(Self {
                vk_code: 0x22,
                unicode_char: 0,
            }),
            "insert" => Ok(Self {
                vk_code: 0x2D,
                unicode_char: 0,
            }),
            s if s.len() == 1 => {
                let c = s.chars().next().unwrap();
                let upper = c.to_uppercase().next().unwrap();
                Ok(Self {
                    vk_code: upper as u16,
                    unicode_char: c as u16,
                })
            }
            other => Err(Error::UnknownKey(other.to_string())),
        }
    }
}
