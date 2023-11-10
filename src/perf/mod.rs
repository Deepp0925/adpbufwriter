use crate::buf_sizes::BufSizes;

mod speed;
pub use speed::{BufSizeType, Speed};

/// maximum number of times the performance is allowed to dip below the average
/// before we decrease the buffer size
pub const MAX_DROP_COUNT: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufSizePerf {
    size: usize,
    /// avg speed of writing the internal buffer to the I/O
    /// generally represented in kb/ms
    perf: Option<usize>,
}

/// keeps track of performance metrics for each buffer size
///
pub struct PerfTracker<const N: usize> {
    tracker: [BufSizePerf; N],
    /// number of times the performance has dipped below the previous average
    /// if this number goes above a certain threshold, we will decrease the buffer size
    /// to the best performing buffer size, beacuse we it is likely the performance won't
    /// increase after that
    drop_count: u8,
    /// index to current buffer size to use
    current: BufSizes,
    /// the index to the best performing buffer size
    best_i: usize,
}

impl<const N: usize> PerfTracker<N> {
    pub fn new(sizes: BufSizes) -> Self {
        let tracker = core::array::from_fn(|i| BufSizePerf {
            size: (i + 1) * sizes.diff(),
            perf: None,
        });

        Self {
            tracker,
            drop_count: 0,
            current: sizes,
            best_i: 0,
        }
    }

    pub fn current_size(&self) -> &usize {
        self.current.size()
    }

    ///updates the performance metrics for the given buffer size
    /// this will also update the current buffer size, based on the requirements
    pub fn update(&mut self, buf_size: &usize, speed: Speed) {
        // we don't want to update the perfromance metrics for the current buffer size
        // beacuse we found the best performing buffer size
        if self.drop_count >= MAX_DROP_COUNT {
            return;
        }

        let avg = speed.cal_avg(buf_size);
        let index = buf_size / self.current.diff() - 1;
        let buf_perf = &mut self.tracker[index];
        buf_perf.perf = Some(avg);

        if index == 0 {
            return;
        }

        // checking the performance has dropped three times in a row
        // from the best performing buffer size, if so we will decrease the buffer size to that
        let best_buf_perf = &self.tracker[self.best_i];
        if let Some(best_avg) = best_buf_perf.perf {
            if avg < best_avg {
                self.drop_count += 1;
            } else {
                self.best_i = index;
                self.drop_count = 0;
            }

            if self.drop_count >= MAX_DROP_COUNT {
                self.current.set_size(best_buf_perf.size);
            } else {
                self.current.next();
            }
        }
    }
}
