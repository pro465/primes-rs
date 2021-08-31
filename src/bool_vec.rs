use std::ops::Index;

static BOOL: [bool; 2] = [false, true];

#[derive(Clone, Copy)]
pub struct BoolVec<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> BoolVec<N> {
    pub fn new() -> Self {
        Self { data: [0xff; N] }
    }

    #[inline(always)]
    ufn reset(&mut self, mut start: usize, step: usize) {
        while start < N * 8 {
            self.reset_bit(start);
            start += step
        }
    }

    #[inline(always)]
    pub fn reset_bit(&mut self, idx: usize) {
        self.data[idx / 8] &= !(1 << (idx % 8));
    }
}

impl<const N: usize> Index<usize> for BoolVec<N> {
    type Output = bool;

    #[inline(always)]
    fn index(&self, idx: usize) -> &'static bool {
        &BOOL[usize::from((self.data[idx / 8] >> (idx % 8)) & 1)]
    }
}
