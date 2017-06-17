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

mod app;
mod board;
mod pieces;
mod utils;

use piston::window::WindowSettings;
use piston_window::{Window, PistonWindow};
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::OpenGL;
use glutin_window::GlutinWindow;
use app::*;
use std::rc::Rc;
use std::cell::RefCell;
use utils::{WIN_H, WIN_W};


fn configure<W: Window>(win: &mut Rc<RefCell<PistonWindow<W>>>) {
    let mut win = win.borrow_mut();
    win.events.set_max_fps(60);
}

fn main() {
    let opengl = OpenGL::V3_2;

    let window: PistonWindow<GlutinWindow> = WindowSettings::new("Tetris", [WIN_W, WIN_H])
                                                 .opengl(opengl)
                                                 .exit_on_esc(true)
                                                 .build()
                                                 .unwrap();

    let mut rcw = Rc::new(RefCell::new(window));

    let mut app = App::new(opengl, rcw.clone());
    app.start();

    {
        configure(&mut rcw);
    }

    loop {
        let e;
        {
            match rcw.borrow_mut().next() {
                None => return,
                Some(s) => e = s,
            };
        }

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