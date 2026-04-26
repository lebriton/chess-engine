use crate::{
    board::{board::Board, board_squares::BoardSquares},
    core::types::{
        castling_availability::CastlingAvailability, color::Color, piece::Piece,
        piece_type::PieceType, square::Square,
    },
    ensure_ascii, ensure_len,
    util::{ascii_digit, str_to_byte},
};
use anyhow::{Result, anyhow};

pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Color {
    pub fn from_fen(b: u8) -> Result<Self> {
        match b {
            b'b' => Ok(Self::Black),
            b'w' => Ok(Self::White),
            _ => Err(anyhow!("invalid value")),
        }
    }
}

impl Board {
    pub fn from_fen(s: &str) -> Result<Self> {
        ensure_ascii!(s);

        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        ensure_len!(parts, 6);

        let board = Self::builder()
            .squares(BoardSquares::from_fen(parts[0])?)
            .active_color(Color::from_fen(str_to_byte(parts[1])?)?)
            .castling_availability(CastlingAvailability::from_fen(parts[2])?)
            .en_passant_target_square(None) // TODO:
            .halfmove_clock(0) // TODO:
            .fullmove_number(0) // TODO:
            .build();

        Ok(board)
    }
}

impl BoardSquares {
    pub fn from_fen(s: &str) -> Result<Self> {
        ensure_ascii!(s);

        let ranks: Vec<&str> = s.split('/').collect();
        ensure_len!(ranks, 8);

        let mut board_squares = Self::default();

        for (rank_idx, rank) in ranks.iter().enumerate() {
            let rank_idx = rank_idx as u8;
            let mut file_idx = 0;

            for c in rank.bytes() {
                if let Some(ascii_digit) = ascii_digit(c) {
                    file_idx += ascii_digit
                } else {
                    let piece = Piece::from_fen(c)?;
                    let square = Square::from_coords(file_idx, rank_idx);
                    board_squares.set_piece(piece, square);
                    file_idx += 1;
                }
            }
        }

        Ok(board_squares)
    }
}

impl CastlingAvailability {
    pub fn from_fen(s: &str) -> Result<Self> {
        ensure_ascii!(s);

        let mut castling_availability = CastlingAvailability::empty();

        if s == "-" {
            return Ok(castling_availability);
        }

        for c in s.bytes() {
            castling_availability |= match c {
                b'K' => CastlingAvailability::WHITE_KINGSIDE,
                b'Q' => CastlingAvailability::WHITE_QUEENSIDE,
                b'k' => CastlingAvailability::BLACK_KINGSIDE,
                b'q' => CastlingAvailability::BLACK_QUEENSIDE,
                _ => return Err(anyhow!("invalid character")),
            }
        }

        Ok(castling_availability)
    }
}

impl Piece {
    pub fn from_fen(b: u8) -> Result<Self> {
        match b {
            b'P' => Ok(Self::new(Color::White, PieceType::Pawn)),
            b'N' => Ok(Self::new(Color::White, PieceType::Knight)),
            b'B' => Ok(Self::new(Color::White, PieceType::Bishop)),
            b'R' => Ok(Self::new(Color::White, PieceType::Rook)),
            b'Q' => Ok(Self::new(Color::White, PieceType::Queen)),
            b'K' => Ok(Self::new(Color::White, PieceType::King)),

            b'p' => Ok(Self::new(Color::Black, PieceType::Pawn)),
            b'n' => Ok(Self::new(Color::Black, PieceType::Knight)),
            b'b' => Ok(Self::new(Color::Black, PieceType::Bishop)),
            b'r' => Ok(Self::new(Color::Black, PieceType::Rook)),
            b'q' => Ok(Self::new(Color::Black, PieceType::Queen)),
            b'k' => Ok(Self::new(Color::Black, PieceType::King)),
            _ => Err(anyhow!("invalid value")),
        }
    }
}
