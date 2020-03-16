use crate::pieces::{TetrisPieceStruct, TetrisPiece};
use std::fmt::{Debug, Formatter, Result};

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
            rows,
            cols,
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

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|row| row.iter().all(|cell| cell.is_none()))
    }

    pub fn finalize(&mut self, piece: &TetrisPieceStruct, r: isize, c: isize) {
        let w = piece.board.cols;
        let h = piece.board.rows;

        for i in 0..h {
            for j in 0..w {
                if piece.board.is_set(i, j) {
                    self.set(i + r, j + c, piece.piece);
                }
            }
        }
    }

    pub fn completed_rows(&mut self) -> Vec<(isize, isize)> {
        let mut ranges = vec![];

        let mut from = None;
        let mut to: Option<isize> = None;

        for i in (0..self.data.len()).rev() {
            let i = i as isize;

            if self.is_complete(i) {
                if from.is_none() {
                    from = Some(i);
                } else {
                    to = Some(i);
                }
            } else if from.is_some() {
                let fromI = from.unwrap();
                let toI = to.unwrap_or(fromI);

                ranges.push((fromI, toI - 1));
                from = None;
                to = None;
            }
        }

        ranges
    }

    pub fn remove_ranges(&mut self, ranges: Vec<(isize, isize)>, last_to_copy: Option<isize>) {
        for range in &ranges {
            self.remove_rows(range.0, range.1, last_to_copy);
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<TetrisCell>> {
        self.data.iter()
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<TetrisCell>> {
        self.data.iter_mut()
    }

    pub fn remove_rows(&mut self, from: isize, to: isize, last_to_copy: Option<isize>) {
        let offset = from - to;

        if offset == 0 {
            return;
        }

        let last_to_copy = last_to_copy.unwrap_or(offset);
        let last_to_copy_rev = self.rows - last_to_copy;

        for i in ((to + 1)..=from).rev() {
            self.data[i as usize] = self.empty_row_proto.clone();
        }
        for i in (last_to_copy_rev..=to).rev() {
            let row_to_clear = i as usize + offset as usize;
            self.data.swap(i as usize, row_to_clear);
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

impl Debug for TetrisBoard {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let cell = self.get(i, j);

                let c = cell.map_or(' ', |_| '*');

                write!(formatter, "{}", c)?
            }

            write!(formatter, "{}", "\n")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::TetrisBoard;
    use crate::pieces::TetrisPiece;

    fn load_board(board: &mut TetrisBoard, s: &str) {
        let c = board.cols;

        let mut i = 0;
        for ch in s.chars() {
            match ch {
                ' ' => board.clear(i / c, i % c),
                '*' => board.set(i / c, i % c, TetrisPiece::O),
                _ => panic!(),
            }
            i += 1;
        }
    }

    #[test]
    fn test_remove_rows() {
        let mut board = TetrisBoard::new(5, 3);

        load_board(&mut board, "     *  *******");

        println!("{:?}", board);
        println!("-----------");

        board.remove_completed_rows(Some(5));

        println!("{:?}", board);
        println!("-----------");

        for i in 0..5 {
            for j in 0..3 {
                if (i == 3 || i == 4) && j == 2 {
                    assert!(board.get(i, j).is_some());
                } else {
                    assert!(board.get(i, j).is_none());
                }
            }
        }
    }

    #[test]
    fn test_remove_rows3() {
        let mut board = TetrisBoard::new(5, 3);

        load_board(&mut board, "      *********");

        println!("{:?}", board);
        println!("-----------");

        board.remove_completed_rows(Some(5));

        println!("{:?}", board);
        println!("-----------");

        for i in 0..5 {
            for j in 0..3 {
                assert!(board.get(i, j).is_none());
            }
        }
    }
}
