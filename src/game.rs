use std::fmt;

use crate::{Color, Figure, Position, Rank, Tile};

pub struct Field([[Tile; 8]; 8]);

#[derive(PartialEq, Eq, Clone, Copy)]
enum MoveType {
    MoveAndAttack { opposite: Color },
    OnlyMove,
    OnlyAttack { opposite: Color },
}

impl Field {
    fn new() -> Self {
        let mut result = Self {
            0: Default::default(),
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

    fn iter(&self) -> std::slice::Iter<'_, [Tile; 8]> {
        self.0.iter()
    }

    pub fn get(&self, position: Position) -> Tile {
        self.0[position.rank as usize - 1][position.file as usize - 1]
    }

    fn set(&mut self, position: Position, tile: Tile) {
        self.0[position.rank as usize - 1][position.file as usize - 1] = tile;
    }

    pub fn moves_available(&self, position: Position, turn: Color) -> Vec<Position> {
        let tile = self.get(position);
        let mut result = vec![];
        match tile {
            Tile::Occupied(color, figure) if color == turn => match figure {
                Figure::Pawn => {
                    let can_make_long_move = (position.rank == Rank::Two && color == Color::White)
                        || (position.rank == Rank::Seven && color == Color::Black);
                    let move_distance = if can_make_long_move { 2 } else { 1 };
                    let move_vector = if color == Color::White {
                        (1, 0)
                    } else {
                        (-1, 0)
                    };
                    self.fill_moves_by_direction(
                        position,
                        move_vector,
                        move_distance,
                        MoveType::OnlyMove,
                        &mut result,
                    );
                    for attack_vector in [(move_vector.0, -1), (move_vector.0, 1)] {
                        self.fill_moves_by_direction(
                            position,
                            attack_vector,
                            1,
                            MoveType::OnlyAttack {
                                opposite: color.opposite(),
                            },
                            &mut result,
                        );
                    }
                }
                Figure::Bishop => {
                    for direction in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        self.fill_moves_by_direction(
                            position,
                            direction,
                            8,
                            MoveType::MoveAndAttack {
                                opposite: color.opposite(),
                            },
                            &mut result,
                        );
                    }
                }
                Figure::Knight => {
                    for direction in [
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ] {
                        self.fill_moves_by_direction(
                            position,
                            direction,
                            1,
                            MoveType::MoveAndAttack {
                                opposite: color.opposite(),
                            },
                            &mut result,
                        );
                    }
                }
                Figure::Rook => {
                    for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        self.fill_moves_by_direction(
                            position,
                            direction,
                            8,
                            MoveType::MoveAndAttack {
                                opposite: color.opposite(),
                            },
                            &mut result,
                        );
                    }
                }
                Figure::Queen => {
                    for direction in [
                        (1, 0),
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                    ] {
                        self.fill_moves_by_direction(
                            position,
                            direction,
                            8,
                            MoveType::MoveAndAttack {
                                opposite: color.opposite(),
                            },
                            &mut result,
                        );
                    }
                }
                Figure::King => {
                    for direction in [
                        (1, 0),
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                    ] {
                        self.fill_moves_by_direction(
                            position,
                            direction,
                            1,
                            MoveType::MoveAndAttack {
                                opposite: color.opposite(),
                            },
                            &mut result,
                        );
                    }
                }
            },
            _ => (),
        }

        result.sort();
        result
    }

    pub fn make_move(&mut self, from: Position, to: Position) {
        let from_tile = self.get(from);

        self.set(to, from_tile);
        self.set(from, Tile::Empty);
    }

    fn fill_moves_by_direction(
        &self,
        from: Position,
        direction: (i8, i8),
        distance: i8,
        move_type: MoveType,
        storage: &mut Vec<Position>,
    ) {
        for i in 1..=distance {
            let difference = (direction.0 * i, direction.1 * i);
            let position = match from.add(difference) {
                Some(position) => position,
                None => return,
            };

            #[derive(PartialEq)]
            enum MatchResult {
                CanMove,
                CannotMove,
                CanMoveNotFurther,
            }

            let can_move = match (self.get(position), move_type) {
                (Tile::Empty, MoveType::OnlyAttack { opposite: _ }) => MatchResult::CannotMove,
                (Tile::Empty, _) => MatchResult::CanMove,
                (
                    Tile::Occupied(color, _),
                    MoveType::MoveAndAttack {
                        opposite: move_color,
                    }
                    | MoveType::OnlyAttack {
                        opposite: move_color,
                    },
                ) if color == move_color => MatchResult::CanMoveNotFurther,
                _ => MatchResult::CannotMove,
            };
            if can_move != MatchResult::CannotMove {
                storage.push(position);
            }
            if can_move != MatchResult::CanMove {
                return;
            }
        }
    }
}

pub struct Game {
    pub field: Field,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Game {
            field: Field::new(),
            turn: Color::White,
        }
    }

    pub fn moves_available(&self, position: Position) -> Vec<Position> {
        self.field.moves_available(position, self.turn)
    }

    pub fn make_move(&mut self, from: Position, to: Position) -> Result<(), ()> {
        let moves = self.field.moves_available(from, self.turn);
        if !moves.contains(&to) {
            return Err(());
        }
        self.field.make_move(from, to);
        self.turn = self.turn.opposite();
        Ok(())
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
    use std::convert::TryInto;

    use super::*;

    macro_rules! pos {
        ($p:ident) => {
            TryInto::<Position>::try_into(stringify!($p)).unwrap()
        };
    }

    macro_rules! pos_list {
        ($($p:ident),+) => {
            {
                let mut pos_list = vec![$(pos!($p)),+];
                pos_list.sort();
                pos_list
            }
        };
    }

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
        let queen_tile = game.field.get(pos!(d8));
        assert_eq!(queen_tile, Tile::Occupied(Color::Black, Figure::Queen))
    }

    #[test]
    fn test_white_rooks_initial_position() {
        let game = Game::new();
        let first_rook_tile = game.field.get(pos!(a1));
        let second_rook_tile = game.field.get(pos!(h1));
        let white_rook = Tile::Occupied(Color::White, Figure::Rook);
        assert_eq!(first_rook_tile, white_rook);
        assert_eq!(second_rook_tile, white_rook);
    }

    #[test]
    fn test_white_pawn_first_move() {
        let game = Game::new();
        let moves = game.moves_available(pos!(e2));
        assert_eq!(moves, vec![pos!(e3), pos!(e4)]);
    }

    #[test]
    fn test_white_pawn_second_move_or_capture() {
        let mut game = Game::new();
        game.make_move(pos!(e2), pos!(e4)).unwrap();
        game.make_move(pos!(d7), pos!(d5)).unwrap();
        let moves = game.moves_available(pos!(e4));
        assert_eq!(moves, pos_list!(e5, d5));
    }

    #[test]
    fn test_knight_moves() {
        let mut game = Game::new();
        game.make_move(pos!(b1), pos!(c3)).unwrap();
        game.make_move(pos!(d7), pos!(d5)).unwrap();
        let moves = game.moves_available(pos!(c3));
        assert_eq!(moves, pos_list!(b1, a4, b5, d5, e4));
    }

    #[test]
    fn test_bishop_moves() {
        let mut game = Game::new();
        game.make_move(pos!(e2), pos!(e3)).unwrap();
        game.make_move(pos!(f7), pos!(f5)).unwrap();
        game.make_move(pos!(f1), pos!(d3)).unwrap();
        game.make_move(pos!(e7), pos!(e6)).unwrap();
        let moves = game.moves_available(pos!(d3));
        assert_eq!(moves, pos_list!(a6, b5, c4, e2, f1, e4, f5));
    }
}
