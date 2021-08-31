use crate::bool_vec::BoolVec;

const SIZE: usize = 2_usize.pow(16);
const BITS_PER_BLOCK: usize = 8 * SIZE;

pub struct Prime {
    data: Vec<BoolVec<SIZE>>,
    len: usize,
    blocks: usize,
}

impl Prime {
    pub fn new(size: usize) -> Self {
        let blocks = size / BITS_PER_BLOCK + (size % BITS_PER_BLOCK != 0) as usize;

        Self {
            data: vec![BoolVec::new(); blocks],
            len: size,
            blocks,
        }
    }

    pub fn seive(&mut self) {
        let sqrt = (self.len as f64).sqrt().ceil() as usize;

        let rem = sqrt % BITS_PER_BLOCK;

        let sqrt = sqrt + (BITS_PER_BLOCK - rem) * (rem != 0) as usize;

        let fourth_root = f64::sqrt(sqrt as f64).ceil() as usize;

        let sqrt = BITS_PER_BLOCK + sqrt;

        for j in 2..=fourth_root {
            if !self.get(j) {
                continue;
            }

            let sqrt_block = sqrt / BITS_PER_BLOCK;
            let j_sq_block = j * j / BITS_PER_BLOCK;

            for k in j_sq_block..sqrt_block {
                let bit_idx = k * BITS_PER_BLOCK;
                let rem = bit_idx + j;
                let ceil = (bit_idx / j) + (rem != 0) as usize;

                let start = j * ceil - bit_idx;

                self.data[k].reset(start, j);
            }
        }

        for block_idx in sqrt / BITS_PER_BLOCK..self.blocks {
            let bit_idx = block_idx * BITS_PER_BLOCK;

            let sqrt = f64::sqrt((bit_idx + BITS_PER_BLOCK).min(self.len) as f64).ceil() as usize;

            let mut j = 2;

            while j <= sqrt {
                if !self.get(j) {
                    j += 2;

                    continue;
                }

                let rem = bit_idx % j;

                let ceil = (bit_idx / j) + (rem != 0) as usize;

                let start = j * ceil - bit_idx;

                self.data[block_idx].reset(start, j);

                j += 1 + (j & 1);
            }
        }
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> bool {
        self.data[idx / BITS_PER_BLOCK][idx % BITS_PER_BLOCK]
    }
}
