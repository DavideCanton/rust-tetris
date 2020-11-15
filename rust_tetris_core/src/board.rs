use crate::enums::{PlayableTetrisPieceType, TetrisPieceType};
use std::fmt::{Debug, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrisCell {
    FilledCell(TetrisPieceType),
    EmptyCell,
}

pub fn is_filled(cell: TetrisCell) -> bool {
    match cell {
        TetrisCell::FilledCell(_) => true,
        TetrisCell::EmptyCell => false,
    }
}

pub fn playable_piece_to_cell(piece: PlayableTetrisPieceType) -> TetrisCell {
    TetrisCell::FilledCell(TetrisPieceType::Playable(piece))
}

pub fn not_playable_piece_to_cell() -> TetrisCell {
    TetrisCell::FilledCell(TetrisPieceType::NotPlayable)
}

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
            empty_row_proto: vec![TetrisCell::EmptyCell; cols as usize],
        };

        for _ in 0..rows {
            board.data.push(board.empty_row_proto.clone());
        }

        board
    }

    pub fn get(&self, i: isize, j: isize) -> TetrisCell {
        self.data[i as usize][j as usize]
    }

    pub fn is_in_bounds(&self, i: isize, j: isize) -> bool {
        !(i >= self.rows || j >= self.cols || i < 0 || j < 0)
    }

    pub fn is_set(&self, i: isize, j: isize) -> bool {
        if !self.is_in_bounds(i, j) {
            false
        } else {
            is_filled(self.data[i as usize][j as usize])
        }
    }

    pub fn set(&mut self, i: isize, j: isize, p: TetrisPieceType) {
        self.set_val(i, j, TetrisCell::FilledCell(p));
    }

    pub fn clear(&mut self, i: isize, j: isize) {
        self.set_val(i, j, TetrisCell::EmptyCell);
    }

    pub fn set_val(&mut self, i: isize, j: isize, b: TetrisCell) {
        self.data[i as usize][j as usize] = b;
    }

    pub fn is_complete(&self, i: isize) -> bool {
        self.data[i as usize].iter().all(|&cell| is_filled(cell))
    }

    pub fn is_empty(&self) -> bool {
        self.data
            .iter()
            .all(|row| row.iter().all(|&cell| !is_filled(cell)))
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
                let from_i = from.unwrap();
                let to_i = to.unwrap_or(from_i);

                ranges.push((from_i, to_i - 1));
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
        if from == to {
            return;
        }

        let offset = from - to;
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
        (0..self.cols)
            .flat_map(|j| (0..self.rows).map(move |i| (i, j)))
            .find(|&(i, j)| self.is_set(i, j))
            .map(|(_, j)| j)
    }

    pub fn get_last_set_col(&self) -> Option<isize> {
        (0..self.cols)
            .rev()
            .flat_map(|j| (0..self.rows).map(move |i| (i, j)))
            .find(|&(i, j)| self.is_set(i, j))
            .map(|(_, j)| j)
    }
}

impl Debug for TetrisBoard {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = match self.get(i, j) {
                    TetrisCell::FilledCell(_) => '*',
                    TetrisCell::EmptyCell => ' ',
                };
                write!(formatter, "{}", c)?
            }

            writeln!(formatter)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_board(board: &mut TetrisBoard, s: &str) {
        let c = board.cols;

        for (i, ch) in s.chars().enumerate() {
            let i = i as isize;
            match ch {
                ' ' => board.clear(i / c, i % c),
                '*' => board.set(
                    i / c,
                    i % c,
                    TetrisPieceType::Playable(PlayableTetrisPieceType::O),
                ),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_remove_rows() {
        let mut board = TetrisBoard::new(5, 3);

        load_board(&mut board, "     *  *******");

        println!("{:?}", board);
        println!("-----------");

        let ranges = board.completed_rows();
        board.remove_ranges(ranges, Some(5));

        println!("{:?}", board);
        println!("-----------");

        for i in 0..5 {
            for j in 0..3 {
                if (i == 3 || i == 4) && j == 2 {
                    assert!(is_filled(board.get(i, j)));
                } else {
                    assert!(!is_filled(board.get(i, j)));
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

        let ranges = board.completed_rows();
        board.remove_ranges(ranges, Some(5));

        println!("{:?}", board);
        println!("-----------");

        for i in 0..5 {
            for j in 0..3 {
                assert!(!is_filled(board.get(i, j)));
            }
        }
    }
}
