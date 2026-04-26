use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::EnumCount;
use strum_macros::EnumCount as EnumCountMacro;

#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, EnumCountMacro, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    #[inline]
    pub fn is_black(self) -> bool {
        matches!(self, Color::Black)
    }

    #[inline]
    pub fn is_opposite(self, other: Color) -> bool {
        self != other
    }

    #[inline]
    pub fn is_same(self, other: Color) -> bool {
        self == other
    }

    #[inline]
    pub fn is_white(self) -> bool {
        matches!(self, Color::White)
    }

    #[inline]
    pub fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

pub const COLOR_COUNT: usize = Color::COUNT;
