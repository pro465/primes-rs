use std::ops::Index;

static BOOL: [bool; 2] = [false, true];

pub struct BoolVec<const N: usize> {
    data: Vec<[u8; N]>,
}

impl<const N: usize> BoolVec<N> {
    const BITS_PER_BLOCK: usize = N * 8;

    pub fn new(size: usize) -> Self {
        Self {
            data: vec![
                [0xff; N];
                size / Self::BITS_PER_BLOCK + (size % Self::BITS_PER_BLOCK != 0) as usize
            ],
        }
    }

    pub fn reset(&mut self, idx: usize) {
        self.data[idx / Self::BITS_PER_BLOCK][idx / 8 % N] &= !(1 << idx % 8);
    }
}

impl<const N: usize> Index<usize> for BoolVec<N> {
    type Output = bool;
    fn index(&self, idx: usize) -> &'static bool {
        &BOOL[usize::from((self.data[idx / Self::BITS_PER_BLOCK][idx / 8 % N] >> (idx % 8)) & 1)]
    }
}
