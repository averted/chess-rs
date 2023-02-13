mod board;
mod core;
mod piece;
mod position;
mod renderer;

use crate::board::Board;
use crate::position::Position;
use std::error::Error;
use std::fs::File;
use std::io;

//fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    let mut board = Board::new();

    board.generate_pieces();
    board.render();

    loop {
        println!("Enter move ({:?}):", board.turn);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");

        let input: String = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        let positions: Vec<&str> = if input.len() > 0 {
            input.split('-').collect()
        } else {
            vec![]
        };

        if let Some(from) = core::get_position_from_string(positions.get(0)) {
            match core::get_position_from_string(positions.get(1)) {
                Some(to) => {
                    board.move_piece(from, to);
                }
                None => {
                    // todo
                    board.move_piece(Position::new('a', 2), from);
                }
            }
        }
    }
}
