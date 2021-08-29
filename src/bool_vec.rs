use std::ops::Index;

static BOOL: [bool; 2] = [false, true];

pub struct BoolVec {
    data: Vec<u8>,
    len: usize,
}

impl BoolVec {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0xff; (size + 1) / 8 + ((size + 1) % 8 != 0) as usize],
            len: size + 1,
        }
    }

    #[inline(always)]
    pub fn reset(&mut self, mut start: usize, step: usize, size: usize) {
        let end = (start + size).min(self.len);

        while start < end {
            self.reset_bit(start);
            start += step
        }
    }

    #[inline(always)]
    pub fn reset_bit(&mut self, idx: usize) {
        self.data[idx / 8] &= !(1 << (idx % 8));
    }
}

impl Index<usize> for BoolVec {
    type Output = bool;

    #[inline(always)]
    fn index(&self, idx: usize) -> &'static bool {
        &BOOL[usize::from((self.data[idx / 8] >> (idx % 8)) & 1)]
    }
}
