use ggez::graphics::Color;

use rust_tetris_core::{
    constants::Kick,
    enums::{PlayableTetrisPieceType, TetrisPieceType},
};

pub const R: isize = 20;
pub const C: isize = 10;

pub const GRAVITY: f64 = 0.016; // 1/60 cells per frame, @60fps it means 1 cell per second
pub const SOFT_DROP_FACTOR: f64 = 40.0;
pub const DAS: f64 = 10.0; // frames after starts autorepeat
pub const ARR: f64 = 2.0; // frames per movement
pub const WIDTH: f32 = 30.0;
pub const WIN_W: f32 = 800.0;
pub const WIN_H: f32 = 600.0;
pub const BASE_X: f32 = (WIN_W - (WIDTH * 10.0f32)) / 2.0;
pub const HOLD_X: f32 = (BASE_X - (WIDTH * 3.0f32)) / 2.0;
pub const GHOST_ALPHA: f32 = 0.3;

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

fn apply_shadow(original_color: Color, is_shadow: bool) -> Color {
    let mut color = original_color;

    if is_shadow {
        color.a = GHOST_ALPHA;
    }

    color
}

pub fn piece_to_color(p: TetrisPieceType, is_shadow: bool) -> Color {
    match p {
        TetrisPieceType::Playable(p) => playable_piece_to_color(p, is_shadow),
        TetrisPieceType::NotPlayable => apply_shadow(OTHER_COLOR, is_shadow),
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

    apply_shadow(original_color, is_shadow)
}

pub fn is_not_empty(kick: Kick) -> bool {
    kick.0 != 0 || kick.1 != 0
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_is_not_empty() {
        let mut kick = (1, 0);
        assert_eq!(is_not_empty(kick), true);

        kick = (0, 1);
        assert_eq!(is_not_empty(kick), true);

        kick = (1, 1);
        assert_eq!(is_not_empty(kick), true);

        kick = (0, 0);
        assert_eq!(is_not_empty(kick), false);
    }

    #[test]
    fn test_playable_piece_to_color_no_shadow() {
        let pairs = vec![
            (PlayableTetrisPieceType::I, I_COLOR),
            (PlayableTetrisPieceType::O, O_COLOR),
            (PlayableTetrisPieceType::S, S_COLOR),
            (PlayableTetrisPieceType::Z, Z_COLOR),
            (PlayableTetrisPieceType::L, L_COLOR),
            (PlayableTetrisPieceType::J, J_COLOR),
            (PlayableTetrisPieceType::T, T_COLOR),
        ];

        for (piece, expected_color) in pairs {
            let color = playable_piece_to_color(piece, false);
            assert_eq!(
                color, expected_color,
                "For piece {:?} color should be {:?}, got {:?}",
                piece, expected_color, color
            );
        }
    }

    #[test]
    fn test_playable_piece_to_color_with_shadow() {
        let mut pairs = vec![
            (PlayableTetrisPieceType::I, I_COLOR),
            (PlayableTetrisPieceType::O, O_COLOR),
            (PlayableTetrisPieceType::S, S_COLOR),
            (PlayableTetrisPieceType::Z, Z_COLOR),
            (PlayableTetrisPieceType::L, L_COLOR),
            (PlayableTetrisPieceType::J, J_COLOR),
            (PlayableTetrisPieceType::T, T_COLOR),
        ];

        for (_, c) in pairs.iter_mut() {
            c.a = GHOST_ALPHA;
        }

        for (piece, expected_color) in pairs {
            let color = playable_piece_to_color(piece, true);
            assert_eq!(
                color, expected_color,
                "For piece {:?} color should be {:?}, got {:?}",
                piece, expected_color, color
            );
        }
    }

    #[test]
    fn test_piece_to_color_no_shadow() {
        let pairs = vec![
            (PlayableTetrisPieceType::I, I_COLOR),
            (PlayableTetrisPieceType::O, O_COLOR),
            (PlayableTetrisPieceType::S, S_COLOR),
            (PlayableTetrisPieceType::Z, Z_COLOR),
            (PlayableTetrisPieceType::L, L_COLOR),
            (PlayableTetrisPieceType::J, J_COLOR),
            (PlayableTetrisPieceType::T, T_COLOR),
        ];

        for (piece, expected_color) in pairs {
            let piece_type = TetrisPieceType::Playable(piece);
            let color = piece_to_color(piece_type, false);
            assert_eq!(
                color, expected_color,
                "For piece {:?} color should be {:?}, got {:?}",
                piece_type, expected_color, color
            );
        }

        let color = piece_to_color(TetrisPieceType::NotPlayable, false);
        assert_eq!(
            color,
            OTHER_COLOR,
            "For piece {:?} color should be {:?}, got {:?}",
            TetrisPieceType::NotPlayable,
            OTHER_COLOR,
            color
        );
    }

    #[test]
    fn test_piece_to_color_with_shadow() {
        let mut pairs = vec![
            (PlayableTetrisPieceType::I, I_COLOR),
            (PlayableTetrisPieceType::O, O_COLOR),
            (PlayableTetrisPieceType::S, S_COLOR),
            (PlayableTetrisPieceType::Z, Z_COLOR),
            (PlayableTetrisPieceType::L, L_COLOR),
            (PlayableTetrisPieceType::J, J_COLOR),
            (PlayableTetrisPieceType::T, T_COLOR),
        ];

        for (_, c) in pairs.iter_mut() {
            c.a = GHOST_ALPHA;
        }

        for (piece, expected_color) in pairs {
            let piece_type = TetrisPieceType::Playable(piece);
            let color = piece_to_color(piece_type, true);
            assert_eq!(
                color, expected_color,
                "For piece {:?} color should be {:?}, got {:?}",
                piece_type, expected_color, color
            );
        }

        let color = piece_to_color(TetrisPieceType::NotPlayable, true);
        let mut expected_color = OTHER_COLOR.clone();
        expected_color.a = GHOST_ALPHA;
        assert_eq!(
            color,
            expected_color,
            "For piece {:?} color should be {:?}, got {:?}",
            TetrisPieceType::NotPlayable,
            expected_color,
            color
        );
    }
}
