use crate::bool_vec::BoolVec;

const SIZE: usize = 2_usize.pow(16);

pub struct Prime {
    data: Vec<BoolVec<SIZE>>,
    len: usize,
    blocks: usize,
}

impl Prime {
    pub fn new(size: usize) -> Self {
        let blocks = size / 8 / SIZE + (size / 8 % SIZE != 0) as usize;

        Self {
            data: vec![BoolVec::new(); blocks],
            len: size,
            blocks,
        }
    }

    pub fn seive(&mut self) {
        let sqrt = (self.len as f64).sqrt();

        let sqrt = sqrt.ceil() as usize;

        let modulo = sqrt % (SIZE * 8);

        let sqrt = sqrt + (SIZE * 8 - modulo) * (modulo != 0) as usize;

        let fourth_root = (sqrt as f64).sqrt().ceil() as usize;

        for j in 2..=fourth_root {
            if !self.get(j) {
                continue;
            }

            for k in 0..sqrt / 8 / SIZE {
                self.data[k].reset(j * j, j);
            }
        }

        for block_idx in (sqrt + 1) / 8 / SIZE..self.blocks {
            let sqrt_block =
                f64::sqrt((block_idx * SIZE * 8 + SIZE).min(self.len) as f64).ceil() as usize;

            let mut j = 2;

            while j <= sqrt_block {
                if !self.get(j) {
                    j += 2;

                    continue;
                }

                let rem = (block_idx * 8 * SIZE) % j;

                let start = (j - rem) * (rem != 0) as usize;

                self.data[block_idx].reset(start, j);

                j += 1 + (j & 1);
            }
        }
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> bool {
        self.data[idx / 8 / SIZE][idx % (8 * SIZE)]
    }
}
