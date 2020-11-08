use crate::conf::KeyConfig;
use crate::GameConfig;
use std::collections::HashMap;
use std::rc::Rc;

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
    Pause,
    SoftDrop,
    HardDrop,
    Hold,
    Quit,
}

pub struct Controller {
    app: App,
    config: Rc<GameConfig>,
    keys_map: HashMap<KeyCode, ControllerKey>,
}

fn read_key(c: ControllerKey, k: &KeyConfig, map: &mut HashMap<KeyCode, ControllerKey>) {
    match k.keyboard {
        Some(k) => {
            map.insert(k, c);
        }
        None => (),
    }
}

fn create_keys_map(config: &Rc<GameConfig>) -> HashMap<KeyCode, ControllerKey> {
    let mut keys_map = HashMap::new();

    read_key(ControllerKey::Left, &config.keys.left, &mut keys_map);
    read_key(ControllerKey::Right, &config.keys.right, &mut keys_map);
    read_key(
        ControllerKey::NextRotation,
        &config.keys.next_rotation,
        &mut keys_map,
    );
    read_key(
        ControllerKey::PrevRotation,
        &config.keys.prev_rotation,
        &mut keys_map,
    );
    read_key(ControllerKey::Pause, &config.keys.pause, &mut keys_map);
    read_key(
        ControllerKey::SoftDrop,
        &config.keys.soft_drop,
        &mut keys_map,
    );
    read_key(
        ControllerKey::HardDrop,
        &config.keys.hard_drop,
        &mut keys_map,
    );
    read_key(ControllerKey::Hold, &config.keys.hold, &mut keys_map);
    read_key(ControllerKey::Quit, &config.keys.quit, &mut keys_map);

    keys_map
}

impl Controller {
    pub fn new(app: App, config: Rc<GameConfig>) -> Self {
        let keys_map = create_keys_map(&config);
        Controller {
            app,
            config,
            keys_map,
        }
    }

    pub fn start(&mut self) {
        self.app.start();
    }

    fn exec_if_not_paused<F: FnMut(&mut App)>(&mut self, mut ex: F) {
        if !self.app.is_paused() {
            ex(&mut self.app);
        }
    }

    fn decode_key(&self, keycode: &KeyCode) -> Option<&ControllerKey> {
        self.keys_map.get(keycode)
    }

    fn decode_gamepad(&self, _keycode: &KeyCode) -> Option<&ControllerKey> {
        todo!()
    }

    fn handle_key(&mut self, ctx: &mut Context, keycode: KeyCode) {
        match self.decode_key(&keycode) {
            Some(ControllerKey::Pause) => self.app.toggle_pause(),
            Some(ControllerKey::Left) => self.exec_if_not_paused(|app| app.left_key_pressed()),
            Some(ControllerKey::Right) => self.exec_if_not_paused(|app| app.right_key_pressed()),
            Some(ControllerKey::NextRotation) => {
                self.exec_if_not_paused(|app| app.next_rot_pressed())
            }
            Some(ControllerKey::PrevRotation) => {
                self.exec_if_not_paused(|app| app.prev_rot_pressed())
            }
            Some(ControllerKey::SoftDrop) => {
                self.exec_if_not_paused(|app| app.soft_drop_key_pressed())
            }
            Some(ControllerKey::HardDrop) => {
                self.exec_if_not_paused(|app| app.hard_drop_key_pressed())
            }
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
        match self.decode_key(&keycode) {
            Some(ControllerKey::Left) => self.app.left_key_released(),
            Some(ControllerKey::Right) => self.app.right_key_released(),
            Some(ControllerKey::SoftDrop) => self.app.soft_drop_key_released(),
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
