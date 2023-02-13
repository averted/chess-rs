use crate::core::Color;
use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;
use crate::piece::Piece;
use crate::position::Position;
use crate::renderer::Renderer;

pub struct Board {
    pub turn: Color,
    pieces: Vec<Box<dyn Piece>>,
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
            .push(Box::new(Rook::new(Color::White, Position::new('a', 1))));

        self.pieces
            .push(Box::new(Rook::new(Color::White, Position::new('h', 1))));
        self.pieces
            .push(Box::new(Knight::new(Color::White, Position::new('b', 1))));

        self.pieces
            .push(Box::new(Knight::new(Color::White, Position::new('g', 1))));

        self.pieces
            .push(Box::new(Bishop::new(Color::White, Position::new('c', 1))));
        self.pieces
            .push(Box::new(Bishop::new(Color::White, Position::new('f', 1))));
        self.pieces
            .push(Box::new(Queen::new(Color::White, Position::new('d', 1))));
        self.pieces
            .push(Box::new(King::new(Color::White, Position::new('e', 1))));

        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('a', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('b', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('c', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('d', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('e', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('f', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('g', 2))));
        self.pieces
            .push(Box::new(Pawn::new(Color::White, Position::new('h', 2))));

        self.pieces
            .push(Box::new(Rook::new(Color::Black, Position::new('a', 8))));
        self.pieces
            .push(Box::new(Rook::new(Color::Black, Position::new('h', 8))));
        self.pieces
            .push(Box::new(Knight::new(Color::Black, Position::new('b', 8))));
        self.pieces
            .push(Box::new(Knight::new(Color::Black, Position::new('g', 8))));
        self.pieces
            .push(Box::new(Bishop::new(Color::Black, Position::new('c', 8))));
        self.pieces
            .push(Box::new(Bishop::new(Color::Black, Position::new('f', 8))));
        self.pieces
            .push(Box::new(Queen::new(Color::Black, Position::new('d', 8))));
        self.pieces
            .push(Box::new(King::new(Color::Black, Position::new('e', 8))));

        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('a', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('b', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('c', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('d', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('e', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('f', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('g', 7))));
        self.pieces
            .push(Box::new(Pawn::new(Color::Black, Position::new('h', 7))));
    }

    pub fn render(&self) -> () {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", self.renderer.render(&self.pieces));
    }

    pub fn move_piece(&mut self, from: Position, to: Position) {
        if let Some(from_idx) = self.pieces.iter().position(|x| x.at(&from)) {
            if let Some(piece) = self.pieces.get_mut(from_idx) {
                if *piece.color() != self.turn {
                    println!("Cannot move enemy pieces");
                    return ();
                }

                // let to_piece: Option<&Box<dyn Piece>> = self.pieces.iter().find(|x| x.at(&to));

                match piece.move_to(&to) {
                    Ok(_) => {
                        self.render();
                        self.flip_turn();
                    }
                    Err(error) => {
                        println!("Error moving piece: {:?}", error);
                    }
                }
            }
        } else {
            println!("No piece found at position {}", from);
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
