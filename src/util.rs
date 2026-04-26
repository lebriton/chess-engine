use anyhow::Result;

use crate::ensure_len;

#[inline]
pub const fn ascii_delta_unchecked(b: u8, base: u8) -> u8 {
    b - base
}

pub fn ascii_digit(b: u8) -> Option<u8> {
    b.is_ascii_digit().then(|| ascii_delta_unchecked(b, b'0'))
}

pub fn str_to_byte(s: &str) -> Result<u8> {
    let bytes = s.as_bytes();
    ensure_len!(bytes, 1);

    Ok(bytes[0])
}
