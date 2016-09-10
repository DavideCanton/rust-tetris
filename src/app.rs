use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use board::TetrisBoard;
use pieces::*;
use utils::*;
use graphics;
use rand::{Rng, ThreadRng, thread_rng};
use std::collections::VecDeque;
use enum_primitive::FromPrimitive;

pub struct App {
    gl: GlGraphics,
    board: TetrisBoard,
    r: isize,
    c: isize,
    piece: Option<PieceInfo>,
    rng: ThreadRng,
    time: f64,
    last_movement: f64,
    buffer_next_pieces: VecDeque<TetrisPiece>,
}


impl App {
    pub fn new(opengl: OpenGL) -> Self {
        App {
            gl: GlGraphics::new(opengl),
            board: TetrisBoard::new(R, C),
            r: 0,
            c: 0,
            piece: None,
            rng: thread_rng(),
            time: 0f64,
            last_movement: 0f64,
            buffer_next_pieces: VecDeque::with_capacity(5),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let board = &self.board;
        let pieceMOpt = &self.piece;
        let r = self.r as isize;
        let c = self.c as isize;

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BGCOLOR, gl);

            for i in 0..board.rows {
                for j in 0..board.cols {
                    let p = board.get(i, j);

                    if let Some(piece) = p {
                        App::draw_piece_block(i as isize, j as isize, piece, &ctx, gl);
                    }
                }
            }

            if let Some(pieceInfo) = pieceMOpt.as_ref() {
                let pieceM = &pieceInfo.board;

                for i in 0..pieceM.rows {
                    for j in 0..pieceM.cols {
                        let p = pieceM.get(i, j);

                        let i = i as isize;
                        let j = j as isize;

                        if let Some(piece) = p {
                            let v = j + c;

                            if v >= 0 && v < board.cols as isize {
                                App::draw_piece_block(i + r, v, piece, &ctx, gl);
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn start(&mut self) {
        self.fill_buffer();
        self.next_block();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;

        if self.time - self.last_movement >= MOVE_DOWN_THRESHOLD {
            if self.piece.as_ref().unwrap().collides_on_next(self.r, self.c, &self.board) {
                self.board.finalize(self.piece.as_ref().unwrap(),
                                    self.r as isize,
                                    self.c as isize);

                self.next_block();
            } else {
                self.r += 1;
                self.last_movement = self.time;
            }
        }
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
            self.c += -(c + first_col);
        } else if c + last_col >= (self.board.cols as isize) - 1 {
            self.c -= c + last_col - self.board.cols as isize + 1;
        }
    }

    fn down_key_pressed(&mut self) {}

    pub fn process_keys(&mut self, args: &Button) {
        match *args {
            Button::Keyboard(Key::Left) => self.left_key_pressed(),
            Button::Keyboard(Key::Right) => self.right_key_pressed(),
            Button::Keyboard(Key::Space) => self.space_key_pressed(),
            Button::Keyboard(Key::Down) => self.down_key_pressed(),
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
    }

    fn draw_piece_block(i: isize,
                        j: isize,
                        piece: TetrisPiece,
                        c: &graphics::Context,
                        gl: &mut GlGraphics) {
        use graphics::*;

        let i = i as f64;
        let j = j as f64;

        let square = rectangle::square(j * WIDTH, i * WIDTH, WIDTH);
        rectangle(BGCOLOR, square, c.transform, gl);
        let square = rectangle::square(j * WIDTH + 1.0, i * WIDTH + 1.0, WIDTH - 2.0);
        let color = piece_to_color(piece);
        rectangle(color, square, c.transform, gl);
    }
}