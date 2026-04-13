use bitfield::bitfield;
use bitflags::bitflags;

bitflags! {
    pub struct MoveFlags: u8 {
        const CAPTURE = 1 << 0;
        const EN_PASSANT = 1 << 1;
        const CASTLING = 1 << 2;
        const PROMOTION = 1 << 3;
    }
}

bitfield! {
    pub struct ChessMove(u32);

    u8, from, set_from: 5, 0;
    u8, to, set_to: 11, 6;
    u8, promotion, set_promotion: 14, 12;
    u8, piece, set_piece: 17, 15;
    u8, flags, set_flags: 21, 18;
}
