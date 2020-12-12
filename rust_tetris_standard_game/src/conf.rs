use ggez::{event::Button, input::keyboard::KeyCode};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

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
        let mut errs = vec![];
        if self.gravity <= 0.0 {
            errs.push(Err(String::from("invalid gravity")));
        }
        if self.soft_drop_factor <= 0.0 {
            errs.push(Err(String::from("invalid soft_drop_factor")));
        }
        if self.das <= 0.0 {
            errs.push(Err(String::from("invalid das")));
        }
        if self.arr <= 0.0 {
            errs.push(Err(String::from("invalid arr")));
        }
        join_results_array(errs)
    }
}

type ConfigWithName<'a, 'b> = (&'a str, &'b KeyConfig);

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

        let mut results = vec![];

        for (name, child) in &children {
            results.push(child.validate().map_err(|s| format!("{}: {}", name, s)));
        }

        let duplicated_keys = get_duplicated_entries(&children, |c| &c.keyboard);
        let duplicated_btns = get_duplicated_entries(&children, |c| &c.gamepad);

        results.extend(get_messages(duplicated_keys));
        results.extend(get_messages(duplicated_btns));

        join_results_array(results)
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
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

fn join_results(r1: ValidationResult, r2: ValidationResult) -> ValidationResult {
    match (r1, r2) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(m1), Err(m2)) => join_messages(vec![m1, m2]),
        (e @ Err(_), _) => e,
        (_, e @ Err(_)) => e,
    }
}

fn join_results_array(v: Vec<ValidationResult>) -> ValidationResult {
    v.into_iter().fold(Ok(()), join_results)
}

fn join_messages<S: Into<String>>(s: Vec<S>) -> ValidationResult {
    Err(s
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>()
        .join(";"))
}

fn get_messages<T: Debug>(map: HashMap<&T, Vec<ConfigWithName>>) -> Vec<ValidationResult> {
    map.into_iter()
        .map(|(k, v)| {
            let names = v.iter().map(|&(n, _)| n).collect::<Vec<_>>();
            Err(format!(
                "Binding for {:?} duplicated in entries: {:?}",
                k, names
            ))
        })
        .collect()
}

fn get_duplicated_entries<'b, 'c, T: Hash + Eq, F: Fn(&KeyConfig) -> &Option<Vec<T>>>(
    children: &[ConfigWithName<'b, 'c>],
    getter: F,
) -> HashMap<&'c T, Vec<ConfigWithName<'b, 'c>>> {
    // iterator of pairs (key, keyconfig in file)
    let iter = children.iter().flat_map(|&(k, c)| {
        getter(c)
            .iter()
            .flat_map(|cc| cc.iter().map(|x| (x, (k, c))))
            .collect::<Vec<_>>()
    });

    let mut map = HashMap::<&T, Vec<ConfigWithName>>::new();

    for (k, v) in iter {
        map.entry(k)
            .and_modify(|e| e.push(v))
            .or_insert_with(|| vec![v]);
    }

    map.retain(|_, v| v.len() > 1);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_valid_keyconfig_just_kb() -> KeyConfig {
        KeyConfig {
            keyboard: Some(vec![KeyCode::A]),
            gamepad: None,
        }
    }

    fn get_valid_keyconfig_just_gp() -> KeyConfig {
        KeyConfig {
            keyboard: None,
            gamepad: Some(vec![Button::DPadDown]),
        }
    }

    fn get_valid_keyconfig_both() -> KeyConfig {
        KeyConfig {
            keyboard: Some(vec![KeyCode::A]),
            gamepad: Some(vec![Button::DPadDown]),
        }
    }

    #[test]
    fn test_valid_keyconfig_keyboard() {
        let conf = get_valid_keyconfig_just_kb();
        let res = conf.validate();
        assert!(res.is_ok(), "Invalid configuration: {:?}", res);
    }

    #[test]
    fn test_valid_keyconfig_gamepad() {
        let conf = get_valid_keyconfig_just_gp();
        let res = conf.validate();
        assert!(res.is_ok(), "Invalid configuration: {:?}", res);
    }

    #[test]
    fn test_valid_keyconfig_both() {
        let conf = get_valid_keyconfig_both();
        let res = conf.validate();
        assert!(res.is_ok(), "Invalid configuration: {:?}", res);
    }

    #[test]
    fn test_valid_keyconfig_empty_pad() {
        let conf = KeyConfig {
            keyboard: Some(vec![KeyCode::A]),
            gamepad: Some(vec![]),
        };
        let res = conf.validate();
        assert!(res.is_ok(), "Invalid configuration: {:?}", res);
    }

    #[test]
    fn test_valid_keyconfig_empty_keboard() {
        let conf = KeyConfig {
            keyboard: Some(vec![]),
            gamepad: Some(vec![Button::DPadDown]),
        };
        let res = conf.validate();
        assert!(res.is_ok(), "Invalid configuration: {:?}", res);
    }

    #[test]
    fn test_invalid_keyconfig_both_empty() {
        let conf = KeyConfig {
            keyboard: Some(vec![]),
            gamepad: Some(vec![]),
        };
        let res = conf.validate();
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "One between keyboard and gamepad should be provided"
        );
    }

    #[test]
    fn test_invalid_keyconfig_both_none() {
        let conf = KeyConfig {
            keyboard: None,
            gamepad: None,
        };
        let res = conf.validate();
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "One between keyboard and gamepad should be provided"
        );
    }

    #[test]
    fn test_valid_gameparams() {
        let conf = GameParamsConfig {
            arr: 1.0,
            das: 1.0,
            gravity: 1.0,
            lock_delay: 1,
            soft_drop_factor: 1.0,
        };
        let res = conf.validate();
        assert!(res.is_ok(), "Invalid configuration: {:?}", res);
    }

    #[test]
    fn test_invalid_gameparams() {
        let conf = GameParamsConfig {
            arr: -1.0,
            das: -1.0,
            gravity: -1.0,
            lock_delay: 1,
            soft_drop_factor: -1.0,
        };
        let res = conf.validate();
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "invalid gravity;invalid soft_drop_factor;invalid das;invalid arr"
        );
    }
}
