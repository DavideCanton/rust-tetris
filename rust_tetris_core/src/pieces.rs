use crate::board::{playable_piece_to_cell, TetrisBoard, TetrisCell};
use crate::constants::{Kick, DEFAULT_KICKS, I_KICKS, NEXT_ROTATIONS, PREV_ROTATIONS};
use crate::enums::{PlayableTetrisPieceType, TetrisPieceRotation};

pub struct TetrisPiece {
    pub piece_type: PlayableTetrisPieceType,
    pub board: TetrisBoard,
    pub rotation: TetrisPieceRotation,
}

impl TetrisPiece {
    pub fn new(piece: PlayableTetrisPieceType) -> Self {
        let mut tetris_piece = TetrisPiece {
            piece_type: piece,
            rotation: TetrisPieceRotation::ZERO,
            board: TetrisBoard::new(0, 0),
        };

        tetris_piece.setup_board();

        tetris_piece
    }

    fn setup_board(&mut self) {
        self.board = get_piece_matrix(self.piece_type, self.rotation);
    }

    pub fn rotate_piece(&mut self) {
        self.rotation = *NEXT_ROTATIONS.get(&self.rotation).unwrap();
        self.setup_board();
    }

    pub fn rotate_piece_prev(&mut self) {
        self.rotation = *PREV_ROTATIONS.get(&self.rotation).unwrap();
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

    pub fn all_cells(&self) -> Box<dyn Iterator<Item = (isize, isize)> + '_> {
        let w = self.width();
        let h = self.height();

        let iter = (0..h).flat_map(move |i| (0..w).map(move |j| (i, j)));
        Box::new(iter)
    }

    pub fn set_cells(&self) -> Box<dyn Iterator<Item = (isize, isize)> + '_> {
        let iter = self
            .all_cells()
            .filter(move |&(i, j)| self.board.is_set(i, j));

        Box::new(iter)
    }

    pub fn collides_left(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        for (i, j) in self.all_cells() {
            if j + col == 0 {
                return false;
            }

            if self.board.is_set(i, j) && matrix.is_set(row + i, j + col - 1) {
                return true;
            }
        }

        false
    }

    pub fn collides_right(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        for (i, j) in self.all_cells() {
            if j + col == self.board.cols - 1 {
                return false;
            }

            if self.board.is_set(i, j) && matrix.is_set(row + i, j + col + 1) {
                return true;
            }
        }

        false
    }

    pub fn collides_kick(&self, row: isize, col: isize, matrix: &TetrisBoard, kick: &Kick) -> bool {
        for (i, j) in self.set_cells() {
            let ei = row + i - kick.1;
            let ej = j + col + kick.0;

            if ei < 0 || ei >= matrix.rows || ej < 0 || ej >= matrix.cols {
                return true;
            }

            if matrix.is_set(ei, ej) {
                return true;
            }
        }

        false
    }

    pub fn collides_on_next(&self, row: isize, col: isize, matrix: &TetrisBoard) -> bool {
        for (i, j) in self.set_cells() {
            if row + i == matrix.rows as isize - 1 {
                return true;
            }

            if matrix.is_set(row + i + 1, j + col) {
                return true;
            }
        }

        false
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

        let kicks: &[Kick] = match self.piece_type {
            PlayableTetrisPieceType::I => &I_KICKS[kick_index],
            PlayableTetrisPieceType::O => &[(0, 0)],
            _ => &DEFAULT_KICKS[kick_index],
        };

        kicks
    }

    pub fn fill_piece_matrix(
        piece: PlayableTetrisPieceType,
        matrix: &mut TetrisBoard,
        rotation: TetrisPieceRotation,
    ) {
        let matrix_str = get_rotations(piece, rotation);

        for (row, row_vec) in matrix_str.split('|').zip(matrix.rows_mut()) {
            for (ch, col) in row.chars().zip(row_vec) {
                let ch = match ch {
                    '0' => None,
                    '1' => Some(piece),
                    _ => panic!(),
                };

                *col = ch
                    .map(playable_piece_to_cell)
                    .unwrap_or(TetrisCell::EmptyCell)
            }
        }
    }
}

fn get_piece_matrix(piece: PlayableTetrisPieceType, rotation: TetrisPieceRotation) -> TetrisBoard {
    let (r, c) = get_piece_size(piece);

    let mut matrix = TetrisBoard::new(r, c);

    TetrisPiece::fill_piece_matrix(piece, &mut matrix, rotation);

    matrix
}

fn get_piece_size(piece: PlayableTetrisPieceType) -> (isize, isize) {
    match piece {
        PlayableTetrisPieceType::I => (4, 4),
        PlayableTetrisPieceType::O => (3, 4),
        _ => (3, 3),
    }
}

fn get_rotations(piece: PlayableTetrisPieceType, rotation: TetrisPieceRotation) -> &'static str {
    match piece {
        PlayableTetrisPieceType::O => get_rotations_o(),
        PlayableTetrisPieceType::I => get_rotations_i(rotation),
        PlayableTetrisPieceType::Z => get_rotations_z(rotation),
        PlayableTetrisPieceType::S => get_rotations_s(rotation),
        PlayableTetrisPieceType::J => get_rotations_j(rotation),
        PlayableTetrisPieceType::L => get_rotations_l(rotation),
        PlayableTetrisPieceType::T => get_rotations_t(rotation),
    }
}

