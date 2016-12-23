pub fn make_word16(l: u8, h: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}
