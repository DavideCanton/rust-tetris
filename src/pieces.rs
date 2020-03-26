use crate::board::TetrisBoard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrisPieceType {
    T,
    L,
    J,
    O,
    I,
    S,
    Z,
    OTHER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TetrisPieceRotation {
    ZERO,
    RIGHT,
    TWO,
    LEFT,
}

pub type Kick = (isize, isize);

static I_KICKS: [[Kick; 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];
static OTHER_KICKS: [[Kick; 5]; 8] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

pub struct TetrisPiece {
    pub pieceType: TetrisPieceType,
    pub board: TetrisBoard,
    pub rotation: TetrisPieceRotation,
}

impl TetrisPiece {
    pub fn new(piece: TetrisPieceType) -> Self {
        let mut tetris_piece = TetrisPiece {
            pieceType: piece,
            rotation: TetrisPieceRotation::ZERO,
            board: TetrisBoard::new(0, 0),
        };

        tetris_piece.setup_board();

        tetris_piece
    }

    fn setup_board(&mut self) {
        self.board = TetrisPiece::get_piece_matrix(self.pieceType, self.rotation);
    }

    pub fn rotate_piece(&mut self) {
        self.rotation = TetrisPiece::next_rotation(self.rotation);
        self.setup_board();
    }

    pub fn rotate_piece_prev(&mut self) {
        self.rotation = TetrisPiece::prev_rotation(self.rotation);
        self.setup_board();
    }

    pub fn set_rotation(&mut self, rotation: TetrisPieceRotation) {
        self.rotation = rotation;
        self.setup_board();
    }

    pub fn width(&self) -> isize {
        self.board.cols
    }

    pub fn height(&self) -> isize {
        self.board.rows
    }

    pub fn call_on_set_cells<F: FnMut(isize, isize)>(&self, mut f: F) {
        let w = self.width();
        let h = self.height();
        for i in 0..h {
            for j in 0..w {
                if self.board.is_set(i, j) {
                    f(i, j);
                }
            }
        }
    }

    pub fn collides_left(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        let width = self.width();
        let height = self.height();

        for i in 0..height {
            for j in 0..width {
                let i = i as isize;
                let j = j as isize;

                if j + col == 0 {
                    return false;
                }

                if self.board.is_set(i, j) && matrix.is_set(row + i, j + col - 1) {
                    return true;
                }
            }
        }

        false
    }

    pub fn collides_right(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        let width = self.board.cols;
        let height = self.board.rows;

        for i in 0..height {
            for j in 0..width {
                let i = i as isize;
                let j = j as isize;

                if j + col == self.board.cols - 1 {
                    return false;
                }

                if self.board.is_set(i, j) && matrix.is_set(row + i, j + col + 1) {
                    return true;
                }
            }
        }

        false
    }

    pub fn collides_with_kick(&self, row: isize, col: isize, matrix: &TetrisBoard, kick: &Kick) -> bool {
        let width = self.board.cols;
        let height = self.board.rows;

        for i in 0..height {
            for j in 0..width {
                let i = i as isize;
                let j = j as isize;

                if !self.board.is_set(i, j) {
                    continue;
                }

                let ei = row + i - kick.1;
                let ej = j + col + kick.0;

                if ei < 0 || ei >= matrix.rows || ej < 0 || ej >= matrix.cols {
                    return true;
                }

                if matrix.is_set(ei, ej) {
                    return true;
                }
            }
        }

        false
    }

    pub fn collides_on_next(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        let width = self.board.cols;
        let height = self.board.rows;

        for i in 0..height {
            for j in 0..width {
                let i = i as isize;
                let j = j as isize;

                if self.board.is_set(i, j) {
                    if row + i == matrix.rows as isize - 1 {
                        return true;
                    }

                    if matrix.is_set(row + i + 1, j + col) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_piece_matrix(piece: TetrisPieceType, rotation: TetrisPieceRotation) -> TetrisBoard {
        let (r, c) = TetrisPiece::get_piece_size(piece);

        let mut matrix = TetrisBoard::new(r, c);

        TetrisPiece::fill_piece_matrix(piece, &mut matrix, rotation);

        matrix
    }

    fn next_rotation(rotation: TetrisPieceRotation) -> TetrisPieceRotation {
        match rotation {
            TetrisPieceRotation::ZERO => TetrisPieceRotation::RIGHT,
            TetrisPieceRotation::RIGHT => TetrisPieceRotation::TWO,
            TetrisPieceRotation::TWO => TetrisPieceRotation::LEFT,
            TetrisPieceRotation::LEFT => TetrisPieceRotation::ZERO,
        }
    }

    fn prev_rotation(rotation: TetrisPieceRotation) -> TetrisPieceRotation {
        match rotation {
            TetrisPieceRotation::ZERO => TetrisPieceRotation::LEFT,
            TetrisPieceRotation::LEFT => TetrisPieceRotation::TWO,
            TetrisPieceRotation::TWO => TetrisPieceRotation::RIGHT,
            TetrisPieceRotation::RIGHT => TetrisPieceRotation::ZERO,
        }
    }

    pub fn get_piece_size(piece: TetrisPieceType) -> (isize, isize) {
        match piece {
            TetrisPieceType::I => (4, 4),
            TetrisPieceType::O => (3, 4),
            _ => (3, 3),
        }
    }

    pub fn get_kicks(&self, from_rot: TetrisPieceRotation) -> &'static [Kick] {
        let kick_index = match (from_rot, self.rotation) {
            (TetrisPieceRotation::ZERO, TetrisPieceRotation::RIGHT) => 0,
            (TetrisPieceRotation::RIGHT, TetrisPieceRotation::ZERO) => 1,
            (TetrisPieceRotation::RIGHT, TetrisPieceRotation::TWO) => 2,
            (TetrisPieceRotation::TWO, TetrisPieceRotation::RIGHT) => 3,
            (TetrisPieceRotation::TWO, TetrisPieceRotation::LEFT) => 4,
            (TetrisPieceRotation::LEFT, TetrisPieceRotation::TWO) => 5,
            (TetrisPieceRotation::LEFT, TetrisPieceRotation::ZERO) => 6,
            (TetrisPieceRotation::ZERO, TetrisPieceRotation::LEFT) => 7,
            _ => panic!(),
        };

        let kicks: &[Kick] = match self.pieceType {
            TetrisPieceType::I => &I_KICKS[kick_index],
            TetrisPieceType::O => &[(0, 0)],
            _ => &OTHER_KICKS[kick_index],
        };

        kicks
    }

    pub fn fill_piece_matrix(piece: TetrisPieceType, matrix: &mut TetrisBoard, rotation: TetrisPieceRotation) {
        let matrix_str = match piece {
            TetrisPieceType::O => TetrisPiece::get_rotations_O(),
            TetrisPieceType::I => TetrisPiece::get_rotations_I(rotation),
            TetrisPieceType::Z => TetrisPiece::get_rotations_Z(rotation),
            TetrisPieceType::S => TetrisPiece::get_rotations_S(rotation),
            TetrisPieceType::J => TetrisPiece::get_rotations_J(rotation),
            TetrisPieceType::L => TetrisPiece::get_rotations_L(rotation),
            TetrisPieceType::T => TetrisPiece::get_rotations_T(rotation),
            _ => panic!(),
        };

        for (row, row_vec) in matrix_str.split('|').zip(matrix.rows_mut()) {
            for (char, col) in row.chars().zip(row_vec.iter_mut()) {
                let char = match char {
                    '0' => None,
                    '1' => Some(piece),
                    _ => panic!(),
                };

                *col = char;
            }
        }
    }

    fn get_rotations_O() -> &'static str {
        "0110|0110|0000"
    }

    fn get_rotations_I(rotation: TetrisPieceRotation) -> &'static str {
        match rotation {
            TetrisPieceRotation::ZERO => "0000|1111|0000|0000",
            TetrisPieceRotation::RIGHT => "0010|0010|0010|0010",
            TetrisPieceRotation::TWO => "0000|0000|1111|0000",
            TetrisPieceRotation::LEFT => "0100|0100|0100|0100",
        }
    }

    fn get_rotations_Z(rotation: TetrisPieceRotation) -> &'static str {
        match rotation {
            TetrisPieceRotation::ZERO => "110|011|000",
            TetrisPieceRotation::RIGHT => "001|011|010",
            TetrisPieceRotation::TWO => "000|110|011",
            TetrisPieceRotation::LEFT => "010|110|100",
        }
    }

    fn get_rotations_S(rotation: TetrisPieceRotation) -> &'static str {
        match rotation {
            TetrisPieceRotation::ZERO => "011|110|000",
            TetrisPieceRotation::RIGHT => "010|011|001",
            TetrisPieceRotation::TWO => "000|011|110",
            TetrisPieceRotation::LEFT => "100|110|010",
        }
    }

    fn get_rotations_J(rotation: TetrisPieceRotation) -> &'static str {
        match rotation {
            TetrisPieceRotation::ZERO => "100|111|000",
            TetrisPieceRotation::RIGHT => "011|010|010",
            TetrisPieceRotation::TWO => "000|111|001",
            TetrisPieceRotation::LEFT => "010|010|110",
        }
    }

    fn get_rotations_L(rotation: TetrisPieceRotation) -> &'static str {
        match rotation {
            TetrisPieceRotation::ZERO => "001|111|000",
            TetrisPieceRotation::RIGHT => "010|010|011",
            TetrisPieceRotation::TWO => "000|111|100",
            TetrisPieceRotation::LEFT => "110|010|010",
        }
    }

    fn get_rotations_T(rotation: TetrisPieceRotation) -> &'static str {
        match rotation {
            TetrisPieceRotation::ZERO => "010|111|000",
            TetrisPieceRotation::RIGHT => "010|011|010",
            TetrisPieceRotation::TWO => "000|111|010",
            TetrisPieceRotation::LEFT => "010|110|010",
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{TetrisPiece, TetrisPieceRotation, TetrisPieceType};

    #[test]
    fn test_get_piece_size() {
        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::T);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::I);

        assert!(w == 4 && h == 4);

        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::S);

        assert!(w == 3 && h == 3);
        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::Z);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::J);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::L);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPiece::get_piece_size(TetrisPieceType::O);

        assert!(w == 3 && h == 4);
    }

    #[test]
    fn test_next_rotation() {
        let mut rotation = TetrisPieceRotation::ZERO;

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
        rotation = TetrisPiece::next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::RIGHT);
        rotation = TetrisPiece::next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::TWO);
        rotation = TetrisPiece::next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::LEFT);
        rotation = TetrisPiece::next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
    }

    #[test]
    fn test_prev_rotation() {
        let mut rotation = TetrisPieceRotation::ZERO;

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
        rotation = TetrisPiece::prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::LEFT);
        rotation = TetrisPiece::prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::TWO);
        rotation = TetrisPiece::prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::RIGHT);
        rotation = TetrisPiece::prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
    }
}
