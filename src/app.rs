use crate::{
    app_structs::{HoldTetrisPiece, TetrisPieceWithPosition},
    board::TetrisBoard,
    controller::{Controller, ControllerKey},
    drawer::Drawer,
    pieces::{TetrisPiece, TetrisPieceType},
    utils::{C, INITIAL_MOVE_DOWN_THRESHOLD, R, SPED_UP_THRESHOLD},
};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::input::*;

use crate::pieces::TetrisPieceRotation;
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

pub struct App<'a> {
    gl: RefCell<GlGraphics>,
    board: TetrisBoard,
    piece: Option<TetrisPieceWithPosition>,
    pause: bool,
    just_placed: bool,
    hold_piece: Option<HoldTetrisPiece>,
    rng: ThreadRng,
    time: f64,
    controller: Controller,
    last_movement: f64,
    removed_rows: i32,
    current_threshold: f64,
    old_threshold_sped_up: Option<f64>,
    buffer_next_pieces: VecDeque<TetrisPiece>,
    internal_permutation: VecDeque<TetrisPieceType>,
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
            just_placed: false,
            pause: false,
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

    fn initial_setup(&mut self, rows: &[&str], pieces: &[TetrisPieceType]) {
        let mut row_index = R - 1;
        let mut col_index = 0;
        for r in rows {
            for c in r.chars() {
                if c == '1' {
                    self.board.set(row_index, col_index, TetrisPieceType::OTHER);
                }
                col_index += 1;
            }
            row_index -= 1;
            col_index = 0;
        }

        for piece in pieces {
            self.buffer_next_pieces.push_front(TetrisPiece::new(*piece));
        }
    }

    fn get_shadow_row_index(&self, pieceInfo: &TetrisPieceWithPosition) -> Option<isize> {
        if self.pause {
            None
        } else {
            let mut shadow_row = pieceInfo.row();

            while !pieceInfo.collides_on_next_with_row(shadow_row, &self.board) {
                shadow_row += 1;
            }

            Some(shadow_row)
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        self.gl.borrow_mut().draw(args.viewport(), |ctx, gl| {
            let mut drawer = Drawer::new(gl, ctx, self.glyphs.clone());

            drawer.clear();
            drawer.draw_border();

            for (index, np) in self.buffer_next_pieces.iter().rev().take(5).enumerate() {
                drawer.draw_queue_piece(index, np);
            }

            let board = &self.board;
            drawer.draw_board(0.0, 0.0, board);

            if self.pause {
                // draw pause
                drawer.draw_pause();
            }

            if let Some(last_point) = self.last_score.as_ref() {
                let msg = String::from(match last_point {
                    ScoreType::TSpinSingle => "T-Spin Singolo!",
                    ScoreType::TSpinDouble => "T-Spin Doppio!",
                    ScoreType::TSpinTriple => "T-Spin Triplo!",
                    ScoreType::TSpinMini => "T-Spin Mini!",
                    ScoreType::Tetris => "Tetris!",
                    ScoreType::AllClear => "All Clear!",
                });

                drawer.draw_score_text(&msg);

                if self.back_to_back {
                    drawer.draw_b2b_text();
                }
            }

            if let Some(pieceInfo) = self.hold_piece.as_ref() {
                drawer.draw_hold_piece(pieceInfo);
            }

            if let Some(pieceInfo) = self.piece.as_ref() {
                // compute position for shadow
                drawer.draw_piece_on_board(pieceInfo);

                let shadow_r = self.get_shadow_row_index(&pieceInfo);
                if let Some(shadow_r) = shadow_r {
                    drawer.try_draw_shadow(shadow_r, pieceInfo);
                }
            }
        });
    }

    fn handle_finalize(&mut self) {
        let piece_with_position = self.piece.as_ref().unwrap();
        piece_with_position.finalize_on(&mut self.board);
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
        } else if piece_with_position.tetris_piece_ref().pieceType == TetrisPieceType::T {
            // detect T-spin

            // TODO check for mini T-spin
            if completed_rows > 0 && self.last_move == Moves::ROTATE {
                let center_r = piece_with_position.row() + 1;
                let center_c = piece_with_position.col() + 1;
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
            // TODO check this
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

            if piece.collides_on_next(&self.board) {
                println!("Game over!");
                self.controller.close_window();
            }
        }

        if self.time - self.last_movement >= self.current_threshold {
            let mut next_block = false;
            self.just_placed = false;

            {
                let piece = self.piece.as_mut().unwrap();

                if piece.collides_on_next(&self.board) {
                    next_block = true;
                } else {
                    piece.move_down();
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
        let piece = self.piece.as_mut().unwrap();
        if piece.try_move_left(&self.board) {
            self.last_move = Moves::SIDE;
        }
    }

    fn right_key_pressed(&mut self) {
        let piece = self.piece.as_mut().unwrap();
        if piece.try_move_right(&self.board) {
            self.last_move = Moves::SIDE;
        }
    }

    fn rot_pressed(&mut self, next: bool) {
        let piece_with_pos = self.piece.as_mut().unwrap();
        let prev_rot: TetrisPieceRotation;

        {
            let piece_ref = piece_with_pos.tetris_piece_mut();
            prev_rot = piece_ref.rotation;

            if next {
                piece_ref.rotate_piece();
            } else {
                piece_ref.rotate_piece_prev();
            }
        }

        let mut ok = false;

        if let Some(kick) = piece_with_pos.can_rotate(prev_rot, &self.board) {
            piece_with_pos.kick_by(kick);
            ok = true;
        }

        {
            let piece_ref = piece_with_pos.tetris_piece_mut();
            if !ok {
                if !next {
                    piece_ref.rotate_piece();
                } else {
                    piece_ref.rotate_piece_prev();
                }
            } else {
                self.last_move = Moves::ROTATE;
            }
        }
    }

    fn next_rot_pressed(&mut self) {
        self.rot_pressed(true);
    }

    fn prev_rot_pressed(&mut self) {
        self.rot_pressed(false);
    }

    fn up_key_pressed(&mut self) {
        let piece = self.piece.as_mut().unwrap();

        while !piece.collides_on_next(&self.board) {
            piece.move_down();
        }

        self.handle_finalize();
        self.next_block();

        self.last_move = Moves::UP;
    }

    fn hold_key_pressed(&mut self) {
        if HoldTetrisPiece::can_swap(&self.hold_piece) {
            let p = self.piece.take();
            let hp = self.hold_piece.take();

            self.hold_piece = Some(HoldTetrisPiece::new(p.unwrap().tetris_piece()));
            if let Some(hp) = hp {
                self.piece = Some(App::build_piece_with_pos(hp.piece));
            }

            if self.piece.is_none() {
                self.next_block();
            }

            self.hold_piece.as_mut().unwrap().set_hold();
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
        self.buffer_next_pieces.push_front(TetrisPiece::new(piece));
    }

    fn fill_permutation(&mut self) {
        let mut nums: Vec<_> = vec![
            TetrisPieceType::I,
            TetrisPieceType::S,
            TetrisPieceType::Z,
            TetrisPieceType::O,
            TetrisPieceType::T,
            TetrisPieceType::L,
            TetrisPieceType::J,
        ];
        nums.as_mut_slice().shuffle(&mut self.rng);
        self.internal_permutation.extend(nums.into_iter());
    }

    fn fill_buffer(&mut self) {
        for _ in 0..5 {
            self.new_block_in_buffer()
        }
    }

    fn build_piece_with_pos(piece: TetrisPiece) -> TetrisPieceWithPosition {
        TetrisPieceWithPosition::new(0, C / 2 - 1, piece)
    }

    fn next_block(&mut self) {
        let piece = self.buffer_next_pieces.pop_back().unwrap();
        self.piece = Some(App::build_piece_with_pos(piece));
        self.new_block_in_buffer();
        self.current_threshold = self.old_threshold_sped_up.unwrap_or(self.current_threshold);
        self.old_threshold_sped_up = None;
        self.just_placed = true;
        if let Some(hold_piece) = self.hold_piece.as_mut() {
            hold_piece.reset_hold();
        }
    }
}
