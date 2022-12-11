use super::*;
use num_traits::{Bounded, One, Zero};
use quad_rand::RandomRange;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::serde_as;

/// row major array of arrays
/// N: width, M: height
#[serde_as]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConstMatrix<
    T: Copy + Clone + Serialize + DeserializeOwned,
    const M: usize,
    const N: usize,
> {
    #[serde_as(as = "[[_;N];M]")]
    pub data: [[T; N]; M],
}

impl<T: Copy + Serialize + DeserializeOwned + 'static, const M: usize, const N: usize> Matrix<T>
    for ConstMatrix<T, M, N>
{
    fn new(_width: usize, _height: usize, value: T) -> Self {
        ConstMatrix {
            data: [[value; N]; M],
        }
    }

    fn new_with<F: FnMut((usize, usize)) -> T>(_width: usize, _height: usize, mut f: F) -> Self {
        let row = |y| core::array::from_fn(|x| f((x, y)));
        ConstMatrix {
            data: core::array::from_fn(row),
        }
    }

    fn index(&self, (x, y): (usize, usize)) -> T {
        self.data[y][x]
    }

    fn set_at_index(&mut self, (x, y): (usize, usize), value: T) {
        self.data[y][x] = value;
    }

    fn width(&self) -> usize {
        N
    }

    fn height(&self) -> usize {
        M
    }
}

impl<T: Copy + Default + Serialize + DeserializeOwned, const M: usize, const N: usize> MatrixDefault<T> for ConstMatrix<T, M, N> {
    fn new_default(_width: usize, _height: usize) -> ConstMatrix<T, M, N> {
        ConstMatrix {
            data: [[T::default(); N]; M],
        }
    }
}

impl<T: Copy + Zero + One + Serialize + DeserializeOwned, const M: usize, const N: usize> MatrixStdConv<T>
    for ConstMatrix<T, M, N>
{
    fn new_std_conv_matrix(_width: usize, _height: usize) -> ConstMatrix<T, M, N> {
        let mut data = [[One::one(); N]; M];
        data[N / 2][M / 2] = Zero::zero();
        ConstMatrix { data }
    }
}

impl<T: Copy + Zero + One + RandomRange + Serialize + DeserializeOwned + Bounded, const M: usize, const N: usize> MatrixRandom<T>
    for ConstMatrix<T, M, N>
{
    fn new_random(_width: usize, _height: usize) -> ConstMatrix<T, M, N> {
        let mut data = [[Zero::zero(); N]; M];
        for slice in data.iter_mut() {
            for item in slice {
                *item = gen_range(T::min_value(), T::max_value());
            }
        }
        ConstMatrix { data }
    }

    fn new_random_range(
        _width: usize,
        _height: usize,
        range: RangeInclusive<T>,
    ) -> ConstMatrix<T, M, N> {
        let mut data = [[Zero::zero(); N]; M];
        for slice in data.iter_mut() {
            for item in slice.iter_mut() {
                *item = gen_range(*range.start(), *range.end() + One::one());
            }
        }
        ConstMatrix { data }
    }
}
