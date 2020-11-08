
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayableTetrisPieceType {
    T,
    L,
    J,
    O,
    I,
    S,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrisPieceType {
    Playable(PlayableTetrisPieceType),
    NotPlayable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TetrisPieceRotation {
    ZERO,
    RIGHT,
    TWO,
    LEFT,
}