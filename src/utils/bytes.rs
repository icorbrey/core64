pub fn to_u32(i: u32, len: u32, val: u32) -> u32 {
    let offset: u32 = 16 - i - len;
    let mask: u32 = 2_u32.pow(len) - 1;
    (val & (mask << offset)) >> offset
}

pub fn from_u32(i: u32, len: u32, val: u32) -> u32 {
    val << (16 - i - len)
}

pub fn to_bool(i: u32, val: u32) -> bool {
    to_u32(i, 1, val) != 0
}

pub fn from_bool(i: u32, val: bool) -> u32 {
    from_u32(i, 1, if val { 1 } else { 0 })
}
