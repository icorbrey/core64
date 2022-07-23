pub enum VideoMode {
    PAL,
    NTSC,
    MPAL,
}

impl From<u32> for VideoMode {
    fn from(val: u32) -> Self {
        match val {
            0 => Self::PAL,
            1 => Self::NTSC,
            _ => Self::MPAL,
        }
    }
}
