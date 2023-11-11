**_Note_**: This crate has been archived due to specific use case. This will be part of the a different application in the future.

# Adaptive Buffer Writer

Adaptive Buffer Writer is like `std::io::BufWriter` with a flexible internal buffer.

## Why?

I came up with this idea while I was working on my senior project which involved improving file transfer speeds. Amongst many ideas, this was one of them.

## How?

Before a chunk of bytes is written to the output, it is buffered in the memory to reduce the number of system calls made. The buffer size is usually fixed. This crate provides a buffer writer with a flexible buffer size. The buffer size will grow from 8kb upto 128kb (increment of 8kb) if it is suitable based on performance. The size will reduce automatically if the time taken to write the bytes has increased for the last 3 writes. The buffer size will reduce to half of its current size.

### Explanation with Examples

- Suppose it takes 1 second to write 8kb and 1.5 seconds to write 16kb, then it is better to use 16kb as size because that operation is less time consuming and reduces the number of syscalls.

- Suppose it takes 1 second to write 8kb and 1.5 seconds to write 16kb and 2 seconds to write 32kb, then it is better to use 32kb becuase of its performance. 8kb/1 = 8kb/s, 16kb/1.5 = 10.67kb/s, 32kb/2 = 16kb/s. 32kb is the best option.

Subsequent write operations will increase the buffer size and reduce it if the time taken to write has increased for the last 3 writes and the performance is the same/ worse than previous buffer size, it will reduce the buffer size by 8kb.
