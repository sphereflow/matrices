use super::*;
use num_traits::{Bounded, One, Zero};
use quad_rand::RandomRange;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VecMatrix<T: Copy + Clone> {
    pub data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy + 'static> Matrix<T> for VecMatrix<T> {
    fn new_with<F: FnMut((usize, usize)) -> T>(width: usize, height: usize, mut f: F) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for ixy in 0..height {
            for ixx in 0..width {
                data.push(f((ixx, ixy)));
            }
        }
        VecMatrix {
            data,
            width,
            height,
        }
    }

    #[inline(always)]
    fn index(&self, (ixx, ixy): (usize, usize)) -> T {
        self.data[ixx + ixy * self.width]
    }

    #[inline(always)]
    fn set_at_index(&mut self, (ixx, ixy): (usize, usize), value: T) {
        self.data[ixx + ixy * self.width] = value;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn new(width: usize, height: usize, value: T) -> Self {
        VecMatrix {
            data: vec![value; width * height],
            width,
            height,
        }
    }
}

impl<T: Copy + Default> MatrixDefault<T> for VecMatrix<T> {
    fn new_default(width: usize, height: usize) -> VecMatrix<T> {
        VecMatrix {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T: Copy + Zero + One> MatrixStdConv<T> for VecMatrix<T> {
    fn new_std_conv_matrix(width: usize, height: usize) -> Self {
        let mut data = vec![One::one(); width * height];
        data[width / 2 + (height / 2) * width] = Zero::zero();
        VecMatrix {
            data,
            width,
            height,
        }
    }
}

impl<T: Copy + RandomRange + Bounded> MatrixRandom<T> for VecMatrix<T> {
    fn new_random(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(gen_range::<T>(T::min_value(), T::max_value()));
        }
        VecMatrix {
            data,
            width,
            height,
        }
    }

    fn new_random_range(width: usize, height: usize, range: RangeInclusive<T>) -> Self {
        let mut data = Vec::new();
        for _ in 0..(width * height) {
            data.push(gen_range(*range.start(), *range.end()));
        }
        VecMatrix {
            data,
            width,
            height,
        }
    }
}

impl<T: Copy + Display> Display for VecMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ixy in 0..self.height {
            for ixx in 0..self.width {
                write!(f, "{}, ", self.data[ixy * self.width + ixx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
