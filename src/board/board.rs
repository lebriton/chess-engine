use crate::core::types::{
    bitboard::BitBoard, color::COLOR_COUNT, piece::Piece, piece_type::PIECE_TYPE_COUNT,
    square::Square,
};

const BOTH_COLORS_INDEX: usize = 2;
const OCCUPANCY_SIZE: usize = COLOR_COUNT + 1;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Board {
    pub pieces: [[BitBoard; PIECE_TYPE_COUNT]; COLOR_COUNT],
    pub occupancies: [BitBoard; OCCUPANCY_SIZE],
}

impl Board {
    #[inline]
    fn bb_clear(&mut self, bb: BitBoard, color: usize, piece_type: usize) {
        self.pieces[color][piece_type] &= !bb;
        self.occupancies[color] &= !bb;
        self.occupancies[BOTH_COLORS_INDEX] &= !bb;
    }

    #[inline]
    fn bb_set(&mut self, bb: BitBoard, color: usize, piece_type: usize) {
        self.pieces[color][piece_type] |= bb;
        self.occupancies[color] |= bb;
        self.occupancies[BOTH_COLORS_INDEX] |= bb;
    }

    #[inline]
    pub fn clear_piece(&mut self, piece: Piece, square: Square) {
        let bb = BitBoard::from(square);
        self.bb_clear(bb, piece.color() as usize, piece.piece_type() as usize);
    }

    pub fn new() -> Self {
        Self {
            pieces: [[BitBoard::EMPTY; PIECE_TYPE_COUNT]; COLOR_COUNT],
            occupancies: [BitBoard::EMPTY; OCCUPANCY_SIZE],
        }
    }

    #[inline]
    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        let bb = BitBoard::from(square);
        self.bb_set(bb, piece.color() as usize, piece.piece_type() as usize);
    }
}
