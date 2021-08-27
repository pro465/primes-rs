use crate::bool_vec::BoolVec;

pub struct Prime {
    data: BoolVec,
}

impl Prime {
    pub fn new(size: usize) -> Self {
        Self {
            data: BoolVec::new(size - 2),
        }
    }

    pub fn seive(&mut self) {
        for i in 0..=(self.data.len() as f64 + 2.).sqrt().ceil() as usize {
            if !self.data[i] {
                continue;
            }

            for i in ((i + 2) * (i + 2) - 2..=self.data.len()).step_by(i + 2) {
                self.data.set(i, false);
            }
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        self.data[idx]
    }
}
