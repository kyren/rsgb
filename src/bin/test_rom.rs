extern crate image;
extern crate rsgb;

use std::env;
use std::io::Read;
use std::fs::File;
use image::{ImageBuffer, Luma};

use rsgb::emulator::*;
use rsgb::screen::*;

fn main() {
    let mut args = env::args();
    args.next();
    let rom_filename = args.next().expect("no rom argument given");
    let step_count = args.next()
        .expect("no step count given")
        .parse()
        .expect("could not parse step count");
    let image_filename = args.next().expect("no output image name given");

    let mut rom_file = File::open(&rom_filename).expect("could not open rom file");

    let mut rom = Vec::new();
    rom_file
        .read_to_end(&mut rom)
        .expect("could not read rom");

    let mut emulator = Emulator::load_rom(&rom).expect("could not load rom");
    for i in 0..step_count {
        emulator
            .step()
            .expect(&format!("emulation error at step {}", i));
    }

    let screen = emulator.get_screen();
    let image = ImageBuffer::from_fn(HORIZONTAL_SCREEN_PIXELS as u32,
                                     VERTICAL_SCREEN_PIXELS as u32,
                                     |x, y| match screen.get_pixel(x as u8, y as u8) {
                                         Pixel::Black => Luma([0u8]),
                                         Pixel::DarkGray => Luma([70u8]),
                                         Pixel::LightGray => Luma([150u8]),
                                         Pixel::White => Luma([255u8]),
                                     });

    image
        .save(&image_filename)
        .expect("could not write image");
}
