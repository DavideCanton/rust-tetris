use crate::pieces::TetrisPiece;
use graphics::types::Color;

pub const R: isize = 20;
pub const C: isize = 10;

pub const INITIAL_MOVE_DOWN_THRESHOLD: f64 = 1.0;
pub const SPED_UP_THRESHOLD: f64 = 0.05;
pub const WIDTH: f64 = 30.0;
pub const WIN_W: u32 = 800;
pub const WIN_H: u32 = 600;
pub const BASE_X: u32 = (WIN_W - (WIDTH as u32 * 10)) / 2;
pub const HOLD_X: u32 = (BASE_X - (WIDTH as u32 * 3)) / 2;

pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const GRAY: Color = [0.6, 0.6, 0.6, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const LIGHTBLUE: Color = [0.0, 0.75, 1.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const ORANGE: Color = [1.0, 0.6, 0.0, 1.0];
pub const PURPLE: Color = [1.0, 0.0, 1.0, 1.0];

pub const BGCOLOR: Color = BLACK;
pub const O_COLOR: Color = YELLOW;
pub const I_COLOR: Color = LIGHTBLUE;
pub const S_COLOR: Color = GREEN;
pub const Z_COLOR: Color = RED;
pub const T_COLOR: Color = PURPLE;
pub const L_COLOR: Color = ORANGE;
pub const J_COLOR: Color = BLUE;
pub const OTHER_COLOR: Color = GRAY;

pub fn piece_to_color(p: TetrisPiece, is_shadow: bool) -> Color {
    let original_color = match p {
        TetrisPiece::O => O_COLOR,
        TetrisPiece::I => I_COLOR,
        TetrisPiece::S => S_COLOR,
        TetrisPiece::Z => Z_COLOR,
        TetrisPiece::T => T_COLOR,
        TetrisPiece::L => L_COLOR,
        TetrisPiece::J => J_COLOR,
        TetrisPiece::OTHER => OTHER_COLOR
    };

    let mut color = [0.0; 4];
    color.copy_from_slice(&original_color);

    if is_shadow {
        color[3] = 0.3;
    }

    color
}

#[cfg(test)]
mod tests {}
