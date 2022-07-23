use crate::utils::bytes;

pub struct HorizontalSync {
    /// Total duration of a line, measured in quarter-pixels.
    pub total_line_duration: u32,

    /// 5-bit leap pattern, used for PAL systems only.
    pub pal_leap_pattern: u32,
}

impl From<u32> for HorizontalSync {
    fn from(val: u32) -> Self {
        Self {
            total_line_duration: bytes::to_u32(0, 12, val),
            pal_leap_pattern: bytes::to_u32(16, 5, val),
        }
    }
}

impl Into<u32> for HorizontalSync {
    fn into(self) -> u32 {
        0x0000
            | bytes::from_u32(0, 12, self.total_line_duration)
            | bytes::from_u32(16, 5, self.pal_leap_pattern)
    }
}
