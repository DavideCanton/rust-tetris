pub struct TetrisBoard {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<bool>>,
    empty_row_proto: Vec<bool>,
}

impl TetrisBoard {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut board = TetrisBoard {
            rows: rows,
            cols: cols,
            data: vec![],
            empty_row_proto: vec![false; cols],
        };

        for _ in 0..rows {
            board.data.push(board.empty_row_proto.clone());
        }

        board
    }

    pub fn is_set(&self, i: usize, j: usize) -> bool {
        self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize) {
        self.set_val(i, j, true);
    }

    pub fn clear(&mut self, i: usize, j: usize) {
        self.set_val(i, j, false);
    }

    pub fn set_val(&mut self, i: usize, j: usize, b: bool) {
        self.data[i][j] = b;
    }
}
