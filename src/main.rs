#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

#[macro_use] extern crate enum_primitive;
extern crate num;

mod board;
mod pieces;
mod utils;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use board::TetrisBoard;
use pieces::*;
use utils::*;
use rand::Rng;
use enum_primitive::FromPrimitive;

const WIDTH: f64 = 30.0;

pub struct App {
    // OpenGL drawing backend.
    gl: GlGraphics,
    board: TetrisBoard,
    // current piece row
    r: usize,
    // current piece col
    c: usize,
    // piece matrix
    piece: Option<PieceInfo>,
    rng: rand::ThreadRng,
    time: f64,
    last_movement: f64
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let board = &self.board;
        let pieceMOpt = &self.piece;
        let r = self.r;
        let c = self.c;

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BGCOLOR, gl);

            for i in 0..board.rows {
                for j in 0..board.cols {
                    let p = board.get(i, j);

                    if let Some(piece) = p {
                        draw_piece_block(i, j, piece, &ctx, gl);
                    }
                }
            }

            if let Some(pieceInfo) = pieceMOpt.as_ref() {
                let pieceM = &pieceInfo.board;

                for i in 0..pieceM.rows {
                    for j in 0..pieceM.cols {
                        let p = pieceM.get(i, j);

                        if let Some(piece) = p {
                            let v = j.wrapping_add(c);

                            if v >= 0 && v < board.cols {
                                draw_piece_block(i + r, v, piece, &ctx, gl);
                            }
                        }
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;

        if self.time - self.last_movement >= 1.0 {
            self.r += 1;
            self.last_movement = self.time;
        }
    }

    fn process_keys(&mut self, args: &Button) {
        match *args {
            Button::Keyboard(Key::Left) => self.c = self.c.wrapping_sub(1),
            Button::Keyboard(Key::Right) => self.c = self.c.wrapping_add(1),
            Button::Keyboard(Key::Space) => self.piece.as_mut().unwrap().rotate_piece(),
            _ => {}
        }
    }

    fn spawn_random_block(&mut self) {
        let n = self.rng.gen_range(0, 7);
        let piece = TetrisPiece::from_u8(n).unwrap();

        self.piece = Some(PieceInfo::new(piece));
    }
}


fn draw_piece_block(i: usize, j: usize, piece: TetrisPiece, c: &graphics::Context, gl: &mut GlGraphics) {
    use graphics::*;

    let i = i as f64;
    let j = j as f64;

    let square = rectangle::square(j * WIDTH, i * WIDTH, WIDTH);
    rectangle(BGCOLOR, square, c.transform, gl);
    let square = rectangle::square(j * WIDTH + 1.0, i * WIDTH + 1.0, WIDTH - 2.0);
    let color = piece_to_color(piece);
    rectangle(color, square, c.transform, gl);
}


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Tetris", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        board: TetrisBoard::new(20, 10),
        r: 0,
        c: 0,
        piece: None,
        rng: rand::thread_rng(),
        time: 0f64,
        last_movement: 0f64
    };

    app.spawn_random_block();

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(p) = e.press_args() {
            app.process_keys(&p);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}