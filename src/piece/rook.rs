use crate::core::Color;
use crate::position::Position;

pub struct Rook {
    pub color: Color,
    position: Position,
}

impl Rook {
    pub fn new(color: Color, position: Position) -> Self {
        Self { position, color }
    }
}
