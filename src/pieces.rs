use std::vec::Vec;
use std::cmp::max;
use num::FromPrimitive;

pub type PieceMatrix = Vec<Vec<bool>>;

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

pub fn get_piece_matrix(piece: TetrisPiece) -> PieceMatrix {
    let (r, c) = get_piece_size(piece);
    let max_size = max(r, c);

    let mut matrix = Vec::with_capacity(max_size);

    for _ in 0..max_size {
        matrix.push(vec![false; max_size]);
    }

    fill_piece_matrix(piece, &mut matrix, PieceRotation::UP);

    matrix
}

pub fn fill_piece_matrix(piece: TetrisPiece, matrix: &mut PieceMatrix, rotation: PieceRotation) {
    let matrix_str = match piece {
        // O can't rotate
        TetrisPiece::O => "11|11",

        // I, S, Z have only two rotations
        TetrisPiece::I => {
            match rotation {
                PieceRotation::UP | PieceRotation::DOWN => "0010|0010|0010|0010",
                PieceRotation::LEFT | PieceRotation::RIGHT => "0000|1111|0000|0000",
            }
        },

        TetrisPiece::Z => {
            match rotation {
                PieceRotation::UP | PieceRotation::DOWN => "010|110|100",
                PieceRotation::LEFT | PieceRotation::RIGHT => "000|110|011",
            }
        },

        TetrisPiece::S => {
            match rotation {
                PieceRotation::UP | PieceRotation::DOWN => "010|011|001",
                PieceRotation::LEFT | PieceRotation::RIGHT => "000|011|110",
            }
        },

        // J,L,T have four rotations
        TetrisPiece::J => {
            match rotation {
                PieceRotation::UP => "010|010|110",
                PieceRotation::LEFT => "000|111|001",
                PieceRotation::DOWN => "011|010|010",
                PieceRotation::RIGHT => "000|100|111"
            }
        },

        TetrisPiece::L => {
            match rotation {
                PieceRotation::UP => "010|010|011",
                PieceRotation::LEFT => "000|001|111",
                PieceRotation::DOWN => "110|010|010",
                PieceRotation::RIGHT => "000|111|100"
            }
        },

        TetrisPiece::T => {
            match rotation {
                PieceRotation::UP => "010|111|000",
                PieceRotation::LEFT => "010|110|010",
                PieceRotation::DOWN => "000|111|010",
                PieceRotation::RIGHT => "010|011|010"
            }
        }
    };

    for (row, row_vec) in matrix_str.split('|').zip(matrix.iter_mut()) {
        for (c, col) in row.chars().zip(row_vec.iter_mut()) {
            let b = match c {
                '0' => false,
                '1' => true,
                _ => panic!()
            };

            *col = b;
        }
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
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

            assert!(matrix[0][0]);
            assert!(matrix[0][1]);
            assert!(matrix[1][0]);
            assert!(matrix[1][1]);
        }
    }
}