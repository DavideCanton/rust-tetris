macro_rules! rotations {
    ($rot:expr, Z => $z:expr, R => $r:expr, T => $t:expr, L => $l:expr) => {{
        let r = match $rot {
            TetrisPieceRotation::ZERO => $z,
            TetrisPieceRotation::RIGHT => $r,
            TetrisPieceRotation::TWO => $t,
            TetrisPieceRotation::LEFT => $l,
        };
        Vec::from(r)
    }};
}

macro_rules! same_rotation {
    ($rot:expr, $r:expr) => {{
        let r = match $rot {
            TetrisPieceRotation::ZERO => $r,
            TetrisPieceRotation::RIGHT => $r,
            TetrisPieceRotation::TWO => $r,
            TetrisPieceRotation::LEFT => $r,
        };
        Vec::from(r)
    }};
}

#[macro_export]
macro_rules! piece {
    (
        $piece:expr,
        O => $o: expr,
        I => $i: expr,
        Z => $z: expr,
        S => $s: expr,
        J => $j: expr,
        L => $l: expr,
        T => $t: expr,
    ) => {{
        match $piece {
            PlayableTetrisPieceType::O => $o,
            PlayableTetrisPieceType::I => $i,
            PlayableTetrisPieceType::Z => $z,
            PlayableTetrisPieceType::S => $s,
            PlayableTetrisPieceType::J => $j,
            PlayableTetrisPieceType::L => $l,
            PlayableTetrisPieceType::T => $t,
        }
    }};
}
