use crate::piece::Piece;
use crate::renderer::Renderer;

pub struct Board<'a> {
    pieces: Vec<&'a Piece>,
    renderer: Renderer,
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            pieces: vec![],
        }
    }

    pub fn add_piece(&mut self, piece: &'a Piece) {
        self.pieces.push(&piece);
    }

    pub fn render(&self) -> String {
        self.renderer.render(&self.pieces)
    }
}
