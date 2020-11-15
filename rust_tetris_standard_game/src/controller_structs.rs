use rust_tetris_core::enums::PlayableTetrisPieceType;

#[derive(Debug, Copy, Clone)]
pub enum ControllerKey {
    Left,
    Right,
    NextRotation,
    PrevRotation,
    Pause,
    SoftDrop,
    HardDrop,
    Hold,
    Quit,
    Undo,
    Choose(PlayableTetrisPieceType),
    RemoveLine(usize),
}
