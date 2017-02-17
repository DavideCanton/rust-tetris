use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use board::TetrisBoard;
use pieces::*;
use utils::*;
use graphics;
use rand::{Rng, ThreadRng, thread_rng};
use std::collections::VecDeque;
use enum_primitive::FromPrimitive;
use piston_window::{Window, PistonWindow};
use std::rc::Rc;
use std::cell::RefCell;

pub struct App<W: Window> {
    gl: GlGraphics,
    window: Rc<RefCell<PistonWindow<W>>>,
    board: TetrisBoard,
    speed: f64,
    r: isize,
    c: isize,
    pause: bool,
    just_placed: bool,
    piece: Option<PieceInfo>,
    rng: ThreadRng,
    time: f64,
    last_movement: f64,
    removed_rows: i32,
    current_threshold: f64,
    buffer_next_pieces: VecDeque<TetrisPiece>,
}


impl<W: Window> App<W> {
    pub fn new(opengl: OpenGL, window: Rc<RefCell<PistonWindow<W>>>) -> Self {
        App {
            gl: GlGraphics::new(opengl),
            board: TetrisBoard::new(R, C),
            window: window,
            r: 0,
            c: 0,
            speed: 1.0,
            just_placed: false,
            pause: false,
            piece: None,
            rng: thread_rng(),
            time: 0f64,
            removed_rows: 0,
            last_movement: 0f64,
            current_threshold: INITIAL_MOVE_DOWN_THRESHOLD,
            buffer_next_pieces: VecDeque::with_capacity(5),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.pause
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let board = &self.board;
        let pieceMOpt = &self.piece;
        let r = self.r as isize;
        let c = self.c as isize;
        let pause = self.is_paused();

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BGCOLOR, gl);

            for i in 0..board.rows {
                for j in 0..board.cols {
                    let p = board.get(i, j);

                    if let Some(piece) = p {
                        App::<W>::draw_piece_block(i as isize, j as isize, piece, &ctx, gl, false);
                    }
                }
            }

            if let Some(pieceInfo) = pieceMOpt.as_ref() {
                let pieceM = &pieceInfo.board;

                // compute position for shadow
                let mut shadow_r = r;

                while !pieceInfo.collides_on_next(shadow_r, c, board) {
                    shadow_r += 1;
                }

                for i in 0..pieceM.rows {
                    for j in 0..pieceM.cols {
                        let p = pieceM.get(i, j);

                        let i = i as isize;
                        let j = j as isize;

                        if let Some(piece) = p {
                            if j + c >= 0 && j + c < board.cols as isize {
                                App::<W>::draw_piece_block(i + r, j + c, piece, &ctx, gl, false);

                                if !pause {
                                    App::<W>::draw_piece_block(i + shadow_r,
                                                               j + c,
                                                               piece,
                                                               &ctx,
                                                               gl,
                                                               true);
                                }
                            }
                        }
                    }
                }
            }

            if pause {
                // draw pause
                let overlay = rectangle::rectangle_by_corners(0.0, 0.0, 800.0, 600.0);
                rectangle(OVERLAY, overlay, ctx.transform, gl);
            }
        });
    }

    pub fn start(&mut self) {
        self.fill_buffer();
        self.next_block();
    }

    fn handle_finalize(&mut self) {
        let piece = self.piece.as_ref().unwrap();
        self.board.finalize(piece, self.r as isize, self.c as isize);
        let old_removed_rows = self.removed_rows;
        self.removed_rows += self.board.remove_completed_rows(Some(20));
        /*println!("{} {} {} {}",
                 old_removed_rows,
                 self.removed_rows,
                 old_removed_rows / 10,
                 self.removed_rows / 10);*/
        if old_removed_rows / 10 != self.removed_rows / 10 && self.current_threshold > 0.1 {
            println!("Increasing difficulty");
            self.current_threshold -= 0.1;
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt * self.speed;

        if self.just_placed {
            {
                let piece: &PieceInfo = self.piece.as_ref().unwrap();

                if piece.collides_on_next(self.r, self.c, &self.board) {
                    println!("Game over!");
                    self.window.borrow_mut().set_should_close(true);
                }
            }
        }


        if self.time - self.last_movement >= self.current_threshold {

            let mut next_block = false;
            self.just_placed = false;

            {
                let piece: &PieceInfo = self.piece.as_ref().unwrap();

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

        if self.c + last_col < (self.board.cols as isize) - 1 &&
           !piece.collides_right(self.r, self.c, &self.board) {
            self.c += 1;
        }
    }

    fn space_key_pressed(&mut self) {
        let c = self.c as isize;
        let piece = self.piece.as_mut().unwrap();

        piece.rotate_piece();

        let first_col = piece.board.get_first_set_col().unwrap() as isize;
        let last_col = piece.board.get_last_set_col().unwrap() as isize;

        if c + first_col <= 0 {
            self.c -= c + first_col;
        } else if c + last_col >= (self.board.cols as isize) - 1 {
            self.c -= c + last_col - self.board.cols as isize + 1;
        }
    }

    fn up_key_pressed(&mut self) {
        {
            let piece: &PieceInfo = self.piece.as_ref().unwrap();

            while !piece.collides_on_next(self.r, self.c, &self.board) {
                self.r += 1;
            }
        }

        self.handle_finalize();
        self.next_block();
    }

    fn down_key_pressed(&mut self) {
        self.speed = 2.0;
    }

    pub fn process_keys(&mut self, args: &Button) {
        if let Button::Keyboard(Key::Down) = *args {} else {
            self.speed = 1.0;
        }

        if self.pause {
            if let Button::Keyboard(Key::Return) = *args {} else {
                return;
            }
        }

        match *args {
            Button::Keyboard(Key::Left) => self.left_key_pressed(),
            Button::Keyboard(Key::Right) => self.right_key_pressed(),
            Button::Keyboard(Key::Space) => self.space_key_pressed(),
            Button::Keyboard(Key::Return) => self.enter_key_pressed(),
            Button::Keyboard(Key::Down) => self.down_key_pressed(),
            Button::Keyboard(Key::Up) => self.up_key_pressed(),
            _ => {}
        }
    }

    fn new_block_in_buffer(&mut self) {
        let n = self.rng.gen_range(0, 7);
        let piece = TetrisPiece::from_u8(n).unwrap();
        self.buffer_next_pieces.push_front(piece);
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
        self.piece = Some(PieceInfo::new(piece));
        self.new_block_in_buffer();
        self.speed = 1.0;
        self.just_placed = true;
    }

    fn draw_piece_block(i: isize,
                        j: isize,
                        piece: TetrisPiece,
                        c: &graphics::Context,
                        gl: &mut GlGraphics,
                        is_shadow: bool) {
        use graphics::*;

        let i = i as f64;
        let j = j as f64;

        let square = rectangle::square(j * WIDTH, i * WIDTH, WIDTH);
        rectangle(BGCOLOR, square, c.transform, gl);
        let square = rectangle::square(j * WIDTH + 1.0, i * WIDTH + 1.0, WIDTH - 2.0);
        let color = piece_to_color(piece, is_shadow);
        rectangle(color, square, c.transform, gl);
    }
}