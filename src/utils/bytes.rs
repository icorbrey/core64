/// Extracts data from the given value.
///
/// ## Arguments
///
/// * `i` - The index at which the data starts.
/// * `len` - The length of the data.
/// * `val` - The value which contains the data.
///
/// ## Examples
///
/// ```
/// use core64::utils::bytes;
/// assert_eq!(bytes::to_u32(0, 4, 0xABCD_0000), 0xA);
/// assert_eq!(bytes::to_u32(4, 4, 0xABCD_0000), 0xB);
/// assert_eq!(bytes::to_u32(8, 4, 0xABCD_0000), 0xC);
/// assert_eq!(bytes::to_u32(12, 4, 0xABCD_0000), 0xD);
/// ```
pub fn to_u32(i: u32, len: u32, val: u32) -> u32 {
    let offset: u32 = 32 - (len + i);
    let mask: u32 = (2_u32.pow(len) - 1) << offset;
    (val & mask) >> offset
}

/// Injects data into a u32 integer.
///
/// ## Arguments
///
/// * `i` - The index at which the data starts.
/// * `len` - The length of the data.
/// * `val` - The data to inject.
///
/// ## Examples
///
/// ```
/// use core64::utils::bytes;
/// assert_eq!(bytes::from_u32(0, 4, 0xA), 0xA000_0000);
/// assert_eq!(bytes::from_u32(4, 4, 0xB), 0x0B00_0000);
/// assert_eq!(bytes::from_u32(8, 4, 0xC), 0x00C0_0000);
/// assert_eq!(bytes::from_u32(12, 4, 0xD), 0x000D_0000);
/// ```
pub fn from_u32(i: u32, len: u32, val: u32) -> u32 {
    let mask: u32 = 2_u32.pow(len) - 1;
    (val & mask) << (32 - (i + len))
}

/// Extracts a boolean from the given value.
///
/// ## Arguments
///
/// * `i` - The index at which the boolean lives.
/// * `val` - The value which contains the boolean.
///
/// ## Examples
///
/// ```
/// use core64::utils::bytes;
/// assert!(bytes::to_bool(3, 0x1000_0000));
/// assert!(bytes::to_bool(7, 0x0100_0000));
/// assert!(bytes::to_bool(11, 0x0010_0000));
/// assert!(bytes::to_bool(15, 0x0001_0000));
/// ```
pub fn to_bool(i: u32, val: u32) -> bool {
    to_u32(i, 1, val) != 0
}

/// Injects a boolean into a u32 integer.
///
/// ## Arguments
///
/// * `i` - The index to inject the boolean at.
/// * `val` - The boolean to inject.
///
/// ## Examples
///
/// ```
/// use core64::utils::bytes;
/// assert_eq!(bytes::from_bool(3, true), 0x1000_0000);
/// assert_eq!(bytes::from_bool(7, true), 0x0100_0000);
/// assert_eq!(bytes::from_bool(11, true), 0x0010_0000);
/// assert_eq!(bytes::from_bool(15, true), 0x0001_0000);
/// ```
pub fn from_bool(i: u32, val: bool) -> u32 {
    from_u32(i, 1, if val { 1 } else { 0 })
}
