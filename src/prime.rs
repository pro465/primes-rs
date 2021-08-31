use crate::bool_vec::BoolVec;

const SIZE: usize = 2_usize.pow(16);
const BITS_PER_BLOCK: usize = 8 * SIZE;

pub struct Prime {
    data: Vec<BoolVec<SIZE>>,
    len: usize,
    blocks: usize,
}

impl Prime {
    pub fn new(below: usize) -> Self {
        let blocks = below / BITS_PER_BLOCK + (below % BITS_PER_BLOCK != 0) as usize;

        Self {
            data: vec![BoolVec::new(); blocks],
            len: below,
            blocks,
        }
    }

    pub fn seive(&mut self) {
        let sqrt = f64::sqrt(self.len as f64).ceil() as usize;

        let rem = sqrt % BITS_PER_BLOCK;

        let sqrt = sqrt + BITS_PER_BLOCK - rem;

        let fourth_root = f64::sqrt(sqrt as f64).ceil() as usize;

        for j in 2..=fourth_root {
            if !self.get(j) {
                continue;
            }

            let mut k = j * j;

            while k < sqrt {
                let block_idx = k / BITS_PER_BLOCK;
                let start = k % BITS_PER_BLOCK;

                self.data[block_idx].reset(start, j);

                let next = BITS_PER_BLOCK * (block_idx + 1);
                let rem = next % j;
                let ceil = next / j + (rem != 0) as usize;
                let jump = j * ceil;

                k = jump;
            }
        }

        let sqrt_block = sqrt / BITS_PER_BLOCK;

        for block_idx in sqrt_block..self.blocks {
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
