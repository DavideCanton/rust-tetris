#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::default::Default;
use std::io::Read;
use std::rc::Rc;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{run, EventsLoop},
    graphics::Font,
    Context, ContextBuilder,
};
use log::{debug, error, info, LevelFilter};

use rust_tetris_ui_core::utils::{WIN_H, WIN_W};

use crate::{
    app::App,
    conf::{GameConfig, Validable},
    controller::Controller,
};

#[macro_use]
mod app;
mod conf;
mod controller;
mod controller_structs;
mod types;

fn init_log() {
    env_logger::builder()
        // set level to trace, let log! configuration from Cargo.toml to guide
        .filter_level(LevelFilter::Trace)
        // disable log flooding of gfx
        .filter_module("gfx_device_gl", LevelFilter::Warn)
        .init();
}

fn init_ggez() -> (Context, EventsLoop) {
    let window_setup = WindowSetup::default().title("Rust Tetris").vsync(true);

    let window_mode = WindowMode::default()
        .dimensions(WIN_W, WIN_H)
        .resizable(false);

    // Make a Context and an EventLoop.
    ContextBuilder::new("rust_tetris", "Davide Canton")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .unwrap()
}

fn main() {
    init_log();

    let (mut ctx, mut event_loop) = init_ggez();
    debug!("Created context");

    let font = Font::new(&mut ctx, "/fonts/FiraCode.ttf").unwrap();

    debug!("Loaded font");

    let conf_str = ggez::filesystem::open(&mut ctx, "/conf/game_conf.toml")
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        })
        .unwrap();
    let config: GameConfig = toml::from_str(&conf_str).expect("Conf load error");
    config.validate();
    debug!("Config: {:?}", config);

    let rc_config = Rc::new(config);

    let app = App::new(font, Rc::clone(&rc_config));
    let mut controller = Controller::new(app, Rc::clone(&rc_config));

    controller.start();

    match run(&mut ctx, &mut event_loop, &mut controller) {
        Ok(_) => info!("Exited cleanly."),
        Err(e) => error!("Error occured: {}", e),
    }
}
