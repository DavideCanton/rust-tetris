#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use glutin_window::GlutinWindow;
use opengl_graphics::{GlyphCache, OpenGL, TextureSettings};
use piston::{event_loop::EventLoop, window::WindowSettings};
use piston_window::{PistonWindow, Window};
use rust_tetris_ui_core::utils::{WIN_H, WIN_W};

use crate::app::App;
use crate::controller::Controller;

#[macro_use]
mod app;
mod controller;

fn configure<W: Window>(win: &mut PistonWindow<W>) {
    win.events.set_max_fps(60);
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new("Tetris", [WIN_W, WIN_H])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to init OpenGL: {:?}", e));

    let opengl_version = window.device.get_info().version;
    println!("Using version: {:?}", opengl_version);
    configure(&mut window);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font = &assets.join("FiraCode.ttf");

    let glyphs = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

    App::new(opengl, glyphs, Controller::new(window)).start();
}
