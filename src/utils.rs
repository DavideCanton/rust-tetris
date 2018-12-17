use pieces::TetrisPiece;

pub type F32_4 = [f32; 4];

pub const R: isize = 20;
pub const C: isize = 10;

pub const INITIAL_MOVE_DOWN_THRESHOLD: f64 = 0.5;
pub const SPED_UP_THRESHOLD: f64 = 0.05;
pub const WIDTH: f64 = 30.0;
pub const WIN_W: u32 = 800;
pub const WIN_H: u32 = 600;
pub const BASE_X: u32 = (WIN_W - (WIDTH as u32 * 10)) / 2;

pub const BLACK: F32_4 = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: F32_4 = [1.0, 1.0, 0.0, 1.0];
pub const RED: F32_4 = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: F32_4 = [0.0, 0.0, 1.0, 1.0];
pub const LIGHTBLUE: F32_4 = [0.0, 0.75, 1.0, 1.0];
pub const GREEN: F32_4 = [0.0, 1.0, 0.0, 1.0];
pub const ORANGE: F32_4 = [1.0, 0.6, 0.0, 1.0];
pub const PURPLE: F32_4 = [1.0, 0.0, 1.0, 1.0];
pub const OVERLAY: F32_4 = [0.0, 0.0, 0.0, 0.8];

pub const BGCOLOR: F32_4 = BLACK;
pub const O_COLOR: F32_4 = YELLOW;
pub const I_COLOR: F32_4 = LIGHTBLUE;
pub const S_COLOR: F32_4 = RED;
pub const Z_COLOR: F32_4 = GREEN;
pub const T_COLOR: F32_4 = PURPLE;
pub const L_COLOR: F32_4 = BLUE;
pub const J_COLOR: F32_4 = ORANGE;

pub fn piece_to_color(p: &TetrisPiece, is_shadow: bool) -> F32_4 {
    let original_color = match *p {
        TetrisPiece::O => O_COLOR,
        TetrisPiece::I => I_COLOR,
        TetrisPiece::S => S_COLOR,
        TetrisPiece::Z => Z_COLOR,
        TetrisPiece::T => T_COLOR,
        TetrisPiece::L => L_COLOR,
        TetrisPiece::J => J_COLOR,
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
