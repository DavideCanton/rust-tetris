#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
mod app;
mod board;
mod controller;
mod drawables;
mod drawer;
mod pieces;
mod utils;

use crate::app::App;
use crate::controller::Controller;
use crate::utils::{WIN_H, WIN_W};
use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::{event_loop::EventLoop, window::WindowSettings};
use piston_window::{PistonWindow, Window};

fn configure<W: Window>(win: &mut PistonWindow<W>) {
    win.events.set_max_fps(60);
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new("Tetris", [WIN_W, WIN_H])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect(&format!("Failed to init OpenGL {:?}", opengl));

    configure(&mut window);

    let controller = Controller::new(window);
    let mut app = App::new(opengl, controller);
    app.start();
}
