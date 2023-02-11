use crate::core;
use crate::piece::Piece;
use crate::position::Position;
use colored::*;

pub struct Renderer {
    matrix: [[u8; 8]; 8],
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            matrix: [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
        }
    }

    pub fn render(&self, pieces: &Vec<Piece>) -> String {
        let border = String::from("--------------------------");
        let mut result = String::from(&border);

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let mut top = "|".to_string();
            let mut mid = "|".to_string();
            let mut bot = "|".to_string();

            for (col_idx, _) in row.iter().enumerate() {
                let pos = Position::new(
                    core::get_file_from_index(col_idx),
                    core::get_rank_from_index(row_idx),
                );
                let piece = pieces.iter().find(|x| x.at(&pos));
                let (t, m, b) = &self.render_cell(piece, row_idx, col_idx);

                top = format!("{}{}", top, t);
                mid = format!("{}{}", mid, m);
                bot = format!("{}{}", bot, b);
            }

            result = format!("{}\n{}|\n{}|\n{}|", result, top, mid, bot)
        }

        format!("{}\n{}", result, border)
    }

    fn render_cell(
        &self,
        piece: Option<&Piece>,
        row: usize,
        col: usize,
    ) -> (String, String, String) {
        // ░  ·
        let spacer = if (row + col) % 2 == 0 { "·" } else { " " };
        let buffer = format!("{}{}{}", spacer, spacer, spacer);

        (
            String::from(&buffer),
            match piece {
                Some(piece) => {
                    format!(
                        "{}{}{}",
                        spacer,
                        if piece.is_black() {
                            piece.to_string().red()
                        } else {
                            piece.to_string().green()
                        },
                        spacer
                    )
                }
                None => String::from(&buffer),
            },
            String::from(&buffer),
        )
    }
}
