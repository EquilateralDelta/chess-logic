use std::fmt;

#[derive(Clone, Copy)]
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

pub enum Tile {
    Empty,
    White(Figure),
    Black(Figure),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::White(figure) => write!(f, "w{}", figure),
            Self::Black(figure) => write!(f, "b{}", figure),
            Tile::Empty => write!(f, "__"),
        }
    }
}

#[derive(Default)]
pub struct Game {
    tiles: [[Tile; 8]; 8],
}

impl Game {
    pub fn new() -> Self {
        let mut field: Self = Default::default();
        let major_figure_line = [
            Figure::Rook,
            Figure::Knight,
            Figure::Bishop,
            Figure::Queen,
            Figure::King,
            Figure::Bishop,
            Figure::Knight,
            Figure::Rook,
        ];
        let pawn_line = [Figure::Pawn; 8];

        field.set_figure_line(0, Tile::White, major_figure_line);
        field.set_figure_line(1, Tile::White, pawn_line);

        field.set_figure_line(7, Tile::Black, major_figure_line);
        field.set_figure_line(6, Tile::Black, pawn_line);

        field
    }

    fn set_figure_line(&mut self, line: usize, color: fn(Figure) -> Tile, figures: [Figure; 8]) {
        self.tiles[line]
            .iter_mut()
            .zip(figures.iter())
            .for_each(|(tile, &figure)| *tile = color(figure));
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line_index, line) in self.tiles.iter().rev().enumerate() {
            if line_index > 0 {
                writeln!(f, "")?;
            }
            for tile in line.iter() {
                write!(f, " {} ", tile)?;
            }
        }
        Ok(().into())
    }
}

fn main() {
    let game = Game::new();
    println!("{}", game);
}
