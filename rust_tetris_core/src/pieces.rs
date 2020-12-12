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

    pub fn all_cells(&self) -> impl Iterator<Item = (isize, isize)> {
        let w = self.width();
        let h = self.height();

        (0..h).flat_map(move |i| (0..w).map(move |j| (i, j)))
    }

    pub fn set_cells(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.all_cells()
            .filter(move |&(i, j)| self.board.is_set(i, j))
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
            _ => unreachable!(),
        };

        match self.piece_type {
            PlayableTetrisPieceType::I => &I_KICKS[kick_index],
            PlayableTetrisPieceType::O => &[(0, 0)],
            _ => &DEFAULT_KICKS[kick_index],
        }
    }
}

fn fill_piece_matrix(
    piece: PlayableTetrisPieceType,
    matrix: &mut TetrisBoard,
    rotation: TetrisPieceRotation,
) {
    let matrix_bytes = get_rotations(piece, rotation);
    let cols = matrix.cols;

    for (row, row_vec) in matrix_bytes.into_iter().zip(matrix.rows_mut()) {
        let mut acc = 1u8 << (cols - 1);
        for col in row_vec {
            let ch = match row & acc {
                0 => None,
                _ => Some(piece),
            };

            *col = ch
                .map(playable_piece_to_cell)
                .unwrap_or(TetrisCell::EmptyCell);

            acc >>= 1;
        }
    }
}

fn get_piece_matrix(piece: PlayableTetrisPieceType, rotation: TetrisPieceRotation) -> TetrisBoard {
    let (r, c) = get_piece_size(piece);

    let mut matrix = TetrisBoard::new(r, c);

    fill_piece_matrix(piece, &mut matrix, rotation);

    matrix
}

fn get_piece_size(piece: PlayableTetrisPieceType) -> (isize, isize) {
    match piece {
        PlayableTetrisPieceType::I => (4, 4),
        PlayableTetrisPieceType::O => (3, 4),
        _ => (3, 3),
    }
}

fn get_rotations(piece: PlayableTetrisPieceType, rotation: TetrisPieceRotation) -> Vec<u8> {
    piece!(
        piece,
        O => get_rotations_o(rotation),
        I => get_rotations_i(rotation),
        Z => get_rotations_z(rotation),
        S => get_rotations_s(rotation),
        J => get_rotations_j(rotation),
        L => get_rotations_l(rotation),
        T => get_rotations_t(rotation),
    )
}

fn get_rotations_o(rotation: TetrisPieceRotation) -> Vec<u8> {
    same_rotation!(rotation, [6, 6, 0])
}

fn get_rotations_i(rotation: TetrisPieceRotation) -> Vec<u8> {
    rotations!(
        rotation,
        Z => [0, 15, 0, 0],
        R => [2, 2, 2, 2],
        T => [0, 0, 15, 0],
        L => [4, 4, 4, 4]
    )
}

fn get_rotations_z(rotation: TetrisPieceRotation) -> Vec<u8> {
    rotations!(
        rotation,
        Z => [6, 3, 0],
        R => [1, 3, 2],
        T => [0, 6, 3],
        L => [2, 6, 4]
    )
}

fn get_rotations_s(rotation: TetrisPieceRotation) -> Vec<u8> {
    rotations!(
        rotation,
        Z => [3, 6, 0],
        R => [2, 3, 1],
        T => [0, 3, 6],
        L => [4, 6, 2]
    )
}

fn get_rotations_j(rotation: TetrisPieceRotation) -> Vec<u8> {
    rotations!(
        rotation,
        Z => [4, 7, 0],
        R => [3, 2, 2],
        T => [0, 7, 1],
        L => [2, 2, 6]
    )
}

fn get_rotations_l(rotation: TetrisPieceRotation) -> Vec<u8> {
    rotations!(
        rotation,
        Z => [1, 7, 0],
        R => [2, 2, 3],
        T => [0, 7, 4],
        L => [6, 2, 2]
    )
}

fn get_rotations_t(rotation: TetrisPieceRotation) -> Vec<u8> {
    rotations!(
        rotation,
        Z => [2, 7, 0],
        R => [2, 3, 2],
        T => [0, 7, 2],
        L => [2, 6, 2]
    )
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
