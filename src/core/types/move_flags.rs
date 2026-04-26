use bitflags::bitflags;

use crate::impl_bitflags_u8;

bitflags! {
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct MoveFlags: u8 {
        const NORMAL = 1 << 0;
        const PROMOTION = 1 << 1;
        const EN_PASSANT = 1 << 2;
        const CASTLING = 1 << 3;
    }
}

impl_bitflags_u8!(MoveFlags);
