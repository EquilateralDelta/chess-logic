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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}