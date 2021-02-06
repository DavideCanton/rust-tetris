use crate::GameConfig;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::rc::Rc;

use ggez::{graphics, graphics::Font, timer, Context, GameResult};
use log::{debug, trace};
use rand::{prelude::ThreadRng, seq::SliceRandom, thread_rng};

use rust_tetris_core::{
    board::TetrisBoard,
    constants::Kick,
    enums::{PlayableTetrisPieceType, TetrisPieceRotation, TetrisPieceType},
    pieces::TetrisPiece,
};
use rust_tetris_ui_core::{
    app_structs::{HoldTetrisPiece, TetrisPieceWithPosition},
    drawer::Drawer,
    utils::{is_not_empty, C, R},
};

use crate::types::TetrisUpdateResult;

#[derive(PartialEq, Eq, Debug)]
enum Moves {
    FALL,
    ROTATE,
    SIDE,
    DOWN,
    UP,
}

#[derive(PartialEq, Eq, Debug)]
enum SideMoves {
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy)]
enum ScoreType {
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    TSpinMini,
    Tetris,
    AllClear,
    Single,
    Double,
    Triple,
}

fn is_b2b_worth(s: ScoreType) -> bool {
    use ScoreType::*;
    !matches!(s, Single | Double | Triple)
}

pub struct App {
    board: TetrisBoard,
    piece: Option<TetrisPieceWithPosition>,
    pause: bool,
    just_placed: bool,
    hold_piece: Option<HoldTetrisPiece>,
    rng: ThreadRng,
    down_movement_accumulator: f64,
    side_movement_accumulator: f64,
    frames_for_das: i32,
    current_gravity: f64,
    buffer_next_pieces: VecDeque<TetrisPiece>,
    internal_permutation: VecDeque<PlayableTetrisPieceType>,
    last_move: Moves,
    last_score: Option<ScoreType>,
    lock_timer: u32,
    back_to_back: u32,
    current_combo: u32,
    font: Font,
    last_kick: Option<Kick>,
    config: Rc<GameConfig>,
    side_move_to_perform: Option<SideMoves>,
}

impl App {
    pub fn new(font: Font, config: Rc<GameConfig>) -> Self {
        let current_gravity = config.game_params.gravity;
        App {
            font,
            board: TetrisBoard::new(R, C),
            just_placed: false,
            pause: false,
            piece: None,
            hold_piece: None,
            rng: thread_rng(),
            config,
            down_movement_accumulator: 0.0,
            side_movement_accumulator: 0.0,
            current_gravity,
            current_combo: 0,
            buffer_next_pieces: VecDeque::with_capacity(5),
            internal_permutation: VecDeque::with_capacity(7),
            last_move: Moves::FALL,
            last_score: None,
            back_to_back: 0,
            frames_for_das: 0,
            lock_timer: 0,
            last_kick: None,
            side_move_to_perform: None,
        }
    }

    pub fn start(&mut self) {
        // initial setup
        let rows = [];
        let pieces = [];

        self.initial_setup(&rows, &pieces);
        self.fill_buffer();
        self.next_block(None);
    }

