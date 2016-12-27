pub fn make_word16(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

pub fn low_byte(v: u16) -> u8 {
    v as u8
}

pub fn high_byte(v: u16) -> u8 {
    (v >> 8) as u8
}
