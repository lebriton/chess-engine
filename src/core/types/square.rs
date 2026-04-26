use derive_more::Into;

use crate::core::types::{file::File, rank::Rank};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug, Default, Into)]
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
    pub fn new(file: File, rank: Rank) -> Self {
        let sq = (rank as u8) * 8 + (file as u8);
        Self(sq)
    }

    #[inline]
    pub fn rank(self) -> Rank {
        Rank::try_from(self.0 / 8).expect("invalid rank")
    }

    #[inline]
    pub fn try_from_coords(file_idx: u8, rank_idx: u8) -> Self {
        let file = File::try_from(file_idx).expect("invalid file");
        let rank = Rank::try_from(rank_idx).expect("invalid rank");

        Self::new(file, rank)
    }
}

impl TryFrom<u8> for Square {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 64 { Ok(Self(value)) } else { Err(()) }
    }
}
