/// All buffer sizes that are available for allocation in bytes.
/// This is a sorted list of all available buffer sizes in an increasing order
/// of 4KB (4096 bytes) increments starting from 4KB to 64KB (65536 bytes).
pub const SIZES: [usize; 16] = [
    4 * 1024,
    8 * 1024,
    12 * 1024,
    16 * 1024,
    20 * 1024,
    24 * 1024,
    28 * 1024,
    32 * 1024,
    36 * 1024,
    40 * 1024,
    44 * 1024,
    48 * 1024,
    52 * 1024,
    56 * 1024,
    60 * 1024,
    64 * 1024,
];
