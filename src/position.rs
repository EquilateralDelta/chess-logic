use std::convert::{TryInto};
use std::mem::transmute;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

impl Rank {
    pub fn iter() -> std::slice::Iter<'static, Rank> {
        use Rank::*;
        static RANKS: [Rank; 8] = [One, Two, Three, Four, Five, Six, Seven, Eight];
        RANKS.iter()
    }
}

impl TryInto<Rank> for i8 {
    type Error = ();

    fn try_into(self) -> Result<Rank, Self::Error> {
        if self >= 0 && self <= 7 {
            let rank: Rank = unsafe { transmute(self) };
            Ok(rank)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub fn iter() -> std::slice::Iter<'static, File> {
        use File::*;
        static FILES: [File; 8] = [A, B, C, D, E, F, G, H];
        FILES.iter()
    }
}

impl TryInto<File> for i8 {
    type Error = ();

    fn try_into(self) -> Result<File, Self::Error> {
        if self >= 0 && self <= 7 {
            let file: File = unsafe { transmute(self) };
            Ok(file)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

impl Position {
    pub fn add(&self, vector: (i8, i8)) -> Option<Position> {
        let (rank_change, file_change) = vector;
        let rank: Option<Rank> = ((self.rank as i8) + rank_change).try_into().ok();
        let file: Option<File> = ((self.file as i8) + file_change).try_into().ok();
        if let (Some(rank), Some(file)) = (rank, file) {
            Some(Position { rank, file })
        } else {
            None
        }
    }
}
