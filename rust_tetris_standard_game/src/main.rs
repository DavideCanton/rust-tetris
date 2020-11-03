#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
mod app;
mod app_structs;
mod controller;
mod drawer;
mod utils;

use crate::app::App;
use crate::controller::Controller;
use crate::utils::{WIN_H, WIN_W};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlyphCache, OpenGL, TextureSettings};
use piston::{event_loop::EventLoop, window::WindowSettings};
use piston_window::{PistonWindow, Window};

fn configure<W: Window>(win: &mut PistonWindow<W>) {
    win.events.set_max_fps(60);
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new("Tetris", [WIN_W, WIN_H])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_| panic!("Failed to init OpenGL {:?}", opengl));

    configure(&mut window);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font = &assets.join("FiraCode.ttf");

    let glyphs = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

    App::new(opengl, glyphs, Controller::new(window)).start();
}
