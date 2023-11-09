use std::io::{Result as StdIOResult, Write as StdWrite};

use crate::perf::PerfTracker;

pub struct AdpBufWriter<W, const N: usize> {
    inner: W,
    perf_tracker: PerfTracker<N>,
}

impl<W: StdWrite, const N: usize> StdWrite for AdpBufWriter<W, N> {
    fn write(&mut self, buf: &[u8]) -> StdIOResult<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> StdIOResult<()> {
        todo!()
    }
}
