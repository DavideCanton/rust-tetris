#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
mod app;
mod board;
mod controller;
mod drawer;
mod pieces;
mod utils;
mod drawables;

use crate::app::*;
use crate::controller::Controller;
use crate::utils::{WIN_H, WIN_W};
use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::window::WindowSettings;
use piston_window::{PistonWindow, Window};

fn configure<W: Window>(win: &mut PistonWindow<W>) {
    win.events.set_max_fps(60);
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new("Tetris", [WIN_W, WIN_H])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Failed to init OpenGL");

    {
        configure(&mut window);
    }

    let controller = Controller::new(window);
    let mut app = App::new(opengl, controller);
    app.start();
}
