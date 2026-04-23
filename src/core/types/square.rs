use crate::core::types::{file::File, rank::Rank};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Square(u8);

impl Square {
    #[inline]
    pub fn index(self) -> u8 {
        self.0
    }

    #[inline]
    pub fn file(self) -> File {
        File::from(self.0 % 8)
    }

    #[inline]
    pub fn new(file: File, rank: Rank) -> Self {
        let sq = (rank as u8) * 8 + (file as u8);
        Square(sq)
    }

    #[inline]
    pub fn rank(self) -> Rank {
        Rank::from(self.0 / 8)
    }
}
