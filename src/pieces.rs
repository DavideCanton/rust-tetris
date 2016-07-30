use std::cmp::max;
use num::FromPrimitive;
use board::TetrisBoard;

enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TetrisPiece {
        T,
        L,
        J,
        O,
        I,
        S,
        Z
    }
}

enum_from_primitive! {
#[derive(Debug, Clone, Copy, PartialEq)]
    pub enum PieceRotation {
        UP,
        LEFT,
        DOWN,
        RIGHT
    }
}

pub fn next_rotation(rotation: PieceRotation) -> PieceRotation {
    let mut i = rotation as u8;
    i = (i + 1) % 4;
    PieceRotation::from_u8(i).unwrap()
}

pub fn get_piece_size(piece: TetrisPiece) -> (usize, usize) {
    match piece {
        TetrisPiece::I => (1, 4),
        TetrisPiece::O => (2, 2),
        _ => (2, 3)
    }
}

pub fn get_piece_matrix(piece: TetrisPiece) -> TetrisBoard {
    let (r, c) = get_piece_size(piece);
    let max_size = max(r, c);

    let mut matrix = TetrisBoard::new(max_size, max_size);

    fill_piece_matrix(piece, &mut matrix, PieceRotation::UP);

    matrix
}

pub fn collides_on_next(piece: &TetrisBoard, r: usize, c: usize, matrix: &TetrisBoard) -> bool {
    let w = piece.cols;
    let h = piece.rows;

    for i in 0..h {
        for j in 0..w {
            let pcell = piece.is_set(i, j);
            let mcell = matrix.is_set(r + i, j + c);

            if pcell && mcell {
                panic!("Piece overlapping matrix!");
            }

            if pcell && r + i == matrix.rows - 1 {
                return true;
            }

            let mncell = matrix.is_set(r + i + 1, j + c);

            if pcell && mncell {
                return true;
            }
        }
    }

    false
}

pub fn fill_piece_matrix(piece: TetrisPiece, matrix: &mut TetrisBoard, rotation: PieceRotation) {
    let matrix_str = match piece {
        // O can't rotate
        TetrisPiece::O => get_rotations_O(),

        // I, S, Z have only two rotations
        TetrisPiece::I => get_rotations_I(rotation),

        TetrisPiece::Z => get_rotations_Z(rotation),

        TetrisPiece::S => get_rotations_S(rotation),

        // J,L,T have four rotations
        TetrisPiece::J => get_rotations_J(rotation),

        TetrisPiece::L => get_rotations_L(rotation),

        TetrisPiece::T => get_rotations_T(rotation)
    };

    for (row, row_vec) in matrix_str.split('|').zip(matrix.rows_mut()) {
        for (c, col) in row.chars().zip(row_vec.iter_mut()) {
            let b = match c {
                '0' => None,
                '1' => Some(piece),
                _ => panic!()
            };

            *col = b;
        }
    }
}

fn get_rotations_O() -> &'static str {
    "11|11"
}

fn get_rotations_I(rotation: PieceRotation) -> &'static str {
    match rotation {
        PieceRotation::UP | PieceRotation::DOWN => "0010|0010|0010|0010",
        PieceRotation::LEFT | PieceRotation::RIGHT => "0000|1111|0000|0000",
    }
}

fn get_rotations_Z(rotation: PieceRotation) -> &'static str
{
    match rotation {
        PieceRotation::UP | PieceRotation::DOWN => "010|110|100",
        PieceRotation::LEFT | PieceRotation::RIGHT => "000|110|011",
    }
}

fn get_rotations_S(rotation: PieceRotation) -> &'static str
{
    match rotation {
        PieceRotation::UP | PieceRotation::DOWN => "010|011|001",
        PieceRotation::LEFT | PieceRotation::RIGHT => "000|011|110",
    }
}

fn get_rotations_J(rotation: PieceRotation) -> &'static str {
    match rotation {
        PieceRotation::UP => "010|010|110",
        PieceRotation::LEFT => "000|111|001",
        PieceRotation::DOWN => "011|010|010",
        PieceRotation::RIGHT => "000|100|111"
    }
}

fn get_rotations_L(rotation: PieceRotation) -> &'static str {
    match rotation {
        PieceRotation::UP => "010|010|011",
        PieceRotation::LEFT => "000|001|111",
        PieceRotation::DOWN => "110|010|010",
        PieceRotation::RIGHT => "000|111|100"
    }
}

fn get_rotations_T(rotation: PieceRotation) -> &'static str {
    match rotation {
        PieceRotation::UP => "010|111|000",
        PieceRotation::LEFT => "010|110|010",
        PieceRotation::DOWN => "000|111|010",
        PieceRotation::RIGHT => "010|011|010"
    }
}

#[cfg(test)]
mod tests {
    use super::{get_piece_size, get_piece_matrix, fill_piece_matrix, next_rotation};
    use super::{TetrisPiece, PieceRotation};

    #[test]
    fn test_get_piece_size() {
        let (w, h) = get_piece_size(TetrisPiece::T);

        assert!(w == 2 && h == 3);

        let (w, h) = get_piece_size(TetrisPiece::I);

        assert!(w == 1 && h == 4);

        let (w, h) = get_piece_size(TetrisPiece::S);

        assert!(w == 2 && h == 3);
        let (w, h) = get_piece_size(TetrisPiece::Z);

        assert!(w == 2 && h == 3);

        let (w, h) = get_piece_size(TetrisPiece::J);

        assert!(w == 2 && h == 3);

        let (w, h) = get_piece_size(TetrisPiece::L);

        assert!(w == 2 && h == 3);

        let (w, h) = get_piece_size(TetrisPiece::O);

        assert!(w == 2 && h == 2);
    }

    #[test]
    fn test_next_rotation() {
        let mut rotation = PieceRotation::UP;

        assert_eq!(rotation, PieceRotation::UP);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::LEFT);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::DOWN);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::RIGHT);
        rotation = next_rotation(rotation);

        assert_eq!(rotation, PieceRotation::UP);
    }

    #[test]
    fn test_fill_matrix_O() {
        let mut matrix = get_piece_matrix(TetrisPiece::O);
        let mut rotation = PieceRotation::UP;

        for _ in 0..5 {
            assert!(matrix.len() == 2 && matrix[0].len() == 2);

            rotation = next_rotation(rotation);
            fill_piece_matrix(TetrisPiece::O, &mut matrix, rotation);

            assert!(matrix[0][0].is_some());
            assert!(matrix[0][1].is_some());
            assert!(matrix[1][0].is_some());
            assert!(matrix[1][1].is_some());
        }
    }
}