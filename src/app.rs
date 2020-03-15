use crate::{
    board::TetrisBoard,
    controller::{Controller, ControllerKey},
    drawables::{
        drawable_obj::DrawableObject, drawable_piece::DrawablePiece, rectangle::Rectangle,
    },
    drawer::Drawer,
    pieces::{TetrisPiece, TetrisPieceStruct},
    utils::{BASE_X, C, INITIAL_MOVE_DOWN_THRESHOLD, R, SPED_UP_THRESHOLD, WHITE, WIDTH},
};
use enum_primitive::FromPrimitive;
use graphics::types::Scalar;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use rand::{prelude::ThreadRng, seq::SliceRandom, thread_rng};
use std::{cell::RefCell, collections::VecDeque};

const DEBUG: bool = false;

pub struct App {
    gl: RefCell<GlGraphics>,
    board: TetrisBoard,
    r: isize,
    c: isize,
    pause: bool,
    just_placed: bool,
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
}

impl App {
    pub fn new(opengl: OpenGL, controller: Controller) -> Self {
        App {
            gl: RefCell::new(GlGraphics::new(opengl)),
            controller,
            board: TetrisBoard::new(R, C),
            r: 0,
            c: 0,
            just_placed: false,
            pause: false,
            piece: None,
            rng: thread_rng(),
            time: 0.0,
            removed_rows: 0,
            last_movement: 0.0,
            current_threshold: INITIAL_MOVE_DOWN_THRESHOLD,
            old_threshold_sped_up: None,
            buffer_next_pieces: VecDeque::with_capacity(5),
            internal_permutation: VecDeque::with_capacity(7),
        }
    }

    pub fn start(&mut self) {
        self.fill_buffer();
        self.next_block();
        //initial setup
        let rows = [
            "1101111111",
            "1101111111",
            "1000111111",
            "1001111111",
            "1101111111",
            "0001111111",
            "0011111111",
        ];

        let mut ri = R - 1;
        let mut ci = 0;
        for r in &rows {
            for c in r.chars() {
                if c == '1' {
                    self.board.set(ri, ci, TetrisPiece::OTHER);
                }
                ci += 1;
            }
            ri -= 1;
            ci = 0;
        }

        while let Some(e) = self.controller.get_next_event() {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(p) = e.press_args() {
                self.process_keys(&p);
            }

            if let Some(u) = e.update_args() {
                if !self.pause {
                    self.update(&u);
                }
            }
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

    pub fn render(&mut self, args: &RenderArgs) {
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
        self.removed_rows += self.board.remove_completed_rows(Some(20));
        if old_removed_rows / 10 != self.removed_rows / 10 && self.current_threshold > 0.1 {
            self.current_threshold -= 0.1;
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
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
        }
    }

    fn right_key_pressed(&mut self) {
        let piece = self.piece.as_ref().unwrap();
        let last_col = piece.board.get_last_set_col().unwrap() as isize;

        if self.c + last_col < (self.board.cols as isize) - 1
            && !piece.collides_right(self.r, self.c, &self.board)
        {
            self.c += 1;
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
    }

    fn down_key_pressed(&mut self) {
        match self.old_threshold_sped_up {
            None => {
                self.old_threshold_sped_up = Some(self.current_threshold);
                self.current_threshold = SPED_UP_THRESHOLD;
            }
            Some(_) => {}
        }
    }

    fn exec_if_not_paused<F: Fn(&mut App)>(&mut self, ex: F) {
        if !self.pause {
            ex(self);
        }
    }

    pub fn process_keys(&mut self, args: &Button) {
        match self.controller.get_key(args) {
            Some(ControllerKey::Return) => self.enter_key_pressed(),
            Some(ControllerKey::Left) => self.exec_if_not_paused(App::left_key_pressed),
            Some(ControllerKey::Right) => self.exec_if_not_paused(App::right_key_pressed),
            Some(ControllerKey::NextRotation) => self.exec_if_not_paused(App::next_rot_pressed),
            Some(ControllerKey::PrevRotation) => self.exec_if_not_paused(App::prev_rot_pressed),
            Some(ControllerKey::Down) => self.exec_if_not_paused(App::down_key_pressed),
            Some(ControllerKey::Up) => self.exec_if_not_paused(App::up_key_pressed),
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
        let mut nums: Vec<_> = (0..7).map(|n| TetrisPiece::from_u8(n).unwrap()).collect();
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
    }
}
