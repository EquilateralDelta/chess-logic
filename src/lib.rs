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

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
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

pub struct Game {
    pub field: [[Tile; 8]; 8],
}

impl Game {
    pub fn new() -> Self {
        let mut field: Self = Self {
            field: Default::default(),
        };
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

        field.set_figure_line(0, Color::White, major_figure_line);
        field.set_figure_line(1, Color::White, pawn_line);

        field.set_figure_line(7, Color::Black, major_figure_line);
        field.set_figure_line(6, Color::Black, pawn_line);

        field
    }

    fn set_figure_line(&mut self, line: usize, color: Color, figures: [Figure; 8]) {
        self.field[line]
            .iter_mut()
            .zip(figures.iter())
            .for_each(|(tile, &figure)| *tile = Tile::Occupied(color, figure));
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line_index, line) in self.field.iter().rev().enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_newly_created_field() {
        let game = Game::new();
        let displayed = format!("{}", game);
        let displayed_lines: Vec<_> = displayed.split('\n').collect();

        assert_eq!(
            displayed_lines,
            vec![
                " b♖  b♘  b♗  b♕  b♔  b♗  b♘  b♖ ",
                " b♙  b♙  b♙  b♙  b♙  b♙  b♙  b♙ ",
                " __  __  __  __  __  __  __  __ ",
                " __  __  __  __  __  __  __  __ ",
                " __  __  __  __  __  __  __  __ ",
                " __  __  __  __  __  __  __  __ ",
                " w♙  w♙  w♙  w♙  w♙  w♙  w♙  w♙ ",
                " w♖  w♘  w♗  w♕  w♔  w♗  w♘  w♖ ",
            ]
        );
    }
}
