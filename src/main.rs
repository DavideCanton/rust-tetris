#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

#[macro_use]
extern crate enum_primitive;
extern crate num;

#[macro_use]
mod app;
mod board;
mod pieces;
mod utils;
mod controller;
mod drawer;

use piston::window::WindowSettings;
use piston_window::{Window, PistonWindow};
use piston::event_loop::*;
use opengl_graphics::OpenGL;
use glutin_window::GlutinWindow;
use app::*;
use utils::{WIN_H, WIN_W};
use controller::Controller;

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
