use pieces::{TetrisPiece, PieceInfo};
use utils::range_inclusive;

pub type TetrisCell = Option<TetrisPiece>;

pub struct TetrisBoard {
    pub rows: isize,
    pub cols: isize,
    data: Vec<Vec<TetrisCell>>,
    empty_row_proto: Vec<TetrisCell>,
}

impl TetrisBoard {
    pub fn new(rows: isize, cols: isize) -> Self {
        let mut board = TetrisBoard {
            rows: rows,
            cols: cols,
            data: Vec::with_capacity(rows as usize),
            empty_row_proto: vec![None; cols as usize],
        };

        for _ in 0..rows {
            board.data.push(board.empty_row_proto.clone());
        }

        board
    }

    pub fn get(&self, i: isize, j: isize) -> TetrisCell {
        self.data[i as usize][j as usize]
    }

    pub fn is_set(&self, i: isize, j: isize) -> bool {
        if i >= self.rows || j >= self.cols || i < 0 || j < 0 {
            false
        } else {
            self.data[i as usize][j as usize].is_some()
        }
    }


    pub fn set(&mut self, i: isize, j: isize, p: TetrisPiece) {
        self.set_val(i, j, Some(p));
    }

    pub fn clear(&mut self, i: isize, j: isize) {
        self.set_val(i, j, None);
    }

    pub fn set_val(&mut self, i: isize, j: isize, b: TetrisCell) {
        self.data[i as usize][j as usize] = b;
    }

    pub fn is_complete(&self, i: isize) -> bool {
        self.data[i as usize].iter().all(|cell| cell.is_some())
    }

    pub fn finalize(&mut self, piece: &PieceInfo, r: isize, c: isize) {
        let w = piece.board.cols;
        let h = piece.board.rows;

        for i in 0..h {
            for j in 0..w {
                let pcell = piece.board.is_set(i, j);

                if pcell {
                    self.set(i + r, j + c, piece.piece);
                }
            }
        }
    }

    pub fn remove_completed_rows(&mut self, last_to_copy: Option<isize>)
    {
        let mut ranges = vec![];

        let mut from = None;
        let mut to = None;

        for i in range_inclusive(self.data.len() as isize - 1, 0, -1) {
            let i = i as isize;

            if self.is_complete(i) {
                if from.is_none() {
                    from = Some(i);
                } else if to.is_none() {
                    to = Some(i);
                }
            } else {
                ranges.push((from.unwrap(), to.unwrap()));
                from = None;
                to = None;
            }
        }

        for range in ranges {
            self.remove_rows(range.0, range.1, last_to_copy);
        }
    }

    pub fn rows<'a>(&'a self) -> Box<Iterator<Item = &'a Vec<TetrisCell>> + 'a> {
        Box::new(self.data.iter())
    }

    pub fn rows_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Vec<TetrisCell>> + 'a> {
        Box::new(self.data.iter_mut())
    }

    pub fn remove_rows(&mut self, from: isize, to: isize, last_to_copy: Option<isize>) {
        let offset = to - from;

        if offset == 0 {
            return;
        }

        let last_to_copy = last_to_copy.unwrap_or(offset);

        for i in (to - 1)..last_to_copy {
            self.data.swap(i as usize, i as usize - offset as usize);
        }

        for i in last_to_copy..(last_to_copy - offset) {
            self.data[i as usize] = self.empty_row_proto.clone();
        }
    }

    pub fn get_first_set_col(&self) -> Option<isize> {
        for j in 0..self.cols {
            for i in 0..self.rows {
                if self.is_set(i, j) {
                    return Some(j);
                }
            }
        }
        None
    }

    pub fn get_last_set_col(&self) -> Option<isize> {
        for j in (0..self.cols).rev() {
            for i in 0..self.rows {
                if self.is_set(i, j) {
                    return Some(j);
                }
            }
        }
        None
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::TetrisBoard;
    use pieces::TetrisPiece;

    #[test]
    fn test_remove_rows() {
        let mut board = TetrisBoard::new(5, 3);

        board.set(4, 0, TetrisPiece::O);
    }
}