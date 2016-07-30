use pieces::TetrisPiece;

pub type TetrisCell = Option<TetrisPiece>;

pub struct TetrisBoard {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<TetrisCell>>,
    empty_row_proto: Vec<TetrisCell>,
}

impl TetrisBoard {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut board = TetrisBoard {
            rows: rows,
            cols: cols,
            data: Vec::with_capacity(rows),
            empty_row_proto: vec![None; cols],
        };

        for _ in 0..rows {
            board.data.push(board.empty_row_proto.clone());
        }

        board
    }

    pub fn get(&self, i: usize, j: usize) -> TetrisCell {
        self.data[i][j]
    }

    pub fn is_set(&self, i: usize, j: usize) -> bool {
        self.data[i][j].is_some()
    }

    pub fn set(&mut self, i: usize, j: usize, p: TetrisPiece) {
        self.set_val(i, j, Some(p));
    }

    pub fn clear(&mut self, i: usize, j: usize) {
        self.set_val(i, j, None);
    }

    pub fn set_val(&mut self, i: usize, j: usize, b: TetrisCell) {
        self.data[i][j] = b;
    }

    pub fn rows<'a>(&'a self) -> Box<Iterator<Item=&'a Vec<TetrisCell>> + 'a> {
        Box::new(self.data.iter())
    }

    pub fn rows_mut<'a>(&'a mut self) -> Box<Iterator<Item=&'a mut Vec<TetrisCell>> + 'a> {
        Box::new(self.data.iter_mut())
    }
}