use std::result;
use std::error;
use std::ops::{BitAnd, BitOr, Not, Shl};

pub type Error = Box<error::Error>;
pub type Result<T> = result::Result<T, Error>;

pub fn make_word8(h: u8, l: u8) -> u8 {
    (h << 4) | l
}

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
    (a.wrapping_add(b), (a & 0x0f) + (b & 0x0f) > 0x0f, a as u16 + b as u16 > 0xff)
}

// subtract two 8 bit numbers, returns the result, the borrow 4 bit flag and the borrow 8 bit flag.
pub fn sub8(a: u8, b: u8) -> (u8, bool, bool) {
    (a.wrapping_sub(b), low_nibble(b) > low_nibble(a), b > a)
}

// Add two 16 bit numbers, returns the result, 3 bit, 7 bit, 11 bit, and 15 bit carry flags
pub fn add16(a: u16, b: u16) -> (u16, bool, bool, bool, bool) {
    (a.wrapping_add(b),
     (a & 0x0f) + (b & 0x0f) > 0x0f,
     (a & 0xff) + (b & 0xff) > 0xff,
     (a & 0x0fff) + (b & 0x0fff) > 0x0fff,
     a as u32 + b as u32 > 0xffff)
}
