use pieces::TetrisPiece;
use std::ops::Add;

pub type F32_4 = [f32; 4];

pub const R: isize = 20;
pub const C: isize = 10;

pub const INITIAL_MOVE_DOWN_THRESHOLD: f64 = 0.5;
pub const SPED_UP_THRESHOLD: f64 = 0.05;
pub const WIDTH: f64 = 30.0;
pub const WIN_W: u32 = 800;
pub const WIN_H: u32 = 600;
pub const BASE_X: u32 = (WIN_W - (WIDTH as u32 * 10)) / 2;

pub const BLACK: F32_4 = [0.0, 0.0, 0.0, 1.0];
pub const YELLOW: F32_4 = [1.0, 1.0, 0.0, 1.0];
pub const RED: F32_4 = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: F32_4 = [0.0, 0.0, 1.0, 1.0];
pub const LIGHTBLUE: F32_4 = [0.0, 0.75, 1.0, 1.0];
pub const GREEN: F32_4 = [0.0, 1.0, 0.0, 1.0];
pub const ORANGE: F32_4 = [1.0, 0.6, 0.0, 1.0];
pub const PURPLE: F32_4 = [1.0, 0.0, 1.0, 1.0];
pub const OVERLAY: F32_4 = [0.0, 0.0, 0.0, 0.8];

pub const BGCOLOR: F32_4 = BLACK;
pub const O_COLOR: F32_4 = YELLOW;
pub const I_COLOR: F32_4 = LIGHTBLUE;
pub const S_COLOR: F32_4 = RED;
pub const Z_COLOR: F32_4 = GREEN;
pub const T_COLOR: F32_4 = PURPLE;
pub const L_COLOR: F32_4 = BLUE;
pub const J_COLOR: F32_4 = ORANGE;

pub fn piece_to_color(p: TetrisPiece, is_shadow: bool) -> F32_4 {
    let original_color = match p {
        TetrisPiece::O => O_COLOR,
        TetrisPiece::I => I_COLOR,
        TetrisPiece::S => S_COLOR,
        TetrisPiece::Z => Z_COLOR,
        TetrisPiece::T => T_COLOR,
        TetrisPiece::L => L_COLOR,
        TetrisPiece::J => J_COLOR,
    };

    let mut color = [0.0, 0.0, 0.0, 0.0];
    color.copy_from_slice(&original_color);

    if is_shadow {
        color[3] = 0.3;
    }

    color
}

pub struct MyInclusiveRange<T>
    where T: Eq + Add<Output = T> + Copy + Ord
{
    start: T,
    end: T,
    step: T,
    cur: T,
    done: bool,
}

impl<T> MyInclusiveRange<T>
    where T: Eq + Add<Output = T> + Copy + Ord
{
    fn is_ascending(&self) -> bool {
        self.start + self.step > self.start
    }

    fn is_done(&self) -> bool {
        if self.is_ascending() {
            self.cur >= self.end
        } else {
            self.cur <= self.end
        }
    }

    fn is_valid(&self, val: T) -> bool {
        if self.is_ascending() {
            val >= self.start && val <= self.end
        } else {
            val <= self.start && val >= self.end
        }
    }
}

impl<T> Iterator for MyInclusiveRange<T>
    where T: Eq + Add<Output = T> + Copy + Ord
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            let val = self.cur;

            if self.is_done() {
                self.done = true;
            } else {
                self.cur = self.cur + self.step;
            }

            if self.is_valid(val) { Some(val) } else { None }
        }
    }
}

pub fn range_inclusive<T>(start: T, end: T, step: T) -> MyInclusiveRange<T>
    where T: Eq + Add<Output = T> + Copy + Ord
{
    MyInclusiveRange {
        done: false,
        start: start,
        end: end,
        step: step,
        cur: start,
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
    fn test_range_step_2() {
        let v: Vec<_> = range_inclusive(0, 5, 2).collect();
        assert_eq!(&v[..], [0, 2, 4]);
    }

    #[test]
    fn test_desc_range() {
        let v: Vec<_> = range_inclusive(5, 0, -1).collect();
        assert_eq!(&v[..], [5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_range_full() {
        let v: Vec<u8> = range_inclusive(0, 255, 1).collect();
        assert_eq!(v.len(), 256);
        assert_eq!(*v.first().unwrap(), 0);
        assert_eq!(*v.last().unwrap(), 255);
    }
}
