use crate::core;
use crate::piece::Piece;
use crate::position::Position;

pub struct Pawn {
    color: core::Color,
    position: Position,
}

impl Pawn {
    pub fn new(color: core::Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Pawn {
    fn validate(&self, position: &Position, pieces: &Vec<Box<dyn Piece>>) -> Result<(), String> {
        if self.can_move_to(position, pieces) {
            return Ok(());
        }

        Err(String::from("TESTING"))
    }

    // TODO
    fn can_move_to(&self, position: &Position, pieces: &Vec<Box<dyn Piece>>) -> bool {
        /*
        // if there is already a piece at that position -> return false

        // TODO: add move DIAGONAL logic (capturing)
        if self.position.file != position.file {
            if let Some(piece) = to_piece {
                // cannot capture your own pieces
                if piece.color() == self.color() {
                    return false;
                }

                // cannot capture on more than 1 file diff
                if core::get_adjacent_files(&self.position)
                    .iter()
                    .any(|&i| i == position.file)
                {
                    println!("HERE!!!");
                }

                // adjacent file and - black -1 rank; white +1 rank
            }

            return false;
        }

        match self.color {
            core::Color::White => {
                let rank_diff = position.rank - self.position.rank;

                if position.rank <= self.position.rank {
                    return false;
                }

                if rank_diff == 1 {}

                if rank_diff == 2 {}
            }
            core::Color::Black => {
                if position.rank >= self.position.rank {
                    return false;
                }
            }
        }
        */

        true
    }
}

impl Piece for Pawn {
    fn color(&self) -> &core::Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn to_string(&self) -> &str {
        "p"
    }

    fn move_to(&mut self, to: &Position) -> Result<(), String> {
        // self.validate(&position, &pieces)?;

        if true {
            self.position = Position::from(to);
            Ok(())
        } else {
            Err(String::from("Testing"))
        }
    }
}
