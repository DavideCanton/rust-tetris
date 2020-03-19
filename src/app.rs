use crate::{
    board::TetrisBoard,
    controller::{Controller, ControllerKey},
    drawables::{
        drawable_obj::DrawableObject, drawable_piece::DrawablePiece, rectangle::Rectangle,
        text::DrawableText,
    },
    drawer::Drawer,
    pieces::{PieceRotation, TetrisPiece, TetrisPieceStruct},
    utils::*
};
use graphics::types::Scalar;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::input::*;

use rand::{prelude::ThreadRng, seq::SliceRandom, thread_rng};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(PartialEq, Eq, Debug)]
enum Moves {
    FALL,
    ROTATE,
    SIDE,
    DOWN,
    UP,
}

enum ScoreType {
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    TSpinMini,
    Tetris,
    AllClear,
}

const DEBUG: bool = false;

pub struct App<'a> {
    gl: RefCell<GlGraphics>,
    board: TetrisBoard,
    r: isize,
    c: isize,
    pause: bool,
    just_placed: bool,
    already_hold: bool,
    hold_piece: Option<TetrisPieceStruct>,
    piece: Option<TetrisPieceStruct>,
    rng: ThreadRng,
    time: f64,
    controller: Controller,
    last_movement: f64,
    removed_rows: i32,
    current_threshold: f64,
    old_threshold_sped_up: Option<f64>,
    buffer_next_pieces: VecDeque<TetrisPieceStruct>,
    internal_permutation: VecDeque<TetrisPiece>,
    last_move: Moves,
    last_score: Option<ScoreType>,
    back_to_back: bool,
    glyphs: Rc<RefCell<GlyphCache<'a>>>,
}

impl<'a> App<'a> {
    pub fn new(opengl: OpenGL, glyphs: GlyphCache<'a>, controller: Controller) -> Self {
        App {
            gl: RefCell::new(GlGraphics::new(opengl)),
            controller,
            board: TetrisBoard::new(R, C),
            r: 0,
            c: 0,
            just_placed: false,
            pause: false,
            already_hold: false,
            piece: None,
            hold_piece: None,
            rng: thread_rng(),
            time: 0.0,
            removed_rows: 0,
            last_movement: 0.0,
            current_threshold: INITIAL_MOVE_DOWN_THRESHOLD,
            old_threshold_sped_up: None,
            buffer_next_pieces: VecDeque::with_capacity(5),
            internal_permutation: VecDeque::with_capacity(7),
            last_move: Moves::FALL,
            last_score: None,
            glyphs: Rc::new(RefCell::new(glyphs)),
            back_to_back: false,
        }
    }

    pub fn start(&mut self) {
        // initial setup
        let rows = [];
        let pieces = [];

        self.initial_setup(&rows, &pieces);
        self.fill_buffer();
        self.next_block();

        while let Some(e) = self.controller.get_next_event() {
            if let Some(r) = e.render_args() {
                self.render(r);
            }

            if let Some(p) = e.press_args() {
                self.process_keys(p);
            }

            if let Some(u) = e.update_args() {
                if !self.pause {
                    self.update(u);
                }
            }
        }
    }

    fn initial_setup(&mut self, rows: &[&str], pieces: &[TetrisPiece]) {
        let mut ri = R - 1;
        let mut ci = 0;
        for r in rows {
            for c in r.chars() {
                if c == '1' {
                    self.board.set(ri, ci, TetrisPiece::OTHER);
                }
                ci += 1;
            }
            ri -= 1;
            ci = 0;
        }

        for p in pieces {
            self.buffer_next_pieces
                .push_front(TetrisPieceStruct::new(*p));
        }
    }

    fn render_next_block(&self, drawer: &mut Drawer) {
        let mut i = 0.0;
        let mut s = 0.0;

        for np in self.buffer_next_pieces.iter().rev().take(3) {
            let offset = if i == 0.0 { 0.0 } else { 50.0 };
            let pos = [BASE_X as Scalar + 355.0 + offset, s];
            s += np.height() as Scalar * WIDTH + 5.0;
            let dp = DrawablePiece::new(pos, np, false);
            drawer.draw_next_block(&dp);
            i += 1.0;
        }
    }

