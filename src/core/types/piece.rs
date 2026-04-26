use bitfield::bitfield;

use crate::core::types::{color::Color, piece_type::PieceType};

bitfield! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Piece(u8);

    pub u8, from try_into PieceType, piece_type, set_piece_type: 2, 0;
    pub u8, from try_into Color, color, set_color: 3, 3;
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        let mut p = Self(0);
        p.set_piece_type(piece_type);
        p.set_color(color);
        p
    }
}
