use core::ptr::read_volatile;

/// Returns the data stored at the given address.
///
/// ## Arguments
///
/// * `src` - The address to read data from.
pub fn read<T>(src: *const T) -> T {
    unsafe { read_volatile(src) }
}

/// Writes data to the given address.
///
/// ## Arguments
///
/// * `dest` - The address to write data to.
/// * `val` - The value to write.
pub fn write<T>(dest: *mut T, val: T) {
    unsafe { *dest = val }
}
