use ggez::{event::Button, input::keyboard::KeyCode};

use serde_derive::Deserialize;
use std::collections::HashSet;

pub trait Validable {
    fn validate(&self);
}

#[derive(Deserialize, Debug)]
pub struct GameConfig {
    pub game_params: GameParamsConfig,
    pub keys: KeysConfig,
}

impl Validable for GameConfig {
    fn validate(&self) {
        self.game_params.validate();
        self.keys.validate();
    }
}

#[derive(Deserialize, Debug)]
pub struct GameParamsConfig {
    pub gravity: f64,
    pub soft_drop_factor: f64,
    pub das: f64,
    pub arr: f64,
    pub lock_delay: u32,
}

impl Validable for GameParamsConfig {
    fn validate(&self) {
        if self.gravity <= 0.0 {
            panic!("Invalid gravity");
        }
        if self.soft_drop_factor <= 0.0 {
            panic!("Invalid soft_drop_factor");
        }
        if self.das <= 0.0 {
            panic!("Invalid das");
        }
        if self.arr <= 0.0 {
            panic!("Invalid arr");
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct KeysConfig {
    pub left: KeyConfig,
    pub right: KeyConfig,
    pub next_rotation: KeyConfig,
    pub prev_rotation: KeyConfig,
    pub pause: KeyConfig,
    pub soft_drop: KeyConfig,
    pub hard_drop: KeyConfig,
    pub hold: KeyConfig,
    pub quit: KeyConfig,
}

impl Validable for KeysConfig {
    fn validate(&self) {
        let children = vec![
            &self.left,
            &self.right,
            &self.next_rotation,
            &self.prev_rotation,
            &self.pause,
            &self.soft_drop,
            &self.hard_drop,
            &self.hold,
            &self.quit,
        ];

        for child in &children {
            child.validate();
        }

        let keys_vec = children
            .iter()
            .flat_map(|c| c.keyboard.as_ref())
            .flat_map(|v| v)
            .collect::<Vec<_>>();
        let keys_set = keys_vec.iter().collect::<HashSet<_>>();

        // TODO detect gamepad duplicate
        // TODO log duplicate key

        if keys_set.len() != keys_vec.len() {
            panic!("Duplicate keys!");
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub keyboard: Option<Vec<KeyCode>>,
    pub gamepad: Option<Vec<Button>>,
}

impl Validable for KeyConfig {
    fn validate(&self) {
        // TODO improve this code
        match (self.keyboard.as_ref(), self.gamepad.as_ref()) {
            (None, None) => panic!("One between keyboard and gamepad should be provided"),
            (Some(v), None) if v.len() == 0 => panic!("No key configured"),
            (None, Some(v2)) if v2.len() == 0 => panic!("No key configured"),
            (Some(v), Some(v2)) if v.len() == 0 || v2.len() == 0 => panic!("No key configured"),
            _ => {}
        }
    }
}
