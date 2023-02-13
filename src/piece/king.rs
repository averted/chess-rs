use crate::core::Color;
use crate::piece::Piece;
use crate::position::Position;

pub struct King {
    color: Color,
    position: Position,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for King {
    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn to_string(&self) -> &str {
        "K"
    }

    fn move_to(&mut self, to: &Position) -> Result<(), String> {
        self.position = Position::from(to);
        Ok(())
    }
}
