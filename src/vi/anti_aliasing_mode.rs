pub enum AntiAliasingMode {
    /// Anti-aliasing and resampling. Always fetches extra lines.
    AlwaysFetchLines,

    /// Anti-aliasing and resampling. Only fetches extra lines when necessary.
    FetchLinesWhenNeeded,

    /// Resampling only. Treat all lines as fully covered.
    ResampleOnly,

    /// No anti-aliasing nor resampling. Replicates pixels, no interpolation.
    None,
}

impl From<u32> for AntiAliasingMode {
    fn from(val: u32) -> Self {
        match val {
            0 => AntiAliasingMode::AlwaysFetchLines,
            1 => AntiAliasingMode::FetchLinesWhenNeeded,
            2 => AntiAliasingMode::ResampleOnly,
            _ => AntiAliasingMode::None,
        }
    }
}

impl Into<u32> for AntiAliasingMode {
    fn into(self) -> u32 {
        match self {
            Self::AlwaysFetchLines => 0,
            Self::FetchLinesWhenNeeded => 1,
            Self::ResampleOnly => 2,
            Self::None => 3,
        }
    }
}
