use crate::core::Color;
use crate::piece::Piece;
use crate::position::Position;

pub struct Queen {
    color: Color,
    position: Position,
}

impl Queen {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Queen {
    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn to_string(&self) -> &str {
        "Q"
    }

    fn move_to(&mut self, to: &Position) -> Result<(), String> {
        self.position = Position::from(to);
        Ok(())
    }
}
