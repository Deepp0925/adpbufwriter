use std::{
    io::{Error, ErrorKind, Result as IOResult},
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use crate::{
    buf_sizes::BufSizes,
    perf::{BufSizeType, PerfTracker, Speed},
};
use futures_lite::{io::BufWriter, AsyncWrite};

pub struct AdpBufWriter<W, const N: usize> {
    inner: BufWriter<W>,
    time: Option<Instant>,
    perf_tracker: PerfTracker<N>,
}

impl<W: Unpin + AsyncWrite, const N: usize> AdpBufWriter<W, N> {
    pub fn new(inner: W, sizes: BufSizes) -> Self {
        let perf_tracker = PerfTracker::new(sizes);
        Self {
            inner: BufWriter::with_capacity(*perf_tracker.current_size(), inner),
            time: None,
            perf_tracker,
        }
    }

    pub fn get_ref(&self) -> &W {
        self.inner.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut W {
        self.inner.get_mut()
    }
}

impl<W: AsyncWrite + Unpin, const N: usize> AsyncWrite for AdpBufWriter<W, N> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<IOResult<usize>> {
        let this = self.get_mut();
        if this.time.is_none() {
            this.time = Some(Instant::now());
        }
        todo!()
    }

    // TODO fix this
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        // use the time if already set, otherwise it means the poll write directly called poll flush
        // so we can create new
        let this = self.get_mut();
        if this.time.is_none() {
            this.time = Some(Instant::now());
        }

        let res = Pin::new(&mut this.inner).poll_flush(cx);
        if let Poll::Ready(Ok(_)) = res {
            // ok to unwrap since we set it above
            let time = this.time.take().unwrap();
            let len = this.inner.buffer().len();
            let current_size = *this.perf_tracker.current_size();
            let speed = Speed {
                time: time.elapsed(),
                type_: if current_size >= len {
                    BufSizeType::Full
                } else {
                    BufSizeType::Portion(len)
                },
            };
            this.perf_tracker.update(&current_size, speed);
        }

        res
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<IOResult<()>> {
        todo!()
    }
}
