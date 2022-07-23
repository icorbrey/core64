use crate::utils::bytes;

pub struct ColorBurst {
    /// End of color burst, measured in half-lines.
    pub color_burst_end: u32,

    /// Start of color-burst, measured in half-lines.
    pub color_burst_start: u32,
}

impl From<u32> for ColorBurst {
    fn from(val: u32) -> Self {
        Self {
            color_burst_end: bytes::to_u32(0, 10, val),
            color_burst_start: bytes::to_u32(16, 10, val),
        }
    }
}

impl Into<u32> for ColorBurst {
    fn into(self) -> u32 {
        0x0000
            | bytes::from_u32(0, 10, self.color_burst_end)
            | bytes::from_u32(16, 10, self.color_burst_start)
    }
}
