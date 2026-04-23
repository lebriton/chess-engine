use num_enum::{FromPrimitive, IntoPrimitive};
use strum::EnumCount;
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumCountMacro, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PieceType {
    #[default]
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

pub const PIECE_TYPE_COUNT: usize = PieceType::COUNT;
