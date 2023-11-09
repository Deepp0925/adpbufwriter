#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufSizes {
    // current size
    size: usize,
    // min buffer size
    max: usize,
    // max buffer size
    min: usize,
    // difference between each buffer size
    diff: usize,
}

impl BufSizes {
    pub fn min(&self) -> &usize {
        &self.min
    }

    pub fn max(&self) -> &usize {
        &self.max
    }

    pub fn diff(&self) -> &usize {
        &self.diff
    }

    pub fn new(min: usize, max: usize, diff: usize) -> Self {
        assert!(min < max, "min must be less than max");
        assert!(min % diff == 0, "min must be divisible by diff");
        assert!(max % diff == 0, "max must be divisible by diff");
        Self {
            size: min,
            max,
            min,
            diff,
        }
    }

    pub fn size(&self) -> &usize {
        &self.size
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size.clamp(self.min, self.max);
    }

    pub fn next(&mut self) {
        self.size += self.diff;
        self.size = self.size.clamp(self.min, self.max);
    }
}
