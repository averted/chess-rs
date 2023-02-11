use crate::position::Position;

#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
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

pub fn get_position_from_string(s: Option<&&str>) -> Option<Position> {
    match s {
        Some(value) => {
            println!("VAL: {}", value);
            let file: char = value.chars().nth(0).unwrap();
            let rank: u8 = value.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
            Some(Position::new(file, rank))
        }
        None => {
            println!("No value");
            None
        }
    }
}
