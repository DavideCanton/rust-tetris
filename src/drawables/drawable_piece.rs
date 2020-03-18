use crate::{
    drawables::{drawable_obj::DrawableObject, square::Square},
    pieces::TetrisPieceStruct,
    utils::{piece_to_color, WIDTH},
};
use graphics::{math::Vec2d, types::Scalar, Context};
use opengl_graphics::GlGraphics;

pub struct DrawablePiece<'a> {
    pos: Vec2d,
    piece: &'a TetrisPieceStruct,
    is_shadow: bool
}

impl<'a> DrawablePiece<'a> {
    pub fn new(pos: Vec2d, piece: &'a TetrisPieceStruct, is_shadow: bool) -> Self {
        DrawablePiece {
            pos,
            piece,
            is_shadow
        }
    }
}

impl DrawableObject for DrawablePiece<'_> {
    fn position(&self) -> Vec2d {
        self.pos
    }

    fn draw_object_after_traslate(&self, gl: &mut GlGraphics, ctx: Context) {
        let piece_board = &self.piece.board;

        for i in 0..piece_board.rows {
            for j in 0..piece_board.cols {
                if let Some(ref p) = piece_board.get(i, j) {
                    let i = i as Scalar;
                    let j = j as Scalar;

                    let pos = [j * WIDTH, i * WIDTH];
                    let color = piece_to_color(*p, self.is_shadow);

                    Square::new(pos, WIDTH, color).draw_object(gl, ctx);
                }
            }
        }
    }
}
