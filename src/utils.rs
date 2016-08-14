use pieces::TetrisPiece;

pub type F32_4 = [f32; 4];

pub const BLACK: F32_4 = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: F32_4 = [1.0, 1.0, 0.0, 1.0];
pub const RED: F32_4 = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: F32_4 = [0.0, 0.0, 1.0, 1.0];
pub const LIGHTBLUE: F32_4 = [0.0, 0.0, 0.6, 1.0]; // TODO
pub const GREEN: F32_4 = [0.0, 1.0, 0.0, 1.0];
pub const ORANGE: F32_4 = [1.0, 0.6, 0.0, 1.0];
pub const PURPLE: F32_4 = [1.0, 0.0, 1.0, 1.0];

pub const BGCOLOR : F32_4 = BLACK;
pub const O_COLOR : F32_4 = YELLOW;
pub const I_COLOR : F32_4 = LIGHTBLUE;
pub const S_COLOR : F32_4 = RED;
pub const Z_COLOR : F32_4 = GREEN;
pub const T_COLOR : F32_4 = PURPLE;
pub const L_COLOR : F32_4 = BLUE;
pub const J_COLOR : F32_4 = ORANGE;

pub fn piece_to_color(p: TetrisPiece) -> F32_4 {
    match p {
        TetrisPiece::O => O_COLOR,
        TetrisPiece::I => I_COLOR,
        TetrisPiece::S => S_COLOR,
        TetrisPiece::Z => Z_COLOR,
        TetrisPiece::T => T_COLOR,
        TetrisPiece::L => L_COLOR,
        TetrisPiece::J => J_COLOR,
    }
}