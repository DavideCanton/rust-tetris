use ggez::input::keyboard::KeyCode;
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
            .map(|c| c.keyboard)
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect::<Vec<_>>();
        let keys_set = keys_vec.iter().collect::<HashSet<_>>();

        if keys_set.len() != keys_vec.len() {
            panic!("Duplicate keys!");
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub keyboard: Option<KeyCode>,
    pub gamepad: Option<String>,
}

impl Validable for KeyConfig {
    fn validate(&self) {
        match (self.keyboard.as_ref(), self.gamepad.as_ref()) {
            (None, None) => panic!("One between keyboard and gamepad should be provided"),
            _ => {}
        }
    }
}
