use ggez::{
    event,
    event::{EventHandler, KeyCode},
    input::keyboard::KeyMods,
    Context, GameResult,
};
use log::trace;

use crate::app::App;
use crate::types::TetrisUpdateResult;

#[derive(Debug)]
pub enum ControllerKey {
    Left,
    Right,
    NextRotation,
    PrevRotation,
    Return,
    Down,
    Up,
    Hold,
    Quit,
}

pub struct Controller {
    app: App,
}

impl Controller {
    pub fn start(&mut self) {
        self.app.start();
    }

    pub fn new(app: App) -> Self {
        Controller { app }
    }

    pub fn get_key(&self, keycode: KeyCode) -> Option<ControllerKey> {
        match keycode {
            KeyCode::Left => Some(ControllerKey::Left),
            KeyCode::Right => Some(ControllerKey::Right),
            KeyCode::X => Some(ControllerKey::NextRotation),
            KeyCode::Z => Some(ControllerKey::PrevRotation),
            KeyCode::Return => Some(ControllerKey::Return),
            KeyCode::Down => Some(ControllerKey::Down),
            KeyCode::Up => Some(ControllerKey::Up),
            KeyCode::C => Some(ControllerKey::Hold),
            KeyCode::Escape => Some(ControllerKey::Quit),
            _ => None,
        }
    }

    fn exec_if_not_paused<F: FnMut(&mut App)>(&mut self, mut ex: F) {
        if !self.app.is_paused() {
            ex(&mut self.app);
        }
    }
}

impl EventHandler for Controller {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.app.update(ctx) {
            Ok(TetrisUpdateResult::GameOver) => {
                panic!("Game over");
            }
            x => x.map(|_| ()),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.app.render(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        trace!("Key {:?} pressed, repeat: {}", keycode, repeat);
        match self.get_key(keycode) {
            Some(ControllerKey::Return) => self.app.toggle_pause(),
            Some(ControllerKey::Left) => self.exec_if_not_paused(|app| app.left_key_pressed()),
            Some(ControllerKey::Right) => self.exec_if_not_paused(|app| app.right_key_pressed()),
            Some(ControllerKey::NextRotation) => {
                self.exec_if_not_paused(|app| app.next_rot_pressed())
            }
            Some(ControllerKey::PrevRotation) => {
                self.exec_if_not_paused(|app| app.prev_rot_pressed())
            }
            Some(ControllerKey::Down) => self.exec_if_not_paused(|app| app.down_key_pressed()),
            Some(ControllerKey::Up) => self.exec_if_not_paused(|app| app.up_key_pressed()),
            Some(ControllerKey::Hold) => self.exec_if_not_paused(|app| app.hold_key_pressed()),
            Some(ControllerKey::Quit) => event::quit(ctx),
            _ => {}
        }
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            if self.app.is_paused() {
                self.app.resume();
            }
        } else {
            self.app.pause();
        }
    }
}
