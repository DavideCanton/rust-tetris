use ggez::{
    graphics,
    graphics::{Color, DrawMode, DrawParam, Font, Mesh, Rect, Scale, Text},
    mint::Point2,
    Context, GameResult,
};

use rust_tetris_core::{
    board::{TetrisBoard, TetrisCell},
    pieces::TetrisPiece,
    pieces::TetrisPieceType,
};

use crate::{
    app_structs::{HoldTetrisPiece, TetrisPieceWithPosition},
    utils::*,
};

type Scalar = f32;

pub struct Drawer<'a> {
    ctx: &'a mut Context,
    font: Font,
}

const SCORE_SIZE: f32 = 24.0;

impl<'a> Drawer<'a> {
    pub fn new(ctx: &'a mut Context, font: Font) -> Self {
        Drawer { ctx, font }
    }

    pub fn try_draw_shadow(
        &mut self,
        shadow_r: isize,
        piece: &TetrisPieceWithPosition,
    ) -> GameResult {
        if piece.row() + piece.tetris_piece_ref().height() <= shadow_r {
            let ps = Point2::from([
                BASE_X as Scalar + piece.col() as Scalar * WIDTH,
                shadow_r as Scalar * WIDTH,
            ]);
            self.draw_piece_struct(ps, piece.tetris_piece_ref(), true, None)
        } else {
            Ok(())
        }
    }

    pub fn draw_piece_on_board(&mut self, piece: &TetrisPieceWithPosition) -> GameResult {
        let pp = Point2::from([
            BASE_X as Scalar + piece.col() as Scalar * WIDTH,
            piece.row() as Scalar * WIDTH,
        ]);
        self.draw_piece_struct(pp, piece.tetris_piece_ref(), false, None)
    }

    fn draw_piece_struct(
        &mut self,
        base: Point2<Scalar>,
        piece: &TetrisPiece,
        is_shadow: bool,
        override_color: Option<Color>,
    ) -> GameResult {
        piece.call_on_set_cells_with_result(|i, j| {
            let i = i as Scalar;
            let j = j as Scalar;
            let pos = [j * WIDTH, i * WIDTH];
            let color =
                override_color.unwrap_or(playable_piece_to_color(piece.piece_type, is_shadow));
            self.draw_square_by_pos(
                Point2::from([base.x + pos[0], base.y + pos[1]]),
                WIDTH,
                color,
            )
        })
    }

    pub fn draw_hold_piece(&mut self, piece: &HoldTetrisPiece, can_swap: bool) -> GameResult {
        let pp = Point2::from([HOLD_X as Scalar, WIDTH]);
        let color = match can_swap {
            true => None,
            false => Some(OTHER_COLOR),
        };
        self.draw_piece_struct(pp, &piece.piece, false, color)
    }

    pub fn draw_board(
        &mut self,
        base_x: Scalar,
        base_y: Scalar,
        piece_board: &TetrisBoard,
    ) -> GameResult {
        for i in 0..piece_board.rows {
            for j in 0..piece_board.cols {
                if let TetrisCell::FilledCell(p) = piece_board.get(i, j) {
                    let i = i as isize;
                    let j = j as isize;

                    self.draw_square_by_index(i as isize, j as isize, p, base_x, base_y)?
                }
            }
        }
        Ok(())
    }

    fn draw_square_by_index(
        &mut self,
        i: isize,
        j: isize,
        piece: TetrisPieceType,
        base_x: Scalar,
        base_y: Scalar,
    ) -> GameResult {
        let i = i as Scalar;
        let j = j as Scalar;

        let pos = Point2::from([BASE_X as Scalar + j * WIDTH + base_x, i * WIDTH + base_y]);
        let width = WIDTH;

        let color = piece_to_color(piece, false);
        self.draw_square(pos, width, color)
    }

    fn draw_square_by_pos(
        &mut self,
        pos: Point2<Scalar>,
        width: Scalar,
        color: Color,
    ) -> GameResult {
        self.draw_square(pos, width, color)
    }

    fn draw_square(&mut self, pos: Point2<Scalar>, width: Scalar, color: Color) -> GameResult {
        let square = Mesh::new_rectangle(
            self.ctx,
            DrawMode::fill(),
            Rect::new(pos.x + 1.0, pos.y + 1.0, width - 2.0, width - 2.0),
            color,
        )?;
        graphics::draw(self.ctx, &square, DrawParam::default())
    }

    pub fn draw_score_text(&mut self, text: &str) -> GameResult {
        let pp = Point2::from([HOLD_X as Scalar, (WIN_H as Scalar) - WIDTH * 3.0]);
        self.draw_text(RED, SCORE_SIZE, text, pp)
    }

    pub fn draw_b2b_text(&mut self, current_b2b: u32) -> GameResult {
        let pp = Point2::from([HOLD_X as Scalar, (WIN_H as Scalar) - WIDTH * 2.0]);
        self.draw_text(
            RED,
            SCORE_SIZE,
            &format!("Back to back: {}x", current_b2b),
            pp,
        )
    }

    fn draw_text(
        &mut self,
        color: Color,
        size: f32,
        text: &str,
        pos: Point2<Scalar>,
    ) -> GameResult {
        let mut text = Text::new(text);
        text.set_font(self.font.clone(), Scale::uniform(size));
        graphics::draw(self.ctx, &text, DrawParam::default().dest(pos).color(color))
    }

    pub fn draw_border(&mut self) -> GameResult {
        let border = Mesh::new_rectangle(
            self.ctx,
            DrawMode::stroke(1.0),
            Rect::new(
                BASE_X - 1.0,
                0.0,
                1.0 * 2.0 + WIDTH * 10.0,
                1.0 * 2.0 + 600.0,
            ),
            YELLOW,
        )?;
        graphics::draw(self.ctx, &border, DrawParam::default())
    }

    pub fn draw_queue_piece(&mut self, index: usize, np: &TetrisPiece) -> GameResult {
        let i = index as Scalar;
        let offset = if i == 0.0 { 0.0 } else { 50.0 };
        let pos = Point2::from([BASE_X as Scalar + 355.0 + offset, i * WIDTH * 4.0 + 5.0]);
        self.draw_piece_struct(pos, np, false, None)
    }

    pub fn clear(&mut self) -> GameResult {
        graphics::clear(self.ctx, BGCOLOR);
        Ok(())
    }

    pub fn draw_pause(&mut self) -> GameResult {
        let overlay_color = Color::from([0.0, 0.0, 0.0, 0.8]);
        let overlay = Mesh::new_rectangle(
            self.ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, WIN_W, WIN_H),
            overlay_color,
        )?;
        graphics::draw(self.ctx, &overlay, DrawParam::default())
    }
}
