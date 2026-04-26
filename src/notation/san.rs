use crate::{
    core::types::{file::File, rank::Rank, square::Square},
    ensure_ascii, ensure_len,
    util::ascii_delta_unchecked,
};
use anyhow::{Result, anyhow};

impl Square {
    pub fn from_san(s: &str) -> Result<Self> {
        ensure_ascii!(s);
        ensure_len!(s, 2);

        // We use `as_bytes()` instead of `.chars()` because chess square notation
        // is guaranteed to be ASCII ("a1"–"h8"). This avoids UTF-8 decoding,
        // allocations, and iterator overhead, giving direct O(1) byte access.
        let bytes = s.as_bytes();

        let file = match bytes[0] {
            b'a'..=b'h' => File::try_from(ascii_delta_unchecked(bytes[0], b'a'))?,
            _ => return Err(anyhow!("invalid file")),
        };

        let rank = match bytes[1] {
            b'1'..=b'8' => Rank::try_from(ascii_delta_unchecked(bytes[1], b'1'))?,
            _ => return Err(anyhow!("invalid rank")),
        };

        Ok(Self::new(file, rank))
    }
}
