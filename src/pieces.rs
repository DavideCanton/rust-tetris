pub enum TetrisPiece {
    T,
    L,
    J,
    O,
    I,
    S,
    Z
}

pub fn get_piece_size(piece: TetrisPiece) -> (usize, usize) {
    match piece {
        TetrisPiece::I => (1, 4),
        TetrisPiece::O => (2, 2),
        _ => (2, 3)
    }
}

pub fn get_piece_matrix(piece: TetrisPiece) -> Vec<Vec<bool>> {
    let matrix_str = match piece {
        TetrisPiece::I => "1|1|1|1",
        TetrisPiece::J => "01|01|11",
        TetrisPiece::L => "10|10|11",
        TetrisPiece::Z => "01|11|10",
        TetrisPiece::S => "10|11|01",
        TetrisPiece::O => "11|11",
        TetrisPiece::T => "10|11|10"
    };

    let mut matrix = vec![];

    for row in matrix_str.split('|') {
        let mut row_vec = vec![];

        for c in row.chars() {
            let b = match c {
                '0' => false,
                '1' => true,
                _ => panic!()
            };

            row_vec.push(b);
        }

        matrix.push(row_vec);
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::get_piece_size;
    use super::TetrisPiece;

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
}