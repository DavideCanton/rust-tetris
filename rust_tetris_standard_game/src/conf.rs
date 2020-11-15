use ggez::{event::Button, input::keyboard::KeyCode};

use serde_derive::Deserialize;
use std::collections::HashSet;

type ValidationResult = Result<(), String>;

pub trait Validable {
    fn validate(&self) -> ValidationResult;
}

#[derive(Deserialize, Debug)]
pub struct GameConfig {
    pub game_params: GameParamsConfig,
    pub keys: KeysConfig,
}

impl Validable for GameConfig {
    fn validate(&self) -> ValidationResult {
        self.game_params.validate()?;
        self.keys.validate()?;
        Ok(())
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
    fn validate(&self) -> ValidationResult {
        if self.gravity <= 0.0 {
            Err(String::from("Invalid gravity"))
        } else if self.soft_drop_factor <= 0.0 {
            Err(String::from("Invalid soft_drop_factor"))
        } else if self.das <= 0.0 {
            Err(String::from("Invalid das"))
        } else if self.arr <= 0.0 {
            Err(String::from("Invalid arr"))
        } else {
            Ok(())
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
    fn validate(&self) -> ValidationResult {
        let children = vec![
            ("left", &self.left),
            ("right", &self.right),
            ("next_rotation", &self.next_rotation),
            ("prev_rotation", &self.prev_rotation),
            ("pause", &self.pause),
            ("soft_drop", &self.soft_drop),
            ("hard_drop", &self.hard_drop),
            ("hold", &self.hold),
            ("quit", &self.quit),
        ];

        for (name, child) in &children {
            child.validate().map_err(|s| format!("{}: {}", name, s))?;
        }

        let keys_vec = children
            .iter()
            .flat_map(|(_, c)| c.keyboard.as_ref())
            .flat_map(|v| v)
            .collect::<Vec<_>>();
        let keys_set = keys_vec.iter().collect::<HashSet<_>>();

        let gamepad_vec = children
            .iter()
            .flat_map(|(_, c)| c.gamepad.as_ref())
            .flat_map(|v| v)
            .collect::<Vec<_>>();
        let gamepad_set = gamepad_vec.iter().collect::<HashSet<_>>();

        // TODO log duplicate key

        if keys_set.len() != keys_vec.len() {
            Err(String::from("Duplicate keys!"))
        } else if gamepad_set.len() != gamepad_vec.len() {
            Err(String::from("Duplicate pad!"))
        } else {
            Ok(())
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub keyboard: Option<Vec<KeyCode>>,
    pub gamepad: Option<Vec<Button>>,
}

impl Validable for KeyConfig {
    fn validate(&self) -> ValidationResult {
        fn get_size<T>(o: &Option<Vec<T>>) -> usize {
            o.as_ref().map(|v| v.len()).unwrap_or(0)
        }

        let kb_len = get_size(&self.keyboard);
        let gpad_len = get_size(&self.gamepad);

        match (kb_len, gpad_len) {
            (0, 0) => Err(String::from(
                "One between keyboard and gamepad should be provided",
            )),
            _ => Ok(()),
        }
    }
}
