use crate::utils::bytes;

pub enum AntiAliasingMode {
    AlwaysFetchLines,
    FetchLinesWhenNeeded,
    ResampleOnly,
    None,
}

impl From<u32> for AntiAliasingMode {
    fn from(val: u32) -> Self {
        match bytes::to_u32(8, 2, val) {
            0 => AntiAliasingMode::AlwaysFetchLines,
            1 => AntiAliasingMode::FetchLinesWhenNeeded,
            2 => AntiAliasingMode::ResampleOnly,
            _ => AntiAliasingMode::None,
        }
    }
}

impl Into<u32> for AntiAliasingMode {
    fn into(self) -> u32 {
        let anti_aliasing_mode: u32 = match self {
            Self::AlwaysFetchLines => 0,
            Self::FetchLinesWhenNeeded => 1,
            Self::ResampleOnly => 2,
            Self::None => 3,
        };
        bytes::from_u32(8, 2, anti_aliasing_mode)
    }
}