    fn get_shadow_row_index(&self, pieceInfo: &TetrisPieceStruct) -> Option<isize> {
        if self.pause {
            None
        } else {
            let mut shadow_r = self.r;

            while !pieceInfo.collides_on_next(shadow_r, self.c, &self.board) {
                shadow_r += 1;
            }

            Some(shadow_r)
        }
    }

    pub fn draw_board(
        &self,
        drawer: &mut Drawer,
        base_x: Scalar,
        base_y: Scalar,
        piece_board: &TetrisBoard,
    ) {
        for i in 0..piece_board.rows {
            for j in 0..piece_board.cols {
                if let Some(p) = piece_board.get(i, j) {
                    let i = i as isize;
                    let j = j as isize;

                    drawer.draw_square(i as isize, j as isize, p, base_x, base_y);
                }
            }
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        self.gl.borrow_mut().draw(args.viewport(), |ctx, gl| {
            let mut drawer = Drawer::new(gl, ctx);

            drawer.clear();
            drawer.draw_border();

            // draw next block
            self.render_next_block(&mut drawer);

            let board = &self.board;
            self.draw_board(&mut drawer, 0.0, 0.0, board);

            if self.pause {
                // draw pause
                drawer.draw_pause();
            }

            if let Some(last_point) = self.last_score.as_ref() {
                let pp = [HOLD_X as Scalar, (WIN_H as Scalar) - WIDTH * 3.0];

                let msg = String::from(match last_point {
                    ScoreType::TSpinSingle => "T-Spin Singolo!",
                    ScoreType::TSpinDouble => "T-Spin Doppio!",
                    ScoreType::TSpinTriple => "T-Spin Triplo!",
                    ScoreType::TSpinMini => "T-Spin Mini!",
                    ScoreType::Tetris => "Tetris!",
                    ScoreType::AllClear => "All Clear!",
                });
                let dpn = DrawableText::new(pp, &msg, 16, RED, self.glyphs.clone());
                dpn.draw_object(gl, ctx);

                if self.back_to_back {
                    let pp = [HOLD_X as Scalar, (WIN_H as Scalar) - WIDTH * 2.0];
                    let dpn2 = DrawableText::new(pp, "Back to back!", 16, RED, self.glyphs.clone());
                    dpn2.draw_object(gl, ctx);
                }
            }

            if let Some(pieceInfo) = self.hold_piece.as_ref() {
                let pp = [HOLD_X as Scalar, WIDTH];

                let dpn = DrawablePiece::new(pp, pieceInfo, false);
                dpn.draw_object(gl, ctx);
            }

            if let Some(pieceInfo) = self.piece.as_ref() {
                // compute position for shadow
                let shadow_r = self.get_shadow_row_index(pieceInfo);

                let pp = [
                    BASE_X as Scalar + self.c as Scalar * WIDTH,
                    self.r as Scalar * WIDTH,
                ];

                if DEBUG {
                    let size = TetrisPieceStruct::get_piece_size(pieceInfo.piece);
                    let r = Rectangle::new(
                        pp,
                        size.1 as Scalar * WIDTH,
                        size.0 as Scalar * WIDTH,
                        WHITE,
                    );
                    r.draw_object(gl, ctx);
                }

                let dpn = DrawablePiece::new(pp, pieceInfo, false);
                dpn.draw_object(gl, ctx);

                if let Some(shadow_r) = shadow_r {
                    if self.r + pieceInfo.height() <= shadow_r {
                        let ps = [
                            BASE_X as Scalar + self.c as Scalar * WIDTH,
                            shadow_r as Scalar * WIDTH,
                        ];
                        let dps = DrawablePiece::new(ps, pieceInfo, true);
                        dps.draw_object(gl, ctx);
                    }
                }
            }
        });
    }

    fn handle_finalize(&mut self) {
        let piece = self.piece.as_ref().unwrap();
        self.board.finalize(piece, self.r as isize, self.c as isize);
        let old_removed_rows = self.removed_rows;

        let completed_rows_ranges = self.board.completed_rows();
        let completed_rows = completed_rows_ranges
            .iter()
            .map(|r| (r.0 - r.1) as i32)
            .sum();

        self.removed_rows += completed_rows;
        let last = self.last_score.take();

        if completed_rows == 4 {
            self.last_score = Some(ScoreType::Tetris);
        } else if piece.piece == TetrisPiece::T {
            // detect T-spin

            // TODO check for mini T-spin
            if completed_rows > 0 && self.last_move == Moves::ROTATE {
                let center_r = self.r + 1;
                let center_c = self.c + 1;
                let mut occupied = 0;

                for i in &[-1, 1] {
                    for j in &[-1, 1] {
                        let ei = center_r + i;
                        let ej = center_c + j;

                        if ei < 0 || ei >= R || ej < 0 || ej >= C || self.board.is_set(ei, ej) {
                            occupied += 1;
                        }
                    }
                }

                if occupied >= 3 {
                    self.last_score = match completed_rows {
                        1 => Some(ScoreType::TSpinSingle),
                        2 => Some(ScoreType::TSpinDouble),
                        3 => Some(ScoreType::TSpinTriple),
                        _ => None,
                    };
                }
            }
        }

        if completed_rows > 0 {
            self.back_to_back = last.is_some() && self.last_score.is_some();
        }

        self.board.remove_ranges(completed_rows_ranges, Some(20));

        if self.board.is_empty() {
            self.back_to_back = false;
            self.last_score = Some(ScoreType::AllClear);
        }

        if old_removed_rows / 10 != self.removed_rows / 10 && self.current_threshold > 0.1 {
            self.current_threshold -= 0.1;
        }
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.time += args.dt;

        if self.just_placed {
            let piece = self.piece.as_ref().unwrap();

            if piece.collides_on_next(self.r, self.c, &self.board) {
                println!("Game over!");
                self.controller.close_window();
            }
        }

        if self.time - self.last_movement >= self.current_threshold {
            let mut next_block = false;
            self.just_placed = false;

            {
                let piece: &TetrisPieceStruct = self.piece.as_ref().unwrap();

                if piece.collides_on_next(self.r, self.c, &self.board) {
                    next_block = true;
                } else {
                    self.r += 1;
                    self.last_move = Moves::FALL;
                    self.last_movement = self.time;
                }
            }

            if next_block {
                self.handle_finalize();
                self.next_block();
            }
        }

        self.current_threshold = self.old_threshold_sped_up.unwrap_or(self.current_threshold);
        self.old_threshold_sped_up = None;
    }

    fn enter_key_pressed(&mut self) {
        self.pause = !self.pause;
    }

    fn left_key_pressed(&mut self) {
        let piece = self.piece.as_ref().unwrap();
        let first_col = piece.board.get_first_set_col().unwrap() as isize;

        if self.c + first_col > 0 && !piece.collides_left(self.r, self.c, &self.board) {
            self.c -= 1;
            self.last_move = Moves::SIDE;
        }
    }

    fn right_key_pressed(&mut self) {
        let piece = self.piece.as_ref().unwrap();
        let last_col = piece.board.get_last_set_col().unwrap() as isize;

        if self.c + last_col < (self.board.cols as isize) - 1
            && !piece.collides_right(self.r, self.c, &self.board)
        {
            self.c += 1;
            self.last_move = Moves::SIDE;
        }
    }

    fn rot_pressed(&mut self, next: bool) {
        let piece = self.piece.as_mut().unwrap();
        let prev_rot = piece.rotation;

        if next {
            piece.rotate_piece();
        } else {
            piece.rotate_piece_prev();
        }

        let mut ok = false;
        for kick in piece.get_kicks(prev_rot) {
            if !piece.collides(self.r, self.c, &self.board, kick) {
                ok = true;
                self.r -= kick.1;
                self.c += kick.0;
                break;
            }
        }

        if !ok {
            if !next {
                piece.rotate_piece();
            } else {
                piece.rotate_piece_prev();
            }
        } else {
            self.last_move = Moves::ROTATE;
        }
    }

    fn next_rot_pressed(&mut self) {
        self.rot_pressed(true);
    }

    fn prev_rot_pressed(&mut self) {
        self.rot_pressed(false);
    }

    fn up_key_pressed(&mut self) {
        let piece = self.piece.as_ref().unwrap();

        while !piece.collides_on_next(self.r, self.c, &self.board) {
            self.r += 1;
        }

        self.handle_finalize();
        self.next_block();

        self.last_move = Moves::UP;
    }

    fn hold_key_pressed(&mut self) {
        if !self.already_hold {
            let hold_piece = self.hold_piece.take();
            let piece = self.piece.take();

            self.hold_piece = piece;
            self.piece = hold_piece;
            self.hold_piece.as_mut().unwrap().rotation = PieceRotation::ZERO;

            if self.piece.is_none() {
                self.next_block();
            }
            self.already_hold = true;
        }
    }

    fn down_key_pressed(&mut self) {
        match self.old_threshold_sped_up {
            None => {
                self.old_threshold_sped_up = Some(self.current_threshold);
                self.current_threshold = SPED_UP_THRESHOLD;
                self.last_move = Moves::DOWN;
            }
            Some(_) => {}
        }
    }

    fn exec_if_not_paused<F: Fn(&mut App<'a>)>(&mut self, ex: F) {
        if !self.pause {
            ex(self);
        }
    }

    pub fn process_keys(&mut self, args: Button) {
        match self.controller.get_key(args) {
            Some(ControllerKey::Return) => self.enter_key_pressed(),
            Some(ControllerKey::Left) => self.exec_if_not_paused(App::left_key_pressed),
            Some(ControllerKey::Right) => self.exec_if_not_paused(App::right_key_pressed),
            Some(ControllerKey::NextRotation) => self.exec_if_not_paused(App::next_rot_pressed),
            Some(ControllerKey::PrevRotation) => self.exec_if_not_paused(App::prev_rot_pressed),
            Some(ControllerKey::Down) => self.exec_if_not_paused(App::down_key_pressed),
            Some(ControllerKey::Up) => self.exec_if_not_paused(App::up_key_pressed),
            Some(ControllerKey::Hold) => self.exec_if_not_paused(App::hold_key_pressed),
            _ => {}
        }
    }

    fn new_block_in_buffer(&mut self) {
        if self.internal_permutation.is_empty() {
            self.fill_permutation();
        }

        let piece = self.internal_permutation.pop_front().unwrap();
        self.buffer_next_pieces
            .push_front(TetrisPieceStruct::new(piece));
    }

    fn fill_permutation(&mut self) {
        let mut nums: Vec<_> = vec![
            TetrisPiece::I,
            TetrisPiece::S,
            TetrisPiece::Z,
            TetrisPiece::O,
            TetrisPiece::T,
            TetrisPiece::L,
            TetrisPiece::J,
        ];
        nums.as_mut_slice().shuffle(&mut self.rng);
        self.internal_permutation.extend(nums.into_iter());
    }

    fn fill_buffer(&mut self) {
        for _ in 0..5 {
            self.new_block_in_buffer()
        }
    }

    fn next_block(&mut self) {
        let piece = self.buffer_next_pieces.pop_back().unwrap();
        self.r = 0;
        self.c = C / 2 - 1;
        self.piece = Some(piece);
        self.new_block_in_buffer();
        self.current_threshold = self.old_threshold_sped_up.unwrap_or(self.current_threshold);
        self.old_threshold_sped_up = None;
        self.just_placed = true;
        self.already_hold = false;
    }
}
