use crate::utils::bytes;

use super::anti_aliasing_mode::AntiAliasingMode;
use super::interface_type::InterfaceType;

pub struct Status {
    interface_type: InterfaceType,
    is_gamma_dithering_enabled: bool,
    is_gamma_enabled: bool,
    is_divot_enabled: bool,
    is_serrated: bool,
    anti_aliasing_mode: AntiAliasingMode,
}

impl From<u32> for Status {
    fn from(value: u32) -> Self {
        Self {
            interface_type: InterfaceType::from(value),
            is_gamma_dithering_enabled: bytes::to_bool(2, value),
            is_gamma_enabled: bytes::to_bool(3, value),
            is_divot_enabled: bytes::to_bool(4, value),
            is_serrated: bytes::to_bool(6, value),
            anti_aliasing_mode: AntiAliasingMode::from(value),
        }
    }
}

impl Into<u32> for Status {
    fn into(self) -> u32 {
        let interface_type: u32 = self.interface_type.into();
        let anti_aliasing_mode: u32 = self.anti_aliasing_mode.into();
        0b0000_0000_0000_0000_u32
            | interface_type
            | bytes::from_bool(2, self.is_gamma_dithering_enabled)
            | bytes::from_bool(3, self.is_gamma_enabled)
            | bytes::from_bool(4, self.is_divot_enabled)
            | bytes::from_bool(6, self.is_serrated)
            | anti_aliasing_mode
    }
}
