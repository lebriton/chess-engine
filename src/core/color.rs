use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, Eq, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}
