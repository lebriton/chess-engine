use derive_more::{From, Into};

use crate::core::types::{file::File, rank::Rank};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default, From, Into)]
pub struct Square(u8);

impl Square {
    #[inline]
    pub fn index(self) -> u8 {
        self.0
    }

    #[inline]
    pub fn file(self) -> File {
        File::try_from(self.0 % 8).expect("invalid file")
    }

    #[inline]
    pub fn from_coords(file_idx: u8, rank_idx: u8) -> Self {
        let file = File::try_from(file_idx).expect("invalid file");
        let rank = Rank::try_from(rank_idx).expect("invalid rank");

        Self::new(file, rank)
    }

    #[inline]
    pub fn new(file: File, rank: Rank) -> Self {
        let sq = (rank as u8) * 8 + (file as u8);
        Self(sq)
    }

    #[inline]
    pub fn rank(self) -> Rank {
        Rank::try_from(self.0 / 8).expect("invalid rank")
    }
}
