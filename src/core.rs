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
    if let Some(value) = s {
        let file: char = value.chars().nth(0).unwrap();
        let rank: u8 = value.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

        return Some(Position::new(file, rank));
    }

    println!("No value");
    None
}

pub fn get_adjacent_files(p: &Position) -> [char; 2] {
    let arr: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for (idx, _) in arr.iter().enumerate() {
        match p.file {
            'a' => return ['x', 'b'],
            'h' => return ['g', 'x'],
            _ => return [arr[idx - 1], arr[idx + 1]],
        }
    }

    ['x', 'x']
}
