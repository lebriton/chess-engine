use num_enum::TryFromPrimitive;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    #[inline]
    pub fn lsb(self) -> Option<u8> {
        if self.0 == 0 {
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

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
#[repr(i8)]
pub enum Direction {
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}
