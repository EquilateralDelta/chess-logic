use std::convert::{TryFrom, TryInto};
use std::mem::transmute;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Rank {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

impl Rank {
    pub fn iter() -> std::slice::Iter<'static, Rank> {
        use Rank::*;
        static RANKS: [Rank; 8] = [One, Two, Three, Four, Five, Six, Seven, Eight];
        RANKS.iter()
    }
}

impl TryInto<Rank> for u8 {
    type Error = ();

    fn try_into(self) -> Result<Rank, Self::Error> {
        if self > 0 && self <= 8 {
            let rank: Rank = unsafe { transmute(self) };
            Ok(rank)
        } else {
            Err(())
        }
    }
}

impl TryInto<Rank> for char {
    type Error = ();

    fn try_into(self) -> Result<Rank, Self::Error> {
        match self {
            '1' => Ok(Rank::One),
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum File {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
}

impl File {
    pub fn iter() -> std::slice::Iter<'static, File> {
        use File::*;
        static FILES: [File; 8] = [A, B, C, D, E, F, G, H];
        FILES.iter()
    }
}

impl TryInto<File> for u8 {
    type Error = ();

    fn try_into(self) -> Result<File, Self::Error> {
        if self > 0 && self <= 8 {
            let file: File = unsafe { transmute(self) };
            Ok(file)
        } else {
            Err(())
        }
    }
}

impl TryInto<File> for char {
    type Error = ();

    fn try_into(self) -> Result<File, Self::Error> {
        match self {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

impl Position {
    pub fn add(&self, vector: (i8, i8)) -> Option<Position> {
        let (rank_change, file_change) = vector;
        let rank: Option<Rank> = {
            let changed = (self.rank as i8) + rank_change;
            (changed as u8).try_into().ok()
        };
        let file: Option<File> = {
            let changed = (self.file as i8) + file_change;
            (changed as u8).try_into().ok()
        };
        if let (Some(rank), Some(file)) = (rank, file) {
            Some(Position { rank, file })
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Position {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }

        let mut chars = value.chars().into_iter();
        let file = chars.next().ok_or(())?.try_into()?;
        let rank = chars.next().ok_or(())?.try_into()?;
        Ok(Position { rank, file })
    }
}
