use crate::core::{Color, Position};

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
    color: Color,
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

    pub fn at(&self, file: char, rank: u8) -> bool {
        self.position.file == file && self.position.rank == rank
    }

    pub fn is_black(&self) -> bool {
        self.color == Color::Black
    }
}
