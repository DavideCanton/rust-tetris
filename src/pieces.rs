use crate::board::TetrisBoard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrisPiece {
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
pub enum PieceRotation {
    ZERO,
    RIGHT,
    TWO,
    LEFT,
}

static I_KICKS: [[(isize, isize); 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];
static OTHER_KICKS: [[(isize, isize); 5]; 8] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

pub struct TetrisPieceStruct {
    pub piece: TetrisPiece,
    pub board: TetrisBoard,
    pub rotation: PieceRotation,
}

impl TetrisPieceStruct {
    pub fn new(piece: TetrisPiece) -> Self {
        let mut pi = TetrisPieceStruct {
            piece,
            rotation: PieceRotation::ZERO,
            board: TetrisBoard::new(0, 0),
        };

        pi.setup_board();

        pi
    }

    fn setup_board(&mut self) {
        self.board = TetrisPieceStruct::get_piece_matrix(self.piece, self.rotation);
    }

    pub fn rotate_piece(&mut self) {
        self.rotation = TetrisPieceStruct::next_rotation(self.rotation);
        self.setup_board();
    }

    pub fn rotate_piece_prev(&mut self) {
        self.rotation = TetrisPieceStruct::prev_rotation(self.rotation);
        self.setup_board();
    }

    pub fn width(&self) -> isize {
        self.board.cols
    }

    pub fn height(&self) -> isize {
        self.board.rows
    }

    pub fn collides_left(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        let width = self.width();
        let height = self.height();

        for i in 0..height {
            for j in 0..width {
                let i = i as isize;
                let j = j as isize;

                let p_cell = self.board.is_set(i, j);
                let m_cell = matrix.is_set(row + i, j + col);

                if p_cell && m_cell {
                    panic!("Piece overlapping matrix!");
                }

                if j + col == 0 {
                    return false;
                }

                let mn_cell = matrix.is_set(row + i, j + col - 1);

                if p_cell && mn_cell {
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

                let pcell = self.board.is_set(i, j);
                let mcell = matrix.is_set(row + i, j + col);

                if pcell && mcell {
                    panic!("Piece overlapping matrix!");
                }

                if j + col == self.board.cols - 1 {
                    return false;
                }

                let mncell = matrix.is_set(row + i, j + col + 1);

                if pcell && mncell {
                    return true;
                }
            }
        }

        false
    }

    pub fn collides(
        &self,
        row: isize,
        col: isize,
        matrix: &TetrisBoard,
        kick: &(isize, isize),
    ) -> bool {
        let width = self.board.cols;
        let height = self.board.rows;

        for i in 0..height {
            for j in 0..width {
                let i = i as isize;
                let j = j as isize;

                let pcell = self.board.is_set(i, j);

                if !pcell {
                    continue;
                }

                let ei = row + i - kick.1;
                let ej = j + col + kick.0;

                if ei < 0 || ei >= matrix.rows || ej < 0 || ej >= matrix.cols {
                    return true;
                }
                let mcell = matrix.is_set(ei, ej);

                if mcell {
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

                let pcell = self.board.is_set(i, j);
                let mcell = matrix.is_set(row + i, j + col);

                if pcell && mcell {
                    // panic!("Piece overlapping matrix!");
                }

                if pcell && row + i == matrix.rows as isize - 1 {
                    return true;
                }

                let mncell = matrix.is_set(row + i + 1, j + col);

                if pcell && mncell {
                    return true;
                }
            }
        }

        false
    }

    fn get_piece_matrix(piece: TetrisPiece, rotation: PieceRotation) -> TetrisBoard {
        let (r, c) = TetrisPieceStruct::get_piece_size(piece);

        let mut matrix = TetrisBoard::new(r, c);

        TetrisPieceStruct::fill_piece_matrix(piece, &mut matrix, rotation);

        matrix
    }

    fn next_rotation(rotation: PieceRotation) -> PieceRotation {
        match rotation {
            PieceRotation::ZERO => PieceRotation::RIGHT,
            PieceRotation::RIGHT => PieceRotation::TWO,
            PieceRotation::TWO => PieceRotation::LEFT,
            PieceRotation::LEFT => PieceRotation::ZERO,
        }
    }

    fn prev_rotation(rotation: PieceRotation) -> PieceRotation {
        TetrisPieceStruct::next_rotation(
            TetrisPieceStruct::next_rotation(
                TetrisPieceStruct::next_rotation(
                    rotation
                )
            )
        )
    }

    pub fn get_piece_size(piece: TetrisPiece) -> (isize, isize) {
        match piece {
            TetrisPiece::I => (4, 4),
            TetrisPiece::O => (3, 4),
            _ => (3, 3),
        }
    }

    pub fn get_kicks(&self, from_rot: PieceRotation) -> &'static [(isize, isize)] {
        let i = match (from_rot, self.rotation) {
            (PieceRotation::ZERO, PieceRotation::RIGHT) => 0,
            (PieceRotation::RIGHT, PieceRotation::ZERO) => 1,
            (PieceRotation::RIGHT, PieceRotation::TWO) => 2,
            (PieceRotation::TWO, PieceRotation::RIGHT) => 3,
            (PieceRotation::TWO, PieceRotation::LEFT) => 4,
            (PieceRotation::LEFT, PieceRotation::TWO) => 5,
            (PieceRotation::LEFT, PieceRotation::ZERO) => 6,
            (PieceRotation::ZERO, PieceRotation::LEFT) => 7,
            _ => panic!(),
        };

        let kicks: &[(isize, isize)] = match self.piece {
            TetrisPiece::I => &I_KICKS[i],
            TetrisPiece::O => &[(0, 0)],
            _ => &OTHER_KICKS[i],
        };

        kicks
    }

    pub fn fill_piece_matrix(
        piece: TetrisPiece,
        matrix: &mut TetrisBoard,
        rotation: PieceRotation,
    ) {
        let matrix_str = match piece {
            TetrisPiece::O => TetrisPieceStruct::get_rotations_O(),
            TetrisPiece::I => TetrisPieceStruct::get_rotations_I(rotation),
            TetrisPiece::Z => TetrisPieceStruct::get_rotations_Z(rotation),
            TetrisPiece::S => TetrisPieceStruct::get_rotations_S(rotation),
            TetrisPiece::J => TetrisPieceStruct::get_rotations_J(rotation),
            TetrisPiece::L => TetrisPieceStruct::get_rotations_L(rotation),
            TetrisPiece::T => TetrisPieceStruct::get_rotations_T(rotation),
            _ => panic!(),
        };

        for (row, row_vec) in matrix_str.split('|').zip(matrix.rows_mut()) {
            for (c, col) in row.chars().zip(row_vec.iter_mut()) {
                let b = match c {
                    '0' => None,
                    '1' => Some(piece),
                    _ => panic!(),
                };

                *col = b;
            }
        }
    }

    fn get_rotations_O() -> &'static str {
        "0110|0110|0000"
    }

    fn get_rotations_I(rotation: PieceRotation) -> &'static str {
        match rotation {
            PieceRotation::ZERO => "0000|1111|0000|0000",
            PieceRotation::RIGHT => "0010|0010|0010|0010",
            PieceRotation::TWO => "0000|0000|1111|0000",
            PieceRotation::LEFT => "0100|0100|0100|0100",
        }
    }

    fn get_rotations_Z(rotation: PieceRotation) -> &'static str {
        match rotation {
            PieceRotation::ZERO => "110|011|000",
            PieceRotation::RIGHT => "001|011|010",
            PieceRotation::TWO => "000|110|011",
            PieceRotation::LEFT => "010|110|100",
        }
    }

    fn get_rotations_S(rotation: PieceRotation) -> &'static str {
        match rotation {
            PieceRotation::ZERO => "011|110|000",
            PieceRotation::RIGHT => "010|011|001",
            PieceRotation::TWO => "000|011|110",
            PieceRotation::LEFT => "100|110|010",
        }
    }

    fn get_rotations_J(rotation: PieceRotation) -> &'static str {
        match rotation {
            PieceRotation::ZERO => "100|111|000",
            PieceRotation::RIGHT => "011|010|010",
            PieceRotation::TWO => "000|111|001",
            PieceRotation::LEFT => "010|010|110",
        }
    }

    fn get_rotations_L(rotation: PieceRotation) -> &'static str {
        match rotation {
            PieceRotation::ZERO => "001|111|000",
            PieceRotation::RIGHT => "010|010|011",
            PieceRotation::TWO => "000|111|100",
            PieceRotation::LEFT => "110|010|010",
        }
    }

    fn get_rotations_T(rotation: PieceRotation) -> &'static str {
        match rotation {
            PieceRotation::ZERO => "010|111|000",
            PieceRotation::RIGHT => "010|011|010",
            PieceRotation::TWO => "000|111|010",
            PieceRotation::LEFT => "010|110|010",
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{PieceRotation, TetrisPiece, TetrisPieceStruct};

    #[test]
    fn test_get_piece_size() {
        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::T);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::I);

        assert!(w == 4 && h == 4);

        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::S);

        assert!(w == 3 && h == 3);
        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::Z);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::J);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::L);

        assert!(w == 3 && h == 3);

        let (w, h) = TetrisPieceStruct::get_piece_size(TetrisPiece::O);

        assert!(w == 3 && h == 4);
    }

    #[test]
    fn test_next_rotation() {
        let mut rotation = PieceRotation::ZERO;

        assert_eq!(rotation, PieceRotation::ZERO);
        rotation = TetrisPieceStruct::next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::RIGHT);
        rotation = TetrisPieceStruct::next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::TWO);
        rotation = TetrisPieceStruct::next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::LEFT);
        rotation = TetrisPieceStruct::next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::ZERO);
    }

    #[test]
    fn test_prev_rotation() {
        let mut rotation = PieceRotation::ZERO;

        assert_eq!(rotation, PieceRotation::ZERO);
        rotation = TetrisPieceStruct::prev_rotation(rotation);

        assert_eq!(rotation, PieceRotation::LEFT);
        rotation = TetrisPieceStruct::prev_rotation(rotation);

        assert_eq!(rotation, PieceRotation::TWO);
        rotation = TetrisPieceStruct::prev_rotation(rotation);

        assert_eq!(rotation, PieceRotation::RIGHT);
        rotation = TetrisPieceStruct::prev_rotation(rotation);

        assert_eq!(rotation, PieceRotation::ZERO);
    }
}
