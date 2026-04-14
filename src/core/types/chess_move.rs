use bitfield::bitfield;
use bitflags::bitflags;

bitflags! {
    pub struct MoveFlags: u8 {
        const NORMAL = 1 << 0;
        const PROMOTION = 1 << 1;
        const EN_PASSANT = 1 << 2;
        const CASTLING = 1 << 3;
    }
}

bitfield! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ChessMove(u16);

    u8, destination_square, set_destination_square: 5, 0;
    u8, origin_square, set_origin_square: 11, 6;
    u8, promotion_piece_type, set_promotion_piece_type: 13, 12;
    u8, special_move_flags, set_special_move_flags: 15, 14;
}
