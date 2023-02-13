pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::core::Color;
use crate::position::Position;

pub trait Piece {
    fn color(&self) -> &Color;
    fn position(&self) -> &Position;
    fn to_string(&self) -> &str;
    fn move_to(&mut self, to: &Position) -> Result<(), String>;

    fn at(&self, position: &Position) -> bool {
        self.position() == position
    }
}
