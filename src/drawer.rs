use graphics;
use opengl_graphics::GlGraphics;
use pieces::TetrisPiece;
use utils::*;

type GL = GlGraphics;
type CTX = graphics::Context;

pub struct Drawer<'a> {
    gl: &'a mut GL,
    ctx: CTX,
}

impl<'a> Drawer<'a> {
    pub fn new(gl: &'a mut GL, ctx: CTX) -> Self {
        Drawer { gl, ctx }
    }

    pub fn draw_piece_block(&mut self, i: isize, j: isize, piece: &TetrisPiece, is_shadow: bool) {
        let i = i as f64;
        let j = j as f64;

        let square = graphics::rectangle::square(BASE_X as f64 + j * WIDTH, i * WIDTH, WIDTH);
        graphics::rectangle(BGCOLOR, square, self.ctx.transform, self.gl);
        let square = graphics::rectangle::square(BASE_X as f64 + j * WIDTH + 1.0, i * WIDTH + 1.0, WIDTH - 2.0);
        let color = piece_to_color(piece, is_shadow);
        graphics::rectangle(color, square, self.ctx.transform, self.gl);
    }

    pub fn draw_next_block(&mut self, i: isize, j: isize, piece: &TetrisPiece, base_x: f64, base_y: f64) {
        let i = i as f64;
        let j = j as f64;

        let square = graphics::rectangle::square(BASE_X as f64 + j * WIDTH + base_x, i * WIDTH + base_y, WIDTH);
        graphics::rectangle(BGCOLOR, square, self.ctx.transform, self.gl);
        let square = graphics::rectangle::square(BASE_X as f64 + j * WIDTH + base_x + 1.0, i * WIDTH + 1.0 + base_y, WIDTH - 2.0);
        let color = piece_to_color(piece, false);
        graphics::rectangle(color, square, self.ctx.transform, self.gl);
    }

    pub fn draw_border(&mut self) {
        let border = graphics::rectangle::Rectangle::new_border(YELLOW, 1.0);
        let rect = graphics::rectangle::rectangle_by_corners(BASE_X as f64 - 1.0, 0.0, BASE_X as f64 + (WIDTH * 10.0) + 1.0, 600.0);
        border.draw(rect, &self.ctx.draw_state, self.ctx.transform, self.gl);
    }

    pub fn clear(&mut self) {
        graphics::clear(BGCOLOR, self.gl);
    }

    pub fn draw_pause(&mut self) {
        let overlay = graphics::rectangle::rectangle_by_corners(0.0, 0.0, 800.0, 600.0);
        graphics::rectangle(OVERLAY, overlay, self.ctx.transform, self.gl);
    }
}