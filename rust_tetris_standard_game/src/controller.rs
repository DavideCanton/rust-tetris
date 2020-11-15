use crate::{conf::KeyConfig, controller_structs::ControllerKey, GameConfig};
use ggez::{
    event,
    event::{EventHandler, KeyCode},
    input::keyboard::KeyMods,
    Context, GameResult,
};
use log::debug;
use rust_tetris_core::enums::PlayableTetrisPieceType;
use std::{collections::HashMap, rc::Rc};

use crate::app::App;
use crate::types::TetrisUpdateResult;

type KeysMap = HashMap<CodeWithState, ControllerKey>;

fn read_key(c: ControllerKey, k: &KeyConfig, s: KeyMods, map: &mut KeysMap) {
    match k.keyboard {
        Some(k) => {
            map.insert((k, s), c);
        }
        None => (),
    }
}

fn register_debug_keys(keys_map: &mut KeysMap) {
    fn deserialize_key_code<S: AsRef<str>>(s: S) -> KeyConfig {
        KeyConfig {
            keyboard: Some(serde_plain::from_str::<KeyCode>(s.as_ref()).ok().unwrap()),
            gamepad: None,
        }
    }

    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::O),
        &deserialize_key_code("O"),
        KeyMods::ALT,
        keys_map,
    );
    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::I),
        &deserialize_key_code("I"),
        KeyMods::ALT,
        keys_map,
    );
    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::J),
        &deserialize_key_code("J"),
        KeyMods::ALT,
        keys_map,
    );
    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::L),
        &deserialize_key_code("L"),
        KeyMods::ALT,
        keys_map,
    );
    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::T),
        &deserialize_key_code("T"),
        KeyMods::ALT,
        keys_map,
    );
    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::S),
        &deserialize_key_code("S"),
        KeyMods::ALT,
        keys_map,
    );
    read_key(
        ControllerKey::Choose(PlayableTetrisPieceType::Z),
        &deserialize_key_code("Z"),
        KeyMods::ALT,
        keys_map,
    );

    for i in 0..=9 {
        // ctrl + 1..9 -> 19..10, 0 -> 9
        let l = if i > 0 { 20 - i } else { 9 };
        read_key(
            ControllerKey::RemoveLine(l),
            &deserialize_key_code(format!("Key{}", i)),
            KeyMods::CTRL,
            keys_map,
        );

        // ctrl + shift + 1..9 -> 8..0
        let m = 9 - i;
        read_key(
            ControllerKey::RemoveLine(m),
            &deserialize_key_code(format!("Key{}", i)),
            KeyMods::CTRL | KeyMods::SHIFT,
            keys_map,
        );
    }
}

fn create_keys_map(config: &Rc<GameConfig>) -> KeysMap {
    let mut keys_map = HashMap::new();

    read_key(
        ControllerKey::Left,
        &config.keys.left,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::Right,
        &config.keys.right,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::NextRotation,
        &config.keys.next_rotation,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::PrevRotation,
        &config.keys.prev_rotation,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::Pause,
        &config.keys.pause,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::SoftDrop,
        &config.keys.soft_drop,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::HardDrop,
        &config.keys.hard_drop,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::Hold,
        &config.keys.hold,
        KeyMods::default(),
        &mut keys_map,
    );
    read_key(
        ControllerKey::Quit,
        &config.keys.quit,
        KeyMods::default(),
        &mut keys_map,
    );

    if cfg!(debug_assertions) {
        register_debug_keys(&mut keys_map);
    }

    keys_map
}

type CodeWithState = (KeyCode, KeyMods);

pub struct Controller {
    app: App,
    config: Rc<GameConfig>,
    keys_map: KeysMap,
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

    fn decode_key(&self, keycode: KeyCode, mods: KeyMods) -> Option<ControllerKey> {
        self.keys_map.get(&(keycode, mods)).copied()
    }

    fn decode_gamepad(&self, _keycode: KeyCode) -> Option<&ControllerKey> {
        todo!()
    }

    fn handle_key(&mut self, ctx: &mut Context, keycode: KeyCode, mods: KeyMods) {
        let ctrl_key = self.decode_key(keycode, mods);
        if ctrl_key.is_some() {
            debug!(
                "Got key {:?} with mods {:?} -> {:?}",
                keycode, mods, ctrl_key
            );
        }
        match ctrl_key {
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
            Some(ControllerKey::RemoveLine(n)) => self.app.remove_line(n),
            Some(ControllerKey::Choose(p)) => self.app.set_current(p),
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
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.handle_key(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        match self.decode_key(keycode, keymods) {
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
