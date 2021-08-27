use std::ops::Index;

static BOOL: [bool; 2] = [false, true];

pub struct BoolVec {
    data: Vec<u8>,
    length: usize,
}

impl BoolVec {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0xff; size / 8 + (size % 8 != 0) as usize],
            length: size,
        }
    }

    pub fn set(&mut self, idx: usize, val: bool) {
        if val {
            self.data[idx / 8] |= 1 << idx % 8;
        } else {
            self.data[idx / 8] &= !(1 << idx % 8);
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Index<usize> for BoolVec {
    type Output = bool;
    fn index(&self, idx: usize) -> &'static bool {
        &BOOL[usize::from((self.data[idx / 8] >> (idx % 8)) & 1)]
    }
}
