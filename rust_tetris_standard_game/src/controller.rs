use std::collections::HashMap;

use ggez::{
    event,
    event::{EventHandler, KeyCode},
    input::keyboard::KeyMods,
    Context, GameResult,
};

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
    user_to_input: HashMap<KeyCode, ControllerKey>,
}

impl Controller {
    pub fn new(app: App) -> Self {
        let mut user_to_input = HashMap::new();
        user_to_input.insert(KeyCode::Left, ControllerKey::Left);
        user_to_input.insert(KeyCode::Right, ControllerKey::Right);
        user_to_input.insert(KeyCode::X, ControllerKey::NextRotation);
        user_to_input.insert(KeyCode::Z, ControllerKey::PrevRotation);
        user_to_input.insert(KeyCode::Return, ControllerKey::Return);
        user_to_input.insert(KeyCode::Down, ControllerKey::Down);
        user_to_input.insert(KeyCode::Up, ControllerKey::Up);
        user_to_input.insert(KeyCode::C, ControllerKey::Hold);
        user_to_input.insert(KeyCode::Escape, ControllerKey::Quit);

        Controller { app, user_to_input }
    }

    pub fn start(&mut self) {
        self.app.start();
    }

    fn exec_if_not_paused<F: FnMut(&mut App)>(&mut self, mut ex: F) {
        if !self.app.is_paused() {
            ex(&mut self.app);
        }
    }

    fn handle_key(&mut self, ctx: &mut Context, keycode: KeyCode) {
        match self.user_to_input.get(&keycode) {
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
}

impl EventHandler for Controller {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.app.update(ctx) {
            Ok(TetrisUpdateResult::GameOver) => {
                panic!("Game over");
            }
            other => other.map(|_| ()),
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
        _repeat: bool,
    ) {
        self.handle_key(ctx, keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match self.user_to_input.get(&keycode) {
            Some(ControllerKey::Left) => self.app.left_key_released(),
            Some(ControllerKey::Right) => self.app.right_key_released(),
            Some(ControllerKey::Down) => self.app.down_key_released(),
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
