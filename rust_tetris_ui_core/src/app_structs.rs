use rust_tetris_core::{
    board::TetrisBoard,
    pieces::{Kick, TetrisPiece, TetrisPieceRotation, TetrisPieceType},
};

pub struct TetrisPieceWithPosition {
    r: isize,
    c: isize,
    piece: TetrisPiece,
}

impl TetrisPieceWithPosition {
    pub fn new(r: isize, c: isize, piece: TetrisPiece) -> Self {
        TetrisPieceWithPosition { r, c, piece }
    }

    pub fn row(&self) -> isize {
        self.r
    }

    pub fn col(&self) -> isize {
        self.c
    }

    pub fn tetris_piece(self) -> TetrisPiece {
        self.piece
    }

    pub fn tetris_piece_ref(&self) -> &TetrisPiece {
        &self.piece
    }

    pub fn tetris_piece_mut(&mut self) -> &mut TetrisPiece {
        &mut self.piece
    }

    pub fn finalize_on(&self, board: &mut TetrisBoard) {
        self.piece.call_on_set_cells(|i, j| {
            board.set(
                i + self.row(),
                j + self.col(),
                TetrisPieceType::Playable(self.piece.piece_type),
            );
        });
    }

    pub fn collides_on_next(&self, matrix: &TetrisBoard) -> bool {
        self.piece.collides_on_next(self.r, self.c, matrix)
    }

    pub fn collides_on_next_with_row(&self, r: isize, matrix: &TetrisBoard) -> bool {
        self.piece.collides_on_next(r, self.c, matrix)
    }

    pub fn try_move_left(&mut self, matrix: &TetrisBoard) -> bool {
        let first_col = self.piece.board.get_first_set_col().unwrap() as isize;

        if self.c + first_col > 0 && !self.piece.collides_left(self.r, self.c, &matrix) {
            self.move_left();
            true
        } else {
            false
        }
    }

    pub fn try_move_right(&mut self, matrix: &TetrisBoard) -> bool {
        let last_col = self.piece.board.get_last_set_col().unwrap() as isize;

        if self.c + last_col < (matrix.cols as isize) - 1
            && !self.piece.collides_right(self.r, self.c, &matrix)
        {
            self.move_right();
            true
        } else {
            false
        }
    }

    pub fn kick_by(&mut self, kick: Kick) {
        self.r -= kick.1;
        self.c += kick.0;
    }

    pub fn can_rotate(&self, prev_rot: TetrisPieceRotation, matrix: &TetrisBoard) -> Option<Kick> {
        for kick in self.piece.get_kicks(prev_rot) {
            if !self.piece.collides_kick(self.r, self.c, matrix, kick) {
                return Some(*kick);
            }
        }
        None
    }

    pub fn move_down(&mut self) {
        self.r += 1;
    }

    pub fn move_left(&mut self) {
        self.c -= 1;
    }

    pub fn move_right(&mut self) {
        self.c += 1;
    }
}

pub struct HoldTetrisPiece {
    pub piece: TetrisPiece,
    pub already_hold: bool,
}

impl HoldTetrisPiece {
    pub fn new(mut piece: TetrisPiece) -> Self {
        piece.set_rotation(TetrisPieceRotation::ZERO);
        HoldTetrisPiece {
            piece,
            already_hold: false,
        }
    }

    pub fn can_swap(obj: &Option<HoldTetrisPiece>) -> bool {
        obj.is_none() || !obj.as_ref().unwrap().already_hold
    }
    pub fn set_hold(&mut self) {
        self.already_hold = true;
    }
    pub fn reset_hold(&mut self) {
        self.already_hold = false;
    }
}
