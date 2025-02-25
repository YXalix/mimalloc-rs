// 4mb for one segment
pub const MI_SEGMENT_SHIFT: usize = 22;
pub const MI_SEGMENT_SIZE: usize = 1 << MI_SEGMENT_SHIFT;

/// Magic number for mimalloc-rs
/// ----------------------------
/// |        os pool           |
/// ----------------------------
pub const MI_POOL_ALIGNMENT: usize = MI_SEGMENT_SIZE;
pub const MI_POOL_SIZE: usize = MI_SEGMENT_SIZE * 10;

