use crate::core::Color;
use crate::piece::Piece;
use crate::position::Position;

pub struct Rook {
    color: Color,
    position: Position,
}

impl Rook {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Rook {
    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn to_string(&self) -> &str {
        "R"
    }

    fn move_to(&mut self, to: &Position) -> Result<(), String> {
        self.position = Position::from(to);
        Ok(())
    }
}
