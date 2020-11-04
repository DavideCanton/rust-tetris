use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use graphics::{
    Context,
    math::Vec2d,
    types::{Color, FontSize},
};
use graphics::{Transformed, types::Scalar};
use opengl_graphics::{GlGraphics, GlyphCache};

use rust_tetris_core:: {
    board::TetrisBoard,
    pieces::TetrisPiece,
    pieces::TetrisPieceType,

};
use rust_tetris_core::board::TetrisCell;

use crate::{
    app_structs::{HoldTetrisPiece, TetrisPieceWithPosition},
    utils::*,
};

pub struct Drawer<'a, 'b> {
    gl: &'a mut GlGraphics,
    ctx: Context,
    glyphs: Rc<RefCell<GlyphCache<'b>>>,
}

impl<'a, 'b> Drawer<'a, 'b> {
    pub fn new(gl: &'a mut GlGraphics, ctx: Context, glyphs: Rc<RefCell<GlyphCache<'b>>>) -> Self {
        Drawer { gl, ctx, glyphs }
    }

    pub fn try_draw_shadow(&mut self, shadow_r: isize, piece: &TetrisPieceWithPosition) {
        if piece.row() + piece.tetris_piece_ref().height() <= shadow_r {
            let ps = [
                BASE_X as Scalar + piece.col() as Scalar * WIDTH,
                shadow_r as Scalar * WIDTH,
            ];
            self.draw_piece_struct(ps, piece.tetris_piece_ref(), true);
        }
    }

    pub fn draw_piece_on_board(&mut self, piece: &TetrisPieceWithPosition) {
        let pp = [
            BASE_X as Scalar + piece.col() as Scalar * WIDTH,
            piece.row() as Scalar * WIDTH,
        ];
        self.draw_piece_struct(pp, piece.tetris_piece_ref(), false);
    }

    fn draw_piece_struct(&mut self, base: Vec2d, piece: &TetrisPiece, is_shadow: bool) {
        piece.call_on_set_cells(|i, j| {
            let i = i as Scalar;
            let j = j as Scalar;
            let pos = [j * WIDTH, i * WIDTH];
            let color = playable_piece_to_color(piece.piece_type, is_shadow);
            self.draw_square_by_pos([base[0] + pos[0], base[1] + pos[1]], WIDTH, color);
        });
    }

    pub fn draw_hold_piece(&mut self, piece: &HoldTetrisPiece) {
        let pp = [HOLD_X as Scalar, WIDTH];
        self.draw_piece_struct(pp, &piece.piece, false);
    }

    pub fn draw_board(&mut self, base_x: Scalar, base_y: Scalar, piece_board: &TetrisBoard) {
        for i in 0..piece_board.rows {
            for j in 0..piece_board.cols {
                if let TetrisCell::FilledCell(p) = piece_board.get(i, j) {
                    let i = i as isize;
                    let j = j as isize;

                    self.draw_square_by_index(i as isize, j as isize, p, base_x, base_y);
                }
            }
        }
    }

    fn draw_square_by_index(
        &mut self,
        i: isize,
        j: isize,
        piece: TetrisPieceType,
        base_x: Scalar,
        base_y: Scalar,
    ) {
        let i = i as Scalar;
        let j = j as Scalar;

        let pos = [BASE_X as Scalar + j * WIDTH + base_x, i * WIDTH + base_y];
        let width = WIDTH;

        let color = piece_to_color(piece, false);
        self.draw_square(pos, width, color);
    }

    fn draw_square_by_pos(&mut self, pos: Vec2d, width: Scalar, color: Color) {
        self.draw_square(pos, width, color);
    }

    fn draw_square(&mut self, pos: Vec2d, width: Scalar, color: Color) {
        let ctx = self.ctx.trans_pos(pos);

        let square = graphics::rectangle::square(0.0, 0.0, width);
        graphics::rectangle(BGCOLOR, square, ctx.transform, self.gl);
        let square = graphics::rectangle::square(1.0, 1.0, width - 2.0);
        graphics::rectangle(color, square, ctx.transform, self.gl);
    }

    pub fn draw_score_text(&mut self, text: &str) {
        let pp = [HOLD_X as Scalar, (WIN_H as Scalar) - WIDTH * 3.0];
        self.draw_text(RED, 16, text, pp);
    }

    pub fn draw_b2b_text(&mut self) {
        let pp = [HOLD_X as Scalar, (WIN_H as Scalar) - WIDTH * 2.0];
        self.draw_text(RED, 16, "Back to back!", pp);
    }

    fn draw_text(&mut self, color: Color, size: FontSize, text: &str, pos: Vec2d) {
        let r = self.glyphs.clone();
        let mut g = r.borrow_mut();
        let ctx = self.ctx.trans_pos(pos);
        graphics::text(color, size, text, g.deref_mut(), ctx.transform, self.gl).expect("No");
    }

    pub fn draw_border(&mut self) {
        let border = graphics::rectangle::Rectangle::new_border(YELLOW, 1.0);

        let rect = graphics::rectangle::rectangle_by_corners(
            0.0,
            0.0,
            1.0 * 2.0 + WIDTH * 10.0,
            1.0 * 2.0 + 600.0,
        );
        let ctx = self.ctx.trans(BASE_X as Scalar - 1.0, 0.0);
        border.draw(rect, &ctx.draw_state, ctx.transform, self.gl);
    }

    pub fn draw_queue_piece(&mut self, index: usize, np: &TetrisPiece) {
        let i = index as Scalar;
        let offset = if i == 0.0 { 0.0 } else { 50.0 };
        let pos = [BASE_X as Scalar + 355.0 + offset, i * WIDTH * 4.0 + 5.0];
        self.draw_piece_struct(pos, np, false);
    }

    pub fn clear(&mut self) {
        graphics::clear(BGCOLOR, self.gl);
    }

    pub fn draw_pause(&mut self) {
        let overlay_color = [0.0, 0.0, 0.0, 0.8];
        let overlay =
            graphics::rectangle::rectangle_by_corners(0.0, 0.0, WIN_W.into(), WIN_H.into());
        graphics::rectangle(overlay_color, overlay, self.ctx.transform, self.gl);
    }
}
