use crate::utils::bytes;

pub struct VideoTiming {
    horizontal_sync_width: u32,
    color_burst_width: u32,
    vertical_sync_width: u32,
    color_burst_offset: u32,
}

impl From<u32> for VideoTiming {
    fn from(val: u32) -> Self {
        Self {
            horizontal_sync_width: bytes::to_u32(0, 8, val),
            color_burst_width: bytes::to_u32(8, 8, val),
            vertical_sync_width: bytes::to_u32(16, 4, val),
            color_burst_offset: bytes::to_u32(20, 10, val),
        }
    }
}

impl Into<u32> for VideoTiming {
    fn into(self) -> u32 {
        0x0000
            | bytes::from_u32(0, 8, self.horizontal_sync_width)
            | bytes::from_u32(8, 8, self.color_burst_width)
            | bytes::from_u32(16, 4, self.vertical_sync_width)
            | bytes::from_u32(20, 10, self.color_burst_offset)
    }
}
