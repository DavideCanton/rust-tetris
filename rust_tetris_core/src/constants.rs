use crate::enums::TetrisPieceRotation;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Kick = (isize, isize);

pub(crate) static I_KICKS: [[Kick; 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];
pub(crate) static DEFAULT_KICKS: [[Kick; 5]; 8] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

lazy_static! {
    pub(crate) static ref NEXT_ROTATIONS: HashMap<TetrisPieceRotation, TetrisPieceRotation> = {
        let mut m = HashMap::new();
        m.insert(TetrisPieceRotation::ZERO, TetrisPieceRotation::RIGHT);
        m.insert(TetrisPieceRotation::RIGHT, TetrisPieceRotation::TWO);
        m.insert(TetrisPieceRotation::TWO, TetrisPieceRotation::LEFT);
        m.insert(TetrisPieceRotation::LEFT, TetrisPieceRotation::ZERO);
        m
    };
    pub(crate) static ref PREV_ROTATIONS: HashMap<TetrisPieceRotation, TetrisPieceRotation> = {
        let mut m = HashMap::new();
        m.insert(TetrisPieceRotation::ZERO, TetrisPieceRotation::LEFT);
        m.insert(TetrisPieceRotation::LEFT, TetrisPieceRotation::TWO);
        m.insert(TetrisPieceRotation::TWO, TetrisPieceRotation::RIGHT);
        m.insert(TetrisPieceRotation::RIGHT, TetrisPieceRotation::ZERO);
        m
    };
}
