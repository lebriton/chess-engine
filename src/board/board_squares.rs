use crate::core::types::{
    bitboard::BitBoard, color::COLOR_COUNT, piece::Piece, piece_type::PIECE_TYPE_COUNT,
    square::Square,
};

const SQUARE_COUNT: usize = 64;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BoardSquares {
    pub pieces: [[BitBoard; PIECE_TYPE_COUNT]; COLOR_COUNT],
    pub occupancies: [BitBoard; COLOR_COUNT],
    pub all_occupancy: BitBoard,

    pub squares: [Option<Piece>; SQUARE_COUNT],
}

impl BoardSquares {
    #[inline]
    pub fn clear_piece(&mut self, piece: Piece, square: Square) {
        let bitboard = BitBoard::from(square);
        let color = piece.color().unwrap() as usize;
        let piece_type = piece.piece_type().unwrap() as usize;

        self.pieces[color][piece_type] &= !bitboard;
        self.occupancies[color] &= !bitboard;
        self.all_occupancy &= !bitboard;

        self.squares[square.index() as usize] = None;
    }

    #[inline]
    pub fn is_occupied(&self, square: Square) -> bool {
        self.all_occupancy.intersects(BitBoard::from(square))
    }

    /// Moves a piece from `from` to `to`.
    ///
    /// # Panics
    /// This function assumes the move is **legal and already validated**.
    /// It will panic if `from` does not contain a piece.
    ///
    /// No move legality checking is performed here (e.g. pins, turn, rules).
    #[inline]
    pub fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self
            .piece_on(from)
            .expect("attempted to move from an empty square");

        if let Some(capture) = self.piece_on(to) {
            self.clear_piece(capture, to);
        }

        self.set_piece(piece, to);
    }

    #[inline]
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        self.squares[square.index() as usize]
    }

    #[inline]
    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        let bitboard = BitBoard::from(square);
        let color = piece.color().unwrap() as usize;
        let piece_type = piece.piece_type().unwrap() as usize;

        self.pieces[color][piece_type] |= bitboard;
        self.occupancies[color] |= bitboard;
        self.all_occupancy |= bitboard;

        self.squares[square.index() as usize] = Some(piece);
    }
}

impl Default for BoardSquares {
    fn default() -> Self {
        Self {
            pieces: [[BitBoard::EMPTY; PIECE_TYPE_COUNT]; COLOR_COUNT],
            occupancies: [BitBoard::EMPTY; COLOR_COUNT],
            all_occupancy: BitBoard::EMPTY,
            squares: [None; SQUARE_COUNT],
        }
    }
}
