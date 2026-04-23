use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, FromPrimitive, IntoPrimitive)]
#[repr(i8)]
pub enum Direction {
    #[default]
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}
