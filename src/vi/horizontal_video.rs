use crate::utils::bytes;

pub struct HorizontalVideo {
    active_range_end: u32,
    active_range_start: u32,
}

impl From<u32> for HorizontalVideo {
    fn from(val: u32) -> Self {
        Self {
            active_range_end: bytes::to_u32(0, 10, val),
            active_range_start: bytes::to_u32(16, 10, val),
        }
    }
}

impl Into<u32> for HorizontalVideo {
    fn into(self) -> u32 {
        0x0000
            | bytes::from_u32(0, 10, self.active_range_end)
            | bytes::from_u32(16, 10, self.active_range_start)
    }
}
