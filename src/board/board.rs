use strum::IntoEnumIterator;
use typed_builder::TypedBuilder;

use crate::{
    board::board_squares::BoardSquares,
    core::types::{
        castling_availability::CastlingAvailability, color::Color, file::File, rank::Rank,
        square::Square,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TypedBuilder)]
pub struct Board {
    #[builder(default)]
    squares: BoardSquares,

    #[builder(default = Color::White)]
    active_color: Color,

    #[builder(default = CastlingAvailability::empty())]
    castling_availability: CastlingAvailability,

    #[builder(default)]
    en_passant_target_square: Option<Square>,

    #[builder(default = 0)]
    halfmove_clock: u16,

    #[builder(default = 0)]
    fullmove_number: u16,
}

impl Board {
    pub fn do_move(&mut self, from: Square, to: Square) {
        self.squares.move_piece(from, to);

        self.active_color = self.active_color.opposite();

        // TODO:
    }

    pub fn iter_squares(&self) -> impl Iterator<Item = Square> {
        let rank_iter = Rank::iter().rev();
        rank_iter.flat_map(|rank| File::iter().map(move |file| Square::new(file, rank)))
    }
}
