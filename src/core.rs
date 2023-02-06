#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

pub struct Position {
    pub file: char,
    pub rank: u8,
}

impl Position {
    pub fn new(file: char, rank: u8) -> Position {
        Position { file, rank }
    }
}

pub fn get_rank_from_index(row_idx: usize) -> u8 {
    (8 - row_idx) as u8
}

pub fn get_file_from_index(col_idx: usize) -> char {
    match col_idx {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => 'x',
    }
}
