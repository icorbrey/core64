use crate::utils::bytes;

use super::anti_aliasing_mode::AntiAliasingMode;
use super::interface_type::InterfaceType;

pub struct VideoSettings {
    /// The system's video interface type.
    pub interface_type: InterfaceType,

    /// Whether gamma dithering is enabled. Normally `true`, unless in
    /// "special effect" mode.
    pub is_gamma_dithering_enabled: bool,

    /// Whether gamma is enabled. Normally `true`, unless on MPEG/JPEG.
    pub is_gamma_enabled: bool,

    /// Whether divot is enabled. Normally `true` if anti-aliasing is on, unless
    /// decal lines are on.
    pub is_divot_enabled: bool,

    /// Whether the video interface is interlaced.
    pub is_interlaced: bool,

    /// The system's anti-aliasing mode.
    pub anti_aliasing_mode: AntiAliasingMode,
}

impl From<u32> for VideoSettings {
    fn from(val: u32) -> Self {
        Self {
            interface_type: InterfaceType::from(bytes::to_u32(0, 2, val)),
            is_gamma_dithering_enabled: bytes::to_bool(2, val),
            is_gamma_enabled: bytes::to_bool(3, val),
            is_divot_enabled: bytes::to_bool(4, val),
            is_interlaced: bytes::to_bool(6, val),
            anti_aliasing_mode: AntiAliasingMode::from(bytes::to_u32(8, 2, val)),
        }
    }
}

impl Into<u32> for VideoSettings {
    fn into(self) -> u32 {
        0x0000
            | bytes::from_u32(0, 2, self.interface_type.into())
            | bytes::from_bool(2, self.is_gamma_dithering_enabled)
            | bytes::from_bool(3, self.is_gamma_enabled)
            | bytes::from_bool(4, self.is_divot_enabled)
            | bytes::from_bool(6, self.is_interlaced)
            | bytes::from_u32(8, 2, self.anti_aliasing_mode.into())
    }
}