fn get_rotations_o() -> &'static str {
    "0110|0110|0000"
}

fn get_rotations_i(rotation: TetrisPieceRotation) -> &'static str {
    match rotation {
        TetrisPieceRotation::ZERO => "0000|1111|0000|0000",
        TetrisPieceRotation::RIGHT => "0010|0010|0010|0010",
        TetrisPieceRotation::TWO => "0000|0000|1111|0000",
        TetrisPieceRotation::LEFT => "0100|0100|0100|0100",
    }
}

fn get_rotations_z(rotation: TetrisPieceRotation) -> &'static str {
    match rotation {
        TetrisPieceRotation::ZERO => "110|011|000",
        TetrisPieceRotation::RIGHT => "001|011|010",
        TetrisPieceRotation::TWO => "000|110|011",
        TetrisPieceRotation::LEFT => "010|110|100",
    }
}

fn get_rotations_s(rotation: TetrisPieceRotation) -> &'static str {
    match rotation {
        TetrisPieceRotation::ZERO => "011|110|000",
        TetrisPieceRotation::RIGHT => "010|011|001",
        TetrisPieceRotation::TWO => "000|011|110",
        TetrisPieceRotation::LEFT => "100|110|010",
    }
}

fn get_rotations_j(rotation: TetrisPieceRotation) -> &'static str {
    match rotation {
        TetrisPieceRotation::ZERO => "100|111|000",
        TetrisPieceRotation::RIGHT => "011|010|010",
        TetrisPieceRotation::TWO => "000|111|001",
        TetrisPieceRotation::LEFT => "010|010|110",
    }
}

fn get_rotations_l(rotation: TetrisPieceRotation) -> &'static str {
    match rotation {
        TetrisPieceRotation::ZERO => "001|111|000",
        TetrisPieceRotation::RIGHT => "010|010|011",
        TetrisPieceRotation::TWO => "000|111|100",
        TetrisPieceRotation::LEFT => "110|010|010",
    }
}

fn get_rotations_t(rotation: TetrisPieceRotation) -> &'static str {
    match rotation {
        TetrisPieceRotation::ZERO => "010|111|000",
        TetrisPieceRotation::RIGHT => "010|011|010",
        TetrisPieceRotation::TWO => "000|111|010",
        TetrisPieceRotation::LEFT => "010|110|010",
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{
        get_piece_size, PlayableTetrisPieceType, TetrisPiece, TetrisPieceRotation, NEXT_ROTATIONS,
        PREV_ROTATIONS,
    };

    fn next_rotation(r: TetrisPieceRotation) -> TetrisPieceRotation {
        *NEXT_ROTATIONS.get(&r).unwrap()
    }

    fn prev_rotation(r: TetrisPieceRotation) -> TetrisPieceRotation {
        *PREV_ROTATIONS.get(&r).unwrap()
    }

    #[test]
    fn test_get_piece_size() {
        let (w, h) = get_piece_size(PlayableTetrisPieceType::T);
        assert!(w == 3 && h == 3);

        let (w, h) = get_piece_size(PlayableTetrisPieceType::I);
        assert!(w == 4 && h == 4);

        let (w, h) = get_piece_size(PlayableTetrisPieceType::S);
        assert!(w == 3 && h == 3);

        let (w, h) = get_piece_size(PlayableTetrisPieceType::Z);
        assert!(w == 3 && h == 3);

        let (w, h) = get_piece_size(PlayableTetrisPieceType::J);
        assert!(w == 3 && h == 3);

        let (w, h) = get_piece_size(PlayableTetrisPieceType::L);
        assert!(w == 3 && h == 3);

        let (w, h) = get_piece_size(PlayableTetrisPieceType::O);
        assert!(w == 3 && h == 4);
    }

    #[test]
    fn test_next_rotation() {
        let mut rotation = TetrisPieceRotation::ZERO;

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::RIGHT);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::TWO);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::LEFT);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
    }

    #[test]
    fn test_prev_rotation() {
        let mut rotation = TetrisPieceRotation::ZERO;

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
        rotation = prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::LEFT);
        rotation = prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::TWO);
        rotation = prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::RIGHT);
        rotation = prev_rotation(rotation);

        assert_eq!(rotation, TetrisPieceRotation::ZERO);
    }

    #[test]
    fn test_set_cells_T() {
        let t = TetrisPiece::new(PlayableTetrisPieceType::T);
        let cells: Vec<_> = t.set_cells().collect();

        assert_eq!(cells, vec![(0, 1), (1, 0), (1, 1), (1, 2)]);
    }

    #[test]
    fn test_set_cells_I() {
        let t = TetrisPiece::new(PlayableTetrisPieceType::I);
        let cells: Vec<_> = t.set_cells().collect();

        assert_eq!(cells, vec![(1, 0), (1, 1), (1, 2), (1, 3)]);
    }
}
