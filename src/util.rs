use std::ops::{BitAnd, BitOr, Not, Shl};

pub fn make_word16(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

pub fn low_byte(v: u16) -> u8 {
    v as u8
}

pub fn high_byte(v: u16) -> u8 {
    (v >> 8) as u8
}

pub fn low_nibble(v: u8) -> u8 {
    v & 0x0f
}

pub fn high_nibble(v: u8) -> u8 {
    v >> 4
}

pub fn get_bit<T>(v: T, bit: u8) -> bool
    where T: Eq + From<u8> + BitAnd<T, Output = T> + Shl<u8, Output = T>
{
    v & (T::from(1) << bit) != T::from(0)
}

pub fn set_bit<T>(v: T, bit: u8, val: bool) -> T
    where T: From<u8> + BitAnd<T, Output = T> + BitOr<T, Output = T> + Not<Output = T> + Shl<u8, Output = T>
{
    if val {
        v | (T::from(1) << bit)
    } else {
        v & !(T::from(1) << bit)
    }
}

// Add two 8 bit numbers, returns the result, 3 bit and 7 bit carry flags
pub fn add8(a: u8, b: u8) -> (u8, bool, bool) {
    (a + b, low_nibble(a) + low_nibble(b) > 0x0f, a as u16 + b as u16 > 0xff)
}

// subtract two 8 bit numbers, returns the result, the borrow 4 bit flag and the borrow 8 bit flag.
pub fn sub8(a: u8, b: u8) -> (u8, bool, bool) {
    (a - b, low_nibble(b) > low_nibble(a), b > a)
}
