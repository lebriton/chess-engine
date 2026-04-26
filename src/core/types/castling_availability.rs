use bitflags::bitflags;

use crate::impl_bitflags_u8;

bitflags! {
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct CastlingAvailability: u8 {
        const WHITE_KINGSIDE = 1 << 0;
        const WHITE_QUEENSIDE = 1 << 1;
        const BLACK_KINGSIDE = 1 << 2;
        const BLACK_QUEENSIDE = 1 << 3;
    }
}

impl_bitflags_u8!(CastlingAvailability);