    fn initial_setup(&mut self, rows: &[&str], pieces: &[PlayableTetrisPieceType]) {
        let mut row_index = R - 1;
        let mut col_index = 0;
        for r in rows.iter().rev() {
            for c in r.chars() {
                if c != ' ' {
                    self.board
                        .set(row_index, col_index, TetrisPieceType::NotPlayable);
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

    fn get_shadow_row_index(&self, piece_info: &TetrisPieceWithPosition) -> Option<isize> {
        if self.pause {
            None
        } else {
            let mut shadow_row = piece_info.row();

            while !piece_info.collides_on_next_with_row(shadow_row, &self.board) {
                shadow_row += 1;
            }

            Some(shadow_row)
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        let mut drawer = Drawer::new(ctx, self.font);

        drawer.clear()?;
        drawer.draw_border()?;

        for (index, np) in self.buffer_next_pieces.iter().rev().take(5).enumerate() {
            drawer.draw_queue_piece(index, np)?;
        }

        let board = &self.board;
        drawer.draw_board(0.0, 0.0, board)?;

        if self.pause {
            // draw pause
            drawer.draw_pause()?;
        }

        if let Some(last_point) = self.last_score.as_ref() {
            let msg = String::from(match last_point {
                ScoreType::TSpinSingle => "T-Spin Single!",
                ScoreType::TSpinDouble => "T-Spin Double!",
                ScoreType::TSpinTriple => "T-Spin Triple!",
                ScoreType::TSpinMini => "T-Spin Mini!",
                ScoreType::Tetris => "Tetris!",
                ScoreType::AllClear => "All Clear!",
                ScoreType::Single => "Single!",
                ScoreType::Double => "Double!",
                ScoreType::Triple => "Triple!",
            });

            drawer.draw_score_text(&msg)?;
        }

        if self.back_to_back > 0 {
            drawer.draw_b2b_text(self.back_to_back)?;
        }

        if self.current_combo > 1 {
            drawer.draw_combo(self.current_combo - 1)?;
        }

        if let Some(pieceInfo) = self.hold_piece.as_ref() {
            let can_swap = HoldTetrisPiece::can_swap(&self.hold_piece);
            drawer.draw_hold_piece(pieceInfo, can_swap)?;
        }

        if let Some(pieceInfo) = self.piece.as_ref() {
            // compute position for shadow
            drawer.draw_piece_on_board(pieceInfo)?;

            let shadow_r = self.get_shadow_row_index(&pieceInfo);
            if let Some(shadow_r) = shadow_r {
                drawer.try_draw_shadow(shadow_r, pieceInfo)?;
            }
        }
        graphics::present(ctx)
    }

    fn handle_finalize(&mut self) {
        let piece_with_position = self.piece.as_ref().unwrap();
        piece_with_position.finalize_on(&mut self.board);

        let completed_rows_ranges = self.board.completed_rows();
        let completed_rows = completed_rows_ranges
            .iter()
            .map(|r| (r.0 - r.1) as i32)
            .sum();

        if completed_rows == 0 {
            self.current_combo = 0;
        } else {
            self.current_combo += 1;
        }

        let last = self.last_score.take();

        if piece_with_position.tetris_piece_ref().piece_type == PlayableTetrisPieceType::T {
            // detect T-spin

            if completed_rows > 0 && self.last_move == Moves::ROTATE {
                let center_r = piece_with_position.row() + 1;
                let center_c = piece_with_position.col() + 1;
                let mut occupied = 0;

                debug!("Maybe t-spin detected");

                for i in &[-1, 1] {
                    for j in &[-1, 1] {
                        let ei = center_r + i;
                        let ej = center_c + j;

                        if ei < 0 || ei >= R || ej < 0 || ej >= C || self.board.is_set(ei, ej) {
                            occupied += 1;
                        }
                    }
                }

                debug!("{} corners occupied", occupied);

                if occupied >= 3 {
                    self.last_score = match completed_rows {
                        1 => Some(ScoreType::TSpinSingle),
                        2 => Some(ScoreType::TSpinDouble),
                        3 => Some(ScoreType::TSpinTriple),
                        _ => None,
                    };

                    debug!("Score computed: {:?}", self.last_score);
                }
            }
        } else if completed_rows >= 2 && completed_rows <= 4 {
            self.last_score = match completed_rows {
                2 => Some(ScoreType::Double),
                3 => Some(ScoreType::Triple),
                4 => {
                    debug!("Tetris detected");
                    Some(ScoreType::Tetris)
                }
                _ => unreachable!(),
            }
        }

        let is_b2b = completed_rows > 0
            && last.is_some()
            && self.last_score.is_some()
            && is_b2b_worth(self.last_score.unwrap());
        trace!("B2B detected? {}", is_b2b);

        if is_b2b {
            self.back_to_back += 1;
        } else {
            self.back_to_back = 0;
        }

        if self.back_to_back > 0 {
            debug!("B2B level: {}", self.back_to_back);
        }

        self.board.remove_ranges(completed_rows_ranges);

        if self.board.is_empty() {
            self.back_to_back = 0;
            self.last_score = Some(ScoreType::AllClear);
        }
    }

    pub fn is_paused(&self) -> bool {
        self.pause
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<TetrisUpdateResult> {
        graphics::set_window_title(ctx, &format!("Rust Tetris @ {:.2}fps", timer::fps(ctx)));

        let delta = timer::delta(ctx).as_secs_f64();

        if !self.pause {
            self.advance_frame(delta)
        } else {
            Ok(TetrisUpdateResult::Continue)
        }
    }

    fn advance_frame(&mut self, _dt: f64) -> GameResult<TetrisUpdateResult> {
        let piece = self.piece.as_ref().unwrap();
        let grounded = piece.collides_on_next(&self.board);
        let mut put_next_block = false;

        if self.just_placed && grounded {
            println!("Game over!");
            return Ok(TetrisUpdateResult::GameOver);
        }
        self.just_placed = false;

        if grounded {
            trace!("Lock timer = {}", self.lock_timer);
            if self.lock_timer == self.config.game_params.lock_delay {
                trace!("Reached limit of {}", self.config.game_params.lock_delay);
                self.handle_finalize();
                self.next_block(None);
                self.down_movement_accumulator = 0.0;
                put_next_block = true;
            } else {
                trace!("Limit not reached, increasing lock timer");
                self.lock_timer += 1;
            }
        } else {
            trace!("Not grounded, resetting lock timer");
            self.lock_timer = 0;
        }

        if self.apply_side_move() {
            trace!("Moved to the side, resetting lock timer");
            self.lock_timer = 0;
        }

        if !put_next_block && !grounded {
            trace!("Applying gravity...");
            self.apply_gravity();
        }

        Ok(TetrisUpdateResult::Continue)
    }

    fn apply_side_move(&mut self) -> bool {
        let can_das = (self.frames_for_das as f64) >= self.config.game_params.das;
        let can_single_move = !can_das && self.frames_for_das == 0;
        let sign: i32 = match self.side_move_to_perform {
            Some(SideMoves::LEFT) => -1,
            Some(SideMoves::RIGHT) => 1,
            _ => 0,
        };
        let mut moved = false;

        if sign == 0 {
            self.side_movement_accumulator = 0.0;
        } else {
            self.frames_for_das += 1;
            if can_single_move {
                if self.side_move_signed(sign) {
                    moved = true
                }
            } else if can_das {
                self.side_movement_accumulator += (sign as f64) / self.config.game_params.arr;
                let mut abs = self.side_movement_accumulator.abs();
                let sign = self.side_movement_accumulator.signum();
                if abs >= 1.0 {
                    loop {
                        if self.side_move_signed(sign) {
                            moved = true
                        }

                        abs -= 1.0;
                        if abs < 1.0 {
                            break;
                        }
                    }
                }

                self.side_movement_accumulator = abs * sign;
            }
        }

        moved
    }

    fn side_move_signed<T: Into<f64>>(&mut self, sign: T) -> bool {
        if sign.into() > 0.0 {
            self.move_right()
        } else {
            self.move_left()
        }
    }

    fn apply_gravity(&mut self) {
        self.down_movement_accumulator += self.current_gravity;

        if self.down_movement_accumulator >= 1.0 {
            let piece = self.piece.as_mut().unwrap();

            while self.down_movement_accumulator >= 1.0 {
                if !piece.collides_on_next(&self.board) {
                    piece.move_down();
                    self.last_move = Moves::FALL;
                }
                self.down_movement_accumulator -= 1.0;
            }
        }
    }

    fn reset_drop(&mut self) {
        self.current_gravity = self.config.game_params.gravity;
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused() {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn pause(&mut self) {
        debug!("Pausing...");
        self.pause = true;
    }

    pub fn resume(&mut self) {
        debug!("Resuming...");
        self.pause = false;
    }

    pub fn move_left(&mut self) -> bool {
        let piece = self.piece.as_mut().unwrap();
        if piece.try_move_left(&self.board) {
            self.last_move = Moves::SIDE;
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        let piece = self.piece.as_mut().unwrap();
        if piece.try_move_right(&self.board) {
            self.last_move = Moves::SIDE;
            true
        } else {
            false
        }
    }

    pub fn left_key_pressed(&mut self) {
        self.side_move_to_perform = Some(SideMoves::LEFT);
    }

    pub fn right_key_pressed(&mut self) {
        self.side_move_to_perform = Some(SideMoves::RIGHT);
    }

    pub fn left_key_released(&mut self) {
        self.reset_side_key_pressed();
    }

    pub fn right_key_released(&mut self) {
        self.reset_side_key_pressed();
    }

    fn reset_side_key_pressed(&mut self) {
        self.side_move_to_perform = None;
        self.frames_for_das = 0;
    }

    pub fn rot_pressed(&mut self, next: bool) {
        let piece_with_pos = self.piece.as_mut().unwrap();
        let prev_rot: TetrisPieceRotation;

        let piece_ref = piece_with_pos.tetris_piece_mut();
        prev_rot = piece_ref.rotation;

        if next {
            piece_ref.rotate_piece();
        } else {
            piece_ref.rotate_piece_prev();
        }

        let mut ok = false;
        let mut kick_o = None;

        if let Some(kick) = piece_with_pos.can_rotate(prev_rot, &self.board) {
            piece_with_pos.kick_by(kick);
            if is_not_empty(kick) {
                kick_o = Some(kick);
            }
            ok = true;
        }

        let piece_ref = piece_with_pos.tetris_piece_mut();
        if !ok {
            if !next {
                piece_ref.rotate_piece();
            } else {
                piece_ref.rotate_piece_prev();
            }
        } else {
            self.last_move = Moves::ROTATE;
            self.last_kick = kick_o;
        }
    }

    pub fn next_rot_pressed(&mut self) {
        self.rot_pressed(true);
    }

    pub fn prev_rot_pressed(&mut self) {
        self.rot_pressed(false);
    }

    pub fn hard_drop_key_pressed(&mut self) {
        let piece = self.piece.as_mut().unwrap();

        while !piece.collides_on_next(&self.board) {
            piece.move_down();
        }

        self.handle_finalize();
        self.next_block(None);

        self.last_move = Moves::UP;
    }

    pub fn hold_key_pressed(&mut self) {
        if HoldTetrisPiece::can_swap(&self.hold_piece) {
            let p = self.piece.take();
            let hp = self.hold_piece.take();

            self.hold_piece = Some(HoldTetrisPiece::new(p.unwrap().tetris_piece()));
            if let Some(hp) = hp {
                self.piece = Some(App::build_piece_with_pos(hp.piece));
            }

            if self.piece.is_none() {
                self.next_block(None);
            }

            self.hold_piece.as_mut().unwrap().set_hold();
        }
    }

    pub fn soft_drop_key_pressed(&mut self) {
        self.current_gravity =
            self.config.game_params.gravity * self.config.game_params.soft_drop_factor;
        self.last_move = Moves::DOWN;
    }

    pub fn soft_drop_key_released(&mut self) {
        self.reset_drop();
    }

    pub fn remove_line(&mut self, line: usize) {
        self.board.remove_row(line.try_into().unwrap())
    }

    pub fn set_current(&mut self, p: PlayableTetrisPieceType) {
        self.next_block(Some(p));
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
            PlayableTetrisPieceType::I,
            PlayableTetrisPieceType::S,
            PlayableTetrisPieceType::Z,
            PlayableTetrisPieceType::O,
            PlayableTetrisPieceType::T,
            PlayableTetrisPieceType::L,
            PlayableTetrisPieceType::J,
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

    fn next_block(&mut self, force_piece: Option<PlayableTetrisPieceType>) {
        let piece = match force_piece {
            None => self.buffer_next_pieces.pop_back().unwrap(),
            Some(p) => TetrisPiece::new(p),
        };
        self.piece = Some(App::build_piece_with_pos(piece));
        self.new_block_in_buffer();
        self.reset_drop();
        self.just_placed = true;
        if let Some(hold_piece) = self.hold_piece.as_mut() {
            hold_piece.reset_hold();
        }
    }
}
