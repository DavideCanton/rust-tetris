use crate::{
    drawables::{
        border::Border, drawable_obj::DrawableObject, pause_overlay::PauseOverlay, square::Square,
    },
    pieces::TetrisPiece,
    utils::*,
};
use graphics::{types::Scalar, Context};
use opengl_graphics::GlGraphics;

pub struct Drawer<'a> {
    gl: &'a mut GlGraphics,
    ctx: Context,
}

impl<'a> Drawer<'a> {
    pub fn new(gl: &'a mut GlGraphics, ctx: Context) -> Self {
        Drawer { gl, ctx }
    }

    pub fn draw_piece_block(&mut self, i: isize, j: isize, piece: TetrisPiece, is_shadow: bool) {
        let i = i as Scalar;
        let j = j as Scalar;

        let pos = [BASE_X as Scalar + j * WIDTH, i * WIDTH];
        let color = piece_to_color(&piece, is_shadow);
        let sq = Square::new(pos, WIDTH, color);
        sq.draw_object(self.gl, self.ctx);
    }

    pub fn draw_next_block(
        &mut self,
        i: isize,
        j: isize,
        piece: TetrisPiece,
        base_x: Scalar,
        base_y: Scalar,
    ) {
        let i = i as Scalar;
        let j = j as Scalar;

        let pos = [BASE_X as Scalar + j * WIDTH + base_x, i * WIDTH + base_y];
        let sq = Square::new(pos, WIDTH, piece_to_color(&piece, false));
        sq.draw_object(self.gl, self.ctx);
    }

    pub fn draw_border(&mut self) {
        let b = Border::new(
            [BASE_X as Scalar - 1.0, 0.0],
            YELLOW,
            1.0,
            WIDTH * 10.0,
            600.0,
        );
        b.draw_object(self.gl, self.ctx);
    }

    pub fn clear(&mut self) {
        graphics::clear(BGCOLOR, self.gl);
    }

    pub fn draw_pause(&mut self) {
        let p = PauseOverlay;
        p.draw_object(self.gl, self.ctx);
    }
}
