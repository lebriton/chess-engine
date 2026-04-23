use num_enum::{FromPrimitive, IntoPrimitive};
use strum::EnumCount;
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumCountMacro, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

pub const COLOR_COUNT: usize = Color::COUNT;
