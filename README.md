# Adaptive Buffer Writer

Adaptive Buffer Writer is like `std::io::BufWriter` with ever changing internal buffer. The `BufWriter` has a fixed buffer size of 8KiB and
