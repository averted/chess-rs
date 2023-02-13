use crate::core::Color;
use crate::piece::Piece;
use crate::position::Position;

pub struct Knight {
    color: Color,
    position: Position,
}

impl Knight {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Knight {
    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn to_string(&self) -> &str {
        "N"
    }

    fn move_to(&mut self, to: &Position) -> Result<(), String> {
        self.position = Position::from(to);
        Ok(())
    }
}
