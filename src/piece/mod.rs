use crate::core::Color;
use crate::position::Position;

pub enum Kind {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
    Pawn,
}

pub struct Piece {
    kind: Kind,
    pub color: Color,
    position: Position,
}

impl Piece {
    pub fn new(color: Color, kind: Kind, position: Position) -> Piece {
        Piece {
            kind,
            position,
            color,
        }
    }

    pub fn to_string(&self) -> &str {
        match &self.kind {
            Kind::Rook => "R",
            Kind::Bishop => "B",
            Kind::Knight => "N",
            Kind::Queen => "Q",
            Kind::King => "K",
            Kind::Pawn => "p",
        }
    }

    pub fn at(&self, position: &Position) -> bool {
        self.position == *position
    }

    pub fn move_to(&mut self, position: Position) {
        self.position = Position::from(position);
    }

    // TODO
    pub fn can_move_to(&self, position: &Position) -> bool {
        true
    }

    pub fn is_black(&self) -> bool {
        self.color == Color::Black
    }
}
