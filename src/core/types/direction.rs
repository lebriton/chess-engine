use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum Direction {
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}
