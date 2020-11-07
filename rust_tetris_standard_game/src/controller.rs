use crate::types::TetrisUpdateResult;
use ggez::{
    event::{EventHandler, KeyCode},
    input::keyboard::KeyMods,
    Context, GameResult,
};

use crate::app::App;

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
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match self.get_key(keycode) {
            Some(ControllerKey::Return) => self.app.enter_key_pressed(),
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
            _ => {}
        }
    }
}
