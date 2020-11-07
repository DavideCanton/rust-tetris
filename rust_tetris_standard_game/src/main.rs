#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::default::Default;

use ggez::{
    conf::{WindowMode, WindowSetup},
    ContextBuilder,
    event::run,
    graphics::Font,
};

use rust_tetris_ui_core::utils::{WIN_H, WIN_W};

use crate::{app::App, controller::Controller};

#[macro_use]
mod app;
mod controller;
mod types;

fn main() {
    let window_setup = WindowSetup::default().title("Rust Tetris").vsync(true);

    let window_mode = WindowMode::default()
        .dimensions(WIN_W, WIN_H)
        .resizable(false);

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("rust_tetris", "Davide Canton")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .unwrap();

    let font = Font::new(&mut ctx, "/fonts/FiraCode.ttf").unwrap();

    let app = App::new(font);
    let mut controller = Controller::new(app);

    controller.start();

    match run(&mut ctx, &mut event_loop, &mut controller) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
