use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Debug,
    EnumCountMacro,
    EnumIter,
    IntoPrimitive,
    TryFromPrimitive,
)]
#[repr(u8)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

pub const RANK_COUNT: usize = Rank::COUNT;
