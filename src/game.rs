use std::fmt;

use crate::{Color, Figure, Position, Tile};

pub struct Field([[Tile; 8]; 8]);

impl Field {
    fn new() -> Self {
        let mut result = Self {
            0: Default::default()
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

        result.set_figure_line(0, Color::White, major_figure_line);
        result.set_figure_line(1, Color::White, pawn_line);

        result.set_figure_line(7, Color::Black, major_figure_line);
        result.set_figure_line(6, Color::Black, pawn_line);

        result
    }

    fn set_figure_line(&mut self, line: usize, color: Color, figures: [Figure; 8]) {
        self.0[line]
            .iter_mut()
            .zip(figures.iter())
            .for_each(|(tile, &figure)| *tile = Tile::Occupied(color, figure));
    }

    fn iter(&self) -> std::slice::Iter<'_, [Tile; 8]>{
        self.0.iter()
    }

    pub fn get(&self, position: Position) -> Tile {
        self.0[position.rank as usize][position.file as usize]
    }
}

pub struct Game {
    pub field: Field,
}

impl Game {
    pub fn new() -> Self {
        Game {
            field: Field::new(),
        }
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
    use crate::{File, Rank};

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

    #[test]
    fn test_black_queen_initial_position() {
        let game = Game::new();
        let queen_tile = game.field.get(Position {
            rank: Rank::Eight,
            file: File::D,
        });
        assert_eq!(queen_tile, Tile::Occupied(Color::Black, Figure::Queen))
    }

    #[test]
    fn test_white_rooks_initial_position() {
        let game = Game::new();
        let first_rook_tile = game.field.get(Position {
            rank: Rank::One,
            file: File::A,
        });
        let second_rook_tile = game.field.get(Position {
            rank: Rank::One,
            file: File::H,
        });
        let white_rook = Tile::Occupied(Color::White, Figure::Rook);
        assert_eq!(first_rook_tile, white_rook);
        assert_eq!(second_rook_tile, white_rook);
    }
}
