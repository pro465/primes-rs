use crate::bool_vec::BoolVec;

const SIZE: usize = 2_usize.pow(15);

pub struct Prime {
    data: BoolVec<SIZE>,
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
        let sqrt = (self.len as f64).sqrt().ceil() as usize;

        for j in 2..=sqrt {
            if !self.data[j] {
                continue;
            }

            for k in (j * j..=sqrt).step_by(j) {
                self.data.reset(k);
            }
        }

        for block in (sqrt + 1..self.len).step_by(SIZE) {
            let sqrt = f64::sqrt((block + SIZE).min(self.len) as f64).ceil() as usize;

            for j in 2..=sqrt {
                if !self.get(j) {
                    continue;
                }

                let rem = block % j;

                let start = if rem == 0 { 0 } else { j - rem };

                for k in (start..SIZE.min(self.len - block)).step_by(j) {
                    self.data.reset(block + k);
                }
            }
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        self.data[idx]
    }
}
