use std::io::Write;

pub struct AdpBufWriter<W: Write> {
    inner: W,
}

impl<W: Write> Write for AdpBufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
