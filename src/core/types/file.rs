use num_enum::{FromPrimitive, IntoPrimitive};
use strum::EnumCount;
use strum_macros::EnumCount as EnumCountMacro;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumCountMacro, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum File {
    #[default]
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

pub const FILE_COUNT: usize = File::COUNT;
