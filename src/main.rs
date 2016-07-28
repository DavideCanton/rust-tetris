#![allow(dead_code)]
#![allow(non_camel_case_types)]
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod board;
mod pieces;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use board::TetrisBoard;

type F32_4 = [f32; 4];
const BLACK: F32_4 = [0.0, 0.0, 0.0, 1.0];
const YELLOW: F32_4 = [1.0, 1.0, 0.0, 1.0];
const WIDTH: f64 = 30.0;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    board: TetrisBoard,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let board = &self.board;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for i in 0..board.rows {
                for j in 0..board.cols {
                    if board.is_set(i, j) {
                        let i = i as f64;
                        let j = j as f64;

                        let square = rectangle::square(i * WIDTH, j * WIDTH, WIDTH);
                        rectangle(YELLOW, square, c.transform, gl);
                    }
                }
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Tetris", [300, 600])
                                 .opengl(opengl)
                                 .exit_on_esc(true)
                                 .build()
                                 .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        board: TetrisBoard::new(20, 10),
    };

    app.board.set(5, 5);

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