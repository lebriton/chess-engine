use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, From, Into, Not,
};

use crate::core::types::square::Square;

const EMPTY_VALUE: u64 = 0;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Hash,
    Debug,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Not,
    From,
    Into,
)]
pub struct BitBoard(u64);

impl BitBoard {
    pub const EMPTY: Self = BitBoard(EMPTY_VALUE);

    #[inline]
    pub fn intersects(self, other: BitBoard) -> bool {
        (self & other).0 != EMPTY_VALUE
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == EMPTY_VALUE
    }

    #[inline]
    pub fn lsb(self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    #[inline]
    pub fn pop_lsb(&mut self) -> Option<u8> {
        let square = self.lsb()?;
        self.0 &= self.0 - 1;
        Some(square)
    }
}

impl From<Square> for BitBoard {
    #[inline]
    fn from(square: Square) -> Self {
        Self(1u64 << square.index())
    }
}
