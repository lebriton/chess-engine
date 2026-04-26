use bitfield::bitfield;

use crate::core::types::{move_flags::MoveFlags, piece_type::PieceType, square::Square};

bitfield! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct ChessMove(u16);

    u8, from try_into Square, destination_square, set_destination_square: 5, 0;
    u8, from try_into Square, origin_square, set_origin_square: 11, 6;
    u8, from try_into PieceType, promotion_piece_type, set_promotion_piece_type: 13, 12;
    u8, from into MoveFlags, special_move_flags, set_special_move_flags: 15, 14;
}
