use core::ptr::read_volatile;

pub fn read<T>(src: *const T) -> T {
    unsafe { read_volatile(src) }
}

pub fn write<T>(dest: *mut T, value: T) {
    unsafe { *dest = value }
}
