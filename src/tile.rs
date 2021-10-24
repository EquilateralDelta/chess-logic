use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Figure {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = match self {
            Figure::Pawn => "♙",
            Figure::Knight => "♘",
            Figure::Bishop => "♗",
            Figure::Rook => "♖",
            Figure::Queen => "♕",
            Figure::King => "♔",
        };
        write!(f, "{}", character)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = match self {
            Color::White => "w",
            Color::Black => "b",
        };
        write!(f, "{}", character)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Empty,
    Occupied(Color, Figure),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied(color, figure) => write!(f, "{}{}", color, figure),
            Tile::Empty => write!(f, "__"),
        }
    }
}
