use crate::utils::bytes;

pub struct VideoScale {
    /// Inverse of decimal scale factor.
    scale_factor: u32,

    /// Subpixel offset.
    subpixel_offset: u32,
}

impl From<u32> for VideoScale {
    fn from(val: u32) -> Self {
        Self {
            scale_factor: bytes::to_u32(0, 12, val),
            subpixel_offset: bytes::to_u32(16, 12, val),
        }
    }
}

impl Into<u32> for VideoScale {
    fn into(self) -> u32 {
        0x0000
            | bytes::from_u32(0, 12, self.scale_factor)
            | bytes::from_u32(16, 12, self.subpixel_offset)
    }
}
