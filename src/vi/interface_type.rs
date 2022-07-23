use crate::utils::bytes;

pub enum InterfaceType {
    Blank,
    SixteenBit,
    ThirtyTwoBit,
}

impl From<u32> for InterfaceType {
    fn from(val: u32) -> Self {
        match bytes::from_u32(0, 2, val) {
            2 => Self::SixteenBit,
            3 => Self::ThirtyTwoBit,
            _ => Self::Blank,
        }
    }
}

impl Into<u32> for InterfaceType {
    fn into(self) -> u32 {
        let interface_type: u32 = match self {
            Self::Blank => 0,
            Self::SixteenBit => 2,
            Self::ThirtyTwoBit => 3,
        };
        bytes::from_u32(0, 2, interface_type)
    }
}
