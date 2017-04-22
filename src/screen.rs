#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Black,
    DarkGray,
    LightGray,
    White,
}

pub const HORIZONTAL_SCREEN_PIXELS: u8 = 160;
pub const VERTICAL_SCREEN_PIXELS: u8 = 144;

pub struct Screen([Pixel; HORIZONTAL_SCREEN_PIXELS as usize * VERTICAL_SCREEN_PIXELS as usize]);

impl Screen {
    pub fn new() -> Screen {
        Screen([Pixel::White; HORIZONTAL_SCREEN_PIXELS as usize * VERTICAL_SCREEN_PIXELS as usize])
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> Pixel {
        self.0[y as usize * HORIZONTAL_SCREEN_PIXELS as usize + x as usize]
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, p: Pixel) {
        self.0[y as usize * HORIZONTAL_SCREEN_PIXELS as usize + x as usize] = p;
    }
}
