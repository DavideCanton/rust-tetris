use ggez::graphics::Color;

use rust_tetris_core::pieces::{Kick, PlayableTetrisPieceType, TetrisPieceType};

pub const R: isize = 20;
pub const C: isize = 10;

pub const INITIAL_MOVE_DOWN_THRESHOLD: f64 = 1.0;
pub const SPED_UP_THRESHOLD: f64 = 0.05;
pub const WIDTH: f32 = 30.0;
pub const WIN_W: f32 = 800.0;
pub const WIN_H: f32 = 600.0;
pub const BASE_X: f32 = (WIN_W - (WIDTH * 10.0f32)) / 2.0;
pub const HOLD_X: f32 = (BASE_X - (WIDTH * 3.0f32)) / 2.0;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);
pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub const LIGHTBLUE: Color = Color::new(0.0, 0.75, 1.0, 1.0);
pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const ORANGE: Color = Color::new(1.0, 0.6, 0.0, 1.0);
pub const PURPLE: Color = Color::new(1.0, 0.0, 1.0, 1.0);

pub const BGCOLOR: Color = BLACK;
pub const O_COLOR: Color = YELLOW;
pub const I_COLOR: Color = LIGHTBLUE;
pub const S_COLOR: Color = GREEN;
pub const Z_COLOR: Color = RED;
pub const T_COLOR: Color = PURPLE;
pub const L_COLOR: Color = ORANGE;
pub const J_COLOR: Color = BLUE;
pub const OTHER_COLOR: Color = GRAY;

fn apply_shadow(original_color: &Color, is_shadow: bool) -> Color {
    let mut color = original_color.clone();

    if is_shadow {
        color.a = 0.3;
    }

    color
}

pub fn piece_to_color(p: TetrisPieceType, is_shadow: bool) -> Color {
    match p {
        TetrisPieceType::Playable(p) => playable_piece_to_color(p, is_shadow),
        TetrisPieceType::NotPlayable => apply_shadow(&OTHER_COLOR, is_shadow),
    }
}

pub fn playable_piece_to_color(p: PlayableTetrisPieceType, is_shadow: bool) -> Color {
    let original_color = match p {
        PlayableTetrisPieceType::O => O_COLOR,
        PlayableTetrisPieceType::I => I_COLOR,
        PlayableTetrisPieceType::S => S_COLOR,
        PlayableTetrisPieceType::Z => Z_COLOR,
        PlayableTetrisPieceType::T => T_COLOR,
        PlayableTetrisPieceType::L => L_COLOR,
        PlayableTetrisPieceType::J => J_COLOR,
    };

    apply_shadow(&original_color, is_shadow)
}

pub fn is_not_empty(kick: Kick) -> bool {
    kick.0 != 0 || kick.1 != 0
}

#[cfg(test)]
mod tests {}
