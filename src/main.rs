use crate::{board::board::Board, core::types::square::Square, notation::fen};

mod board;
mod core;
mod macros;
mod notation;
mod util;

fn main() {
    let mut board = Board::from_fen(fen::STARTING_POSITION).unwrap();

    let from = Square::from_san("e2").unwrap();
    let to = Square::from_san("e4").unwrap();

    board.do_move(from, to);
}
