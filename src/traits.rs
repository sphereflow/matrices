use std::ops::RangeInclusive;

use num_traits::{One, Zero};
use quad_rand::RandomRange;

pub trait MatrixDefault<T: Copy + Default> {
    fn new_default(width: usize, height: usize) -> Self;
}

pub trait MatrixRandom<T: Copy + RandomRange> {
    fn new_random(width: usize, height: usize) -> Self;
    fn new_random_range(width: usize, height: usize, range: RangeInclusive<T>) -> Self;
}

pub trait MatrixStdConv<T: Copy + Zero + One> {
    fn new_std_conv_matrix(width: usize, height: usize) -> Self;
}

pub trait MatrixNew<
    T: Copy + Zero + One + RandomRange + Default,
    M: MatrixDefault<T> + MatrixRandom<T> + MatrixStdConv<T>,
>
{
}

impl<
        T: Copy + Zero + One + RandomRange + Default,
        M: MatrixDefault<T> + MatrixRandom<T> + MatrixStdConv<T>,
    > MatrixNew<T, M> for M
{
}

pub trait Matrix<T: Copy> {
    fn new(width: usize, height: usize, value: T) -> Self;
    fn new_with<F: FnMut((usize, usize)) -> T>(width: usize, height: usize, f: F) -> Self;
    fn index(&self, ix: (usize, usize)) -> T;
    fn set_at_index(&mut self, ix: (usize, usize), value: T);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn clear(&mut self, val: T) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set_at_index((x, y), val);
            }
        }
    }

    /// form a ring around the middle element in a convolution kernel
    /// e. g. :
    /// 0 0 0 0 0
    /// 0 1 1 1 0
    /// 0 1 0 1 0
    /// 0 1 1 1 0
    /// 0 0 0 0 0
    /// make sure the matrix is a square
    fn donut(&mut self, range: RangeInclusive<usize>, val: T) {
        let wh = self.width();
        let whhalf = wh / 2;
        for ixx in 0..wh {
            let x = if ixx > whhalf { wh - ixx - 1 } else { ixx };
            for ixy in 0..wh {
                let y = if ixy > whhalf { wh - ixy - 1 } else { ixy };
                if (range.contains(&x) || range.contains(&y))
                    && (x >= *range.start() && y >= *range.start())
                {
                    self.set_at_index((ixx, ixy), val);
                }
            }
        }
    }

    fn set_at_index_sym(&mut self, sym: Symmetry, (ixx, ixy): (usize, usize), val: T) {
        // zero based indexing width and height values
        let w = self.width() - 1;
        let h = self.height() - 1;
        let wh: i64 = w as i64 / 2;
        let hh: i64 = h as i64 / 2;
        match sym {
            Symmetry::X => {
                self.set_at_index((ixx, ixy), val);
                self.set_at_index((w - ixx, ixy), val);
            }
            Symmetry::Y => {
                self.set_at_index((ixx, ixy), val);
                self.set_at_index((ixx, h - ixy), val);
            }
            Symmetry::XY => {
                self.set_at_index((ixx, ixy), val);
                self.set_at_index((w - ixx, ixy), val);
                self.set_at_index((ixx, h - ixy), val);
                self.set_at_index((w - ixx, h - ixy), val);
            }
            Symmetry::ROT90 => {
                self.set_at_index((ixx, ixy), val);
                if ixx > h || ixy > w {
                    return;
                }
                let relx: i64 = wh - ixx as i64;
                let rely: i64 = hh - ixy as i64;
                // 90 degrees
                let nx = wh + rely;
                if (0..=w as i64).contains(&nx) && (0..=h).contains(&ixx) {
                    self.set_at_index((nx as usize, ixx), val);
                }
                // 180 degrees
                self.set_at_index((w - ixx, h - ixy), val);
                // 270 degrees
                let ny = hh + relx;
                if (0..=w).contains(&ixy) && (0..=h as i64).contains(&ny) {
                    self.set_at_index((ixy, ny as usize), val);
                }
            }
            Symmetry::ROT180 => {
                self.set_at_index((ixx, ixy), val);
                self.set_at_index((w - ixx, h - ixy), val);
            }
            Symmetry::DONUT => {
                let ix_near = ixx.min(ixy);
                let ix_far = (self.width() - 1) - ixx.max(ixy);
                let ix = ix_near.min(ix_far);
                self.donut(ix..=ix, val);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symmetry {
    X,
    Y,
    XY,
    ROT90,
    ROT180,
    DONUT,
}
