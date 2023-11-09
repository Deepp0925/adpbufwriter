use std::time::Duration;

use crate::buf_sizes::BufSizes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BufSizeType {
    /// the entire buffer is filled
    Full,
    /// only a portion of the buffer is filled
    Portion(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Speed {
    /// the type of buffer size
    pub type_: BufSizeType,
    /// the time it took to write the buffer to the I/O
    pub time: Duration,
}

impl Speed {
    pub fn cal_avg(&self, buf_size: &usize) -> usize {
        match &self.type_ {
            BufSizeType::Full => buf_size / self.time.as_millis() as usize,
            BufSizeType::Portion(bytes) => bytes / self.time.as_millis() as usize,
        }
    }
}
