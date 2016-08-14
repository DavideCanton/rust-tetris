#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

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


const WIDTH: f64 = 30.0;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    board: TetrisBoard,
    r: usize,
    // current piece row
    c: usize,
    // current piece col
    pieceMatrix: Option<TetrisBoard>,
    // piece matrix
    pieceType: Option<TetrisPiece> // piece type
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let board = &self.board;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BGCOLOR, gl);

            for i in 0..board.rows {
                for j in 0..board.cols {
                    let p = board.get(i, j);

                    if let Some(piece) = p {
                        draw_piece_block(i, j, piece, &c, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
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
        pieceType: None,
        pieceMatrix: None
    };

    app.board.set(0, 0, TetrisPiece::S);
    app.board.set(0, 1, TetrisPiece::S);
    app.board.set(1, 1, TetrisPiece::S);
    app.board.set(1, 2, TetrisPiece::S);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}