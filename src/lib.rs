mod adp_buf_writer;
mod buf_sizes;
mod perf;
// use std::io::{self, copy, Write};

// struct AdaptiveBufferedWriter<W: Write> {
//     underlying_writer: W,
//     buffer: Vec<u8>,
//     position: usize,
//     max_buffer_size: usize,
//     min_buffer_size: usize,
// }

// impl<W: Write> AdaptiveBufferedWriter<W> {
//     fn new(
//         underlying_writer: W,
//         initial_buffer_size: usize,
//         max_buffer_size: usize,
//         min_buffer_size: usize,
//     ) -> Self {
//         AdaptiveBufferedWriter {
//             underlying_writer,
//             buffer: vec![0; initial_buffer_size],
//             position: 0,
//             max_buffer_size,
//             min_buffer_size,
//         }
//     }

//     fn write(&mut self, data: &[u8]) -> io::Result<()> {
//         let data_size = data.len();
//         let buffer_size = self.buffer.len();

//         if self.position + data_size > buffer_size {
//             self.flush()?;
//             if data_size > buffer_size {
//                 self.underlying_writer.write_all(data)?;
//                 return Ok(());
//             }
//         }

//         self.buffer[self.position..self.position + data_size].copy_from_slice(data);
//         self.position += data_size;

//         Ok(())
//     }

//     fn flush(&mut self) -> io::Result<()> {
//         if self.position > 0 {
//             self.underlying_writer
//                 .write_all(&self.buffer[..self.position])?;
//             self.position = 0;
//         }

//         Ok(())
//     }

//     fn adjust_buffer_size(&mut self) {
//         let data_size = self.position;
//         let buffer_size = self.buffer.len();

//         if data_size > self.max_buffer_size && buffer_size < self.max_buffer_size {
//             self.resize_buffer(self.max_buffer_size);
//         } else if data_size < self.min_buffer_size && buffer_size > self.min_buffer_size {
//             self.resize_buffer(self.min_buffer_size);
//         }
//     }

//     fn resize_buffer(&mut self, new_size: usize) {
//         let mut new_buffer = vec![0; new_size];
//         new_buffer[..self.position].copy_from_slice(&self.buffer[..self.position]);
//         self.buffer = new_buffer;
//     }

//     fn close(mut self) -> io::Result<()> {
//         self.flush()?;
//         self.underlying_writer.flush()?;
//         Ok(())
//     }
// }
