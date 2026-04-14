use bitfield::bitfield;
use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

bitfield! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Piece(u8);

    u8, piece_type, set_piece_type: 2, 0;
    bool, color, set_color: 3;
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        let mut p = Piece(0);
        p.set_piece_type(piece_type as u8);
        p.set_color(matches!(color, Color::Black));
        p
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}
