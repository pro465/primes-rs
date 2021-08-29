use crate::bool_vec::BoolVec;

const SIZE: usize = 2_usize.pow(16);

pub struct Prime {
    data: BoolVec,
    len: usize,
}

impl Prime {
    pub fn new(size: usize) -> Self {
        Self {
            data: BoolVec::new(size),
            len: size,
        }
    }

    pub fn seive(&mut self) {
        let sqrt = (self.len as f64).sqrt();
        let fourth_root = sqrt.sqrt().ceil() as usize;

        let sqrt = sqrt.ceil() as usize;

        for j in 2..=fourth_root {
            if !self.get(j) {
                continue;
            }

            for k in (j * j..=sqrt).step_by(j) {
                self.data.reset_bit(k);
            }
        }

        let start = sqrt + 1;

        for block in (start..=self.len).step_by(SIZE) {
            let sqrt_block = f64::sqrt((block + SIZE).min(self.len) as f64).ceil() as usize;

            let mut j = 2;

            while j <= sqrt_block {
                if !self.get(j) {
                    j += 2;

                    continue;
                }

                let rem = block % j;

                let start = (j - rem) * (rem != 0) as usize;

                self.data.reset::<SIZE>(block + start, j);

                j += 1 + (j & 1);
            }
        }
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> bool {
        self.data[idx]
    }
}
