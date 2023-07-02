/// All buffer sizes that are available for allocation in bytes.
/// This is a sorted list of all available buffer sizes in an increasing order
/// of 8kb (8096) increments starting from 8KB to 64KB (65536 bytes).
pub const SIZES: [usize; 8] = [
    8 * 1024,
    16 * 1024,
    24 * 1024,
    32 * 1024,
    40 * 1024,
    48 * 1024,
    56 * 1024,
    64 * 1024,
];

/// MAX_SIZE is the maximum size of the buffer that can be allocated.
pub const MAX_SIZE: usize = SIZES[SIZES.len() - 1];

/// MIN_SIZE is the minimum size of the buffer that can be allocated.
pub const MIN_SIZE: usize = SIZES[0];
