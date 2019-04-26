use glutin_window::GlutinWindow;
use piston::input::*;
use piston_window::{PistonWindow, Window};

pub enum ControllerKey {
    Left,
    Right,
    NextRotation,
    PrevRotation,
    Return,
    Down,
    Up,
}

pub struct Controller {
    window: PistonWindow<GlutinWindow>,
}

impl Controller {
    pub fn new(window: PistonWindow<GlutinWindow>) -> Self {
        Controller { window }
    }

    pub fn get_key(&self, args: &Button) -> Option<ControllerKey> {
        match *args {
            Button::Keyboard(Key::Left) => Some(ControllerKey::Left),
            Button::Keyboard(Key::Right) => Some(ControllerKey::Right),
            Button::Keyboard(Key::N) => Some(ControllerKey::NextRotation),
            Button::Keyboard(Key::B) => Some(ControllerKey::PrevRotation),
            Button::Keyboard(Key::Return) => Some(ControllerKey::Return),
            Button::Keyboard(Key::Down) => Some(ControllerKey::Down),
            Button::Keyboard(Key::Up) => Some(ControllerKey::Up),
            _ => None,
        }
    }

    pub fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn get_next_event(&mut self) -> Option<Event> {
        self.window.next()
    }
}
