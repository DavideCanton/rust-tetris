use pieces::TetrisPiece;
use std::ops::Add;

pub type F32_4 = [f32; 4];

pub const BLACK: F32_4 = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: F32_4 = [1.0, 1.0, 0.0, 1.0];
pub const RED: F32_4 = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: F32_4 = [0.0, 0.0, 1.0, 1.0];
pub const LIGHTBLUE: F32_4 = [0.0, 0.0, 0.6, 1.0]; // TODO
pub const GREEN: F32_4 = [0.0, 1.0, 0.0, 1.0];
pub const ORANGE: F32_4 = [1.0, 0.6, 0.0, 1.0];
pub const PURPLE: F32_4 = [1.0, 0.0, 1.0, 1.0];

pub const BGCOLOR: F32_4 = BLACK;
pub const O_COLOR: F32_4 = YELLOW;
pub const I_COLOR: F32_4 = LIGHTBLUE;
pub const S_COLOR: F32_4 = RED;
pub const Z_COLOR: F32_4 = GREEN;
pub const T_COLOR: F32_4 = PURPLE;
pub const L_COLOR: F32_4 = BLUE;
pub const J_COLOR: F32_4 = ORANGE;

pub fn piece_to_color(p: TetrisPiece) -> F32_4 {
    match p {
        TetrisPiece::O => O_COLOR,
        TetrisPiece::I => I_COLOR,
        TetrisPiece::S => S_COLOR,
        TetrisPiece::Z => Z_COLOR,
        TetrisPiece::T => T_COLOR,
        TetrisPiece::L => L_COLOR,
        TetrisPiece::J => J_COLOR,
    }
}

pub struct MyInclusiveRange<T> where T: Eq + Add<Output = T> + Copy {
    start: T,
    end: T,
    step: T,
    cur: T,
    done: bool,
}

pub fn range_inclusive<T>(start: T, end: T, step: T) -> MyInclusiveRange<T> where T: Eq + Add<Output = T> + Copy {
    MyInclusiveRange {
        done: false,
        start: start,
        end: end,
        step: step,
        cur: start,
    }
}

impl<T> Iterator for MyInclusiveRange<T> where T: Eq + Add<Output = T> + Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            let val = self.cur;
            if self.cur == self.end {
                self.done = true;
            } else {
                self.cur = self.cur + self.step;
            }
            Some(val)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::range_inclusive;

    #[test]
    fn test_range() {
        let v: Vec<_> = range_inclusive(0, 5, 1).collect();
        assert_eq!(&v[..], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_desc_range() {
        let v: Vec<_> = range_inclusive(5, 0, -1).collect();
        assert_eq!(&v[..], [5,4,3,2,1,0]);
    }

    #[test]
    fn test_range_full() {
        let v: Vec<u8> = range_inclusive(0, 255, 1).collect();
        assert_eq!(v.len(), 256);
        assert_eq!(*v.first().unwrap(), 0);
        assert_eq!(*v.last().unwrap(), 255);
    }
}