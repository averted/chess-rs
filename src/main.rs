mod board;
mod core;
mod piece;
mod renderer;

use crate::board::Board;
use crate::core::{Color, Position};
use crate::piece::{Kind, Piece};

fn main() {
    let mut board = Board::new();
    let w_rook1 = Piece::new(Color::White, Kind::Rook, Position::new('a', 1));
    let w_rook2 = Piece::new(Color::White, Kind::Rook, Position::new('h', 1));
    let w_knight1 = Piece::new(Color::White, Kind::Knight, Position::new('b', 1));
    let w_knight2 = Piece::new(Color::White, Kind::Knight, Position::new('g', 1));
    let w_bishop1 = Piece::new(Color::White, Kind::Bishop, Position::new('c', 1));
    let w_bishop2 = Piece::new(Color::White, Kind::Bishop, Position::new('f', 1));
    let w_queen = Piece::new(Color::White, Kind::Queen, Position::new('d', 1));
    let w_king = Piece::new(Color::White, Kind::King, Position::new('e', 1));

    let w_pawn1 = Piece::new(Color::White, Kind::Pawn, Position::new('a', 2));
    let w_pawn2 = Piece::new(Color::White, Kind::Pawn, Position::new('b', 2));
    let w_pawn3 = Piece::new(Color::White, Kind::Pawn, Position::new('c', 2));
    let w_pawn4 = Piece::new(Color::White, Kind::Pawn, Position::new('d', 2));
    let w_pawn5 = Piece::new(Color::White, Kind::Pawn, Position::new('e', 2));
    let w_pawn6 = Piece::new(Color::White, Kind::Pawn, Position::new('f', 2));
    let w_pawn7 = Piece::new(Color::White, Kind::Pawn, Position::new('g', 2));
    let w_pawn8 = Piece::new(Color::White, Kind::Pawn, Position::new('h', 2));

    let b_rook1 = Piece::new(Color::Black, Kind::Rook, Position::new('a', 8));
    let b_rook2 = Piece::new(Color::Black, Kind::Rook, Position::new('h', 8));
    let b_knight1 = Piece::new(Color::Black, Kind::Knight, Position::new('b', 8));
    let b_knight2 = Piece::new(Color::Black, Kind::Knight, Position::new('g', 8));
    let b_bishop1 = Piece::new(Color::Black, Kind::Bishop, Position::new('c', 8));
    let b_bishop2 = Piece::new(Color::Black, Kind::Bishop, Position::new('f', 8));
    let b_queen = Piece::new(Color::Black, Kind::Queen, Position::new('d', 8));
    let b_king = Piece::new(Color::Black, Kind::King, Position::new('e', 8));

    let b_pawn1 = Piece::new(Color::Black, Kind::Pawn, Position::new('a', 7));
    let b_pawn2 = Piece::new(Color::Black, Kind::Pawn, Position::new('b', 7));
    let b_pawn3 = Piece::new(Color::Black, Kind::Pawn, Position::new('c', 7));
    let b_pawn4 = Piece::new(Color::Black, Kind::Pawn, Position::new('d', 7));
    let b_pawn5 = Piece::new(Color::Black, Kind::Pawn, Position::new('e', 7));
    let b_pawn6 = Piece::new(Color::Black, Kind::Pawn, Position::new('f', 7));
    let b_pawn7 = Piece::new(Color::Black, Kind::Pawn, Position::new('g', 7));
    let b_pawn8 = Piece::new(Color::Black, Kind::Pawn, Position::new('h', 7));

    board.add_piece(&w_rook1);
    board.add_piece(&w_rook2);
    board.add_piece(&w_knight1);
    board.add_piece(&w_knight2);
    board.add_piece(&w_bishop1);
    board.add_piece(&w_bishop2);
    board.add_piece(&w_queen);
    board.add_piece(&w_king);

    board.add_piece(&w_pawn1);
    board.add_piece(&w_pawn2);
    board.add_piece(&w_pawn3);
    board.add_piece(&w_pawn4);
    board.add_piece(&w_pawn5);
    board.add_piece(&w_pawn6);
    board.add_piece(&w_pawn7);
    board.add_piece(&w_pawn8);

    board.add_piece(&b_rook1);
    board.add_piece(&b_rook2);
    board.add_piece(&b_knight1);
    board.add_piece(&b_knight2);
    board.add_piece(&b_bishop1);
    board.add_piece(&b_bishop2);
    board.add_piece(&b_queen);
    board.add_piece(&b_king);

    board.add_piece(&b_pawn1);
    board.add_piece(&b_pawn2);
    board.add_piece(&b_pawn3);
    board.add_piece(&b_pawn4);
    board.add_piece(&b_pawn5);
    board.add_piece(&b_pawn6);
    board.add_piece(&b_pawn7);
    board.add_piece(&b_pawn8);

    println!("{}", board.render());
}
