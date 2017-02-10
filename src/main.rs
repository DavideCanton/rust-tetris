#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

#[macro_use]
extern crate enum_primitive;
extern crate num;

mod app;
mod board;
mod pieces;
mod utils;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use app::*;


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Tetris", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);
    app.start();

    let mut game_loop = window.events();
    game_loop.set_max_fps(60);    

    while let Some(e) = game_loop.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(p) = e.press_args() {
            app.process_keys(&p);
        }

        if let Some(u) = e.update_args() {
            if !app.is_paused() {
                app.update(&u);
            }
        }
    }
}