use crate::core::Color;
use crate::piece::{Kind, Piece};
use crate::position::Position;
use crate::renderer::Renderer;

pub struct Board {
    pub turn: Color,
    pieces: Vec<Piece>,
    renderer: Renderer,
}

impl Board {
    pub fn new() -> Self {
        Self {
            turn: Color::White,
            renderer: Renderer::new(),
            pieces: vec![],
        }
    }

    pub fn generate_pieces(&mut self) {
        self.pieces
            .push(Piece::new(Color::White, Kind::Rook, Position::new('a', 1)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Rook, Position::new('h', 1)));
        self.pieces.push(Piece::new(
            Color::White,
            Kind::Knight,
            Position::new('b', 1),
        ));
        self.pieces.push(Piece::new(
            Color::White,
            Kind::Knight,
            Position::new('g', 1),
        ));
        self.pieces.push(Piece::new(
            Color::White,
            Kind::Bishop,
            Position::new('c', 1),
        ));
        self.pieces.push(Piece::new(
            Color::White,
            Kind::Bishop,
            Position::new('f', 1),
        ));
        self.pieces
            .push(Piece::new(Color::White, Kind::Queen, Position::new('d', 1)));
        self.pieces
            .push(Piece::new(Color::White, Kind::King, Position::new('e', 1)));

        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('a', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('b', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('c', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('d', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('e', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('f', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('g', 2)));
        self.pieces
            .push(Piece::new(Color::White, Kind::Pawn, Position::new('h', 2)));

        self.pieces
            .push(Piece::new(Color::Black, Kind::Rook, Position::new('a', 8)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Rook, Position::new('h', 8)));
        self.pieces.push(Piece::new(
            Color::Black,
            Kind::Knight,
            Position::new('b', 8),
        ));
        self.pieces.push(Piece::new(
            Color::Black,
            Kind::Knight,
            Position::new('g', 8),
        ));
        self.pieces.push(Piece::new(
            Color::Black,
            Kind::Bishop,
            Position::new('c', 8),
        ));
        self.pieces.push(Piece::new(
            Color::Black,
            Kind::Bishop,
            Position::new('f', 8),
        ));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Queen, Position::new('d', 8)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::King, Position::new('e', 8)));

        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('a', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('b', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('c', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('d', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('e', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('f', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('g', 7)));
        self.pieces
            .push(Piece::new(Color::Black, Kind::Pawn, Position::new('h', 7)));
    }

    pub fn render(&self) -> () {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", self.renderer.render(&self.pieces));
    }

    pub fn move_piece(&mut self, from: Position, to: Position) {
        let result: Option<&mut Piece> = self.pieces.iter_mut().find(|x| x.at(&from));

        match result {
            Some(piece) => {
                if piece.color == self.turn && piece.can_move_to(&to) {
                    piece.move_to(to);
                    self.render();
                    self.flip_turn();
                }
            }
            None => {
                println!("No piece found at: {}", from);
            }
        }
    }

    fn flip_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }
}
