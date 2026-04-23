use bitflags::bitflags;

bitflags! {
    pub struct MoveFlags: u8 {
        const NORMAL = 1 << 0;
        const PROMOTION = 1 << 1;
        const EN_PASSANT = 1 << 2;
        const CASTLING = 1 << 3;
    }
}

impl From<u8> for MoveFlags {
    fn from(v: u8) -> Self {
        MoveFlags::from_bits_truncate(v)
    }
}

impl From<MoveFlags> for u8 {
    fn from(flags: MoveFlags) -> Self {
        flags.bits()
    }
}
