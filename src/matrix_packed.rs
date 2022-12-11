use crate::traits::*;
use quad_rand::gen_range;
use std::iter::repeat;

#[derive(Clone, Debug)]
pub struct MatrixPacked {
    tiles: Vec<[u64; 8]>,
    width: usize,
    height: usize,
}

impl Matrix<u8> for MatrixPacked {
    fn new(width: usize, height: usize, value: u8) -> Self {
        let tiles_x = 1 + ((width - 1) / 8);
        let tiles_y = 1 + ((height - 1) / 8);
        let tiles =
            Vec::from_iter(repeat([u64::from_be_bytes([value; 8]); 8]).take(tiles_x * tiles_y));
        MatrixPacked {
            tiles,
            width,
            height,
        }
    }

    fn new_with<F: FnMut((usize, usize)) -> u8>(width: usize, height: usize, mut f: F) -> Self {
        let tiles_x = 1 + ((width - 1) / 8);
        let tiles_y = 1 + ((height - 1) / 8);
        let tiles = Vec::from_iter(repeat([u64::from_be_bytes([0; 8]); 8]).take(tiles_x * tiles_y));
        let mut res = MatrixPacked {
            tiles,
            width,
            height,
        };
        for ixy in 0..width {
            for ixx in 0..height {
                res.set_at_index((ixx, ixy), f((ixx, ixy)));
            }
        }
        res
    }

    fn index(&self, (ixx, ixy): (usize, usize)) -> u8 {
        let ix_tile_x = ixx / 8;
        let ix_tile_y = ixy / 8;
        let num_tiles_y = (self.height - 1) / 8;
        (self.tiles[ix_tile_x + ix_tile_y * num_tiles_y][ixy % 8].to_le_bytes())[ixx % 8]
    }

    fn set_at_index(&mut self, (ixx, ixy): (usize, usize), value: u8) {
        let ix_tile_x = ixx / 8;
        let ix_tile_y = ixy / 8;
        let num_tiles_y = (self.height - 1) / 8;
        let mask: u64 = 0xFF << (8 * (ixx % 8));
        let v_mask = (value as u64) << (8 * (ixx % 8));
        self.tiles[ix_tile_x + ix_tile_y * num_tiles_y][ixy % 8] &= !mask;
        self.tiles[ix_tile_x + ix_tile_y * num_tiles_y][ixy % 8] |= v_mask;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl MatrixRandom<u8> for MatrixPacked {
    fn new_random(width: usize, height: usize) -> MatrixPacked {
        let tiles_x = 1 + (width - 1) / 8;
        let tiles_y = 1 + (height - 1) / 8;
        let mut tiles = Vec::new();
        for _ in 0..tiles_x {
            for _ in 0..tiles_y {
                let mut tile = [0; 8];
                for tix in &mut tile {
                    for _ in 0..8 {
                        *tix <<= 8;
                        *tix += gen_range(0_u8, 1) as u64;
                    }
                }
                tiles.push(tile);
            }
        }
        MatrixPacked {
            tiles,
            width,
            height,
        }
    }

    fn new_random_range(width: usize, height: usize, range: std::ops::RangeInclusive<u8>) -> Self {
        let tiles_x = 1 + (width - 1) / 8;
        let tiles_y = 1 + (height - 1) / 8;
        let mut tiles = Vec::new();
        for _ in 0..tiles_x {
            for _ in 0..tiles_y {
                let mut tile = [0; 8];
                for tix in &mut tile {
                    for _ in 0..8 {
                        *tix <<= 8;
                        *tix += gen_range(*range.start(), *range.end()) as u64;
                    }
                }
                tiles.push(tile);
            }
        }
        MatrixPacked {
            tiles,
            width,
            height,
        }
    }
}

impl MatrixStdConv<u8> for MatrixPacked {
    fn new_std_conv_matrix(width: usize, height: usize) -> MatrixPacked {
        let rep: u64 = u64::from_be_bytes([1; 8]);
        let mut tiles = Vec::from_iter(repeat([rep; 8]).take((width * height) as usize));

        // set the middle elemnt to 0
        let ix_tile_x = ((width / 2).max(1) - 1) / 8;
        let ix_tile_y = ((height / 2).max(1) - 1) / 8;
        let num_tiles_y = (height - 1) / 8;
        let mask: u64 = 0xFF << ((width / 2) % 8);
        let v_mask = 1 << ((width / 2) % 8);
        let tile_ix = ix_tile_x + ix_tile_y * num_tiles_y;
        tiles[tile_ix][(height / 2) % 8] &= !mask;
        tiles[tile_ix][(height / 2) % 8] |= v_mask;
        MatrixPacked {
            tiles,
            width,
            height,
        }
    }
}

mod tests {
    use super::MatrixPacked;
    use crate::traits::Matrix;
    #[test]
    fn test_set_at_index() {
        let width = 127;
        let height = 123;
        let mut mp = MatrixPacked::new(width, height, 0);
        for ixy in 0..width {
            for ixx in 0..height {
                println!("ixx: {ixx}, ixy: {ixy}");
                mp.set_at_index((ixx, ixy), ixx as u8 + ixy as u8);
                assert_eq!(mp.index((ixx, ixy)), ixx as u8 + ixy as u8);
            }
        }
        for ixy in 0..width {
            for ixx in 0..height {
                mp.set_at_index((ixx, ixy), 0x0F);
            }
        }
        for ixy in 0..width {
            for ixx in 0..height {
                assert_eq!(mp.index((ixx, ixy)), 0x0F);
            }
        }
        for ixy in 0..width {
            for ixx in 0..height {
                mp.set_at_index((ixx, ixy), 0x3A);
            }
        }
        for ixy in 0..width {
            for ixx in 0..height {
                assert_eq!(mp.index((ixx, ixy)), 0x3A);
            }
        }
    }
}
