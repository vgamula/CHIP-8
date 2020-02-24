extern crate sdl2;

use std::env;

mod input_driver;
use input_driver::SdlInputDriver;

mod video_driver;
use video_driver::SdlVideoDriver;

mod chip8;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Required path to chip8 disk");
    }

    let sdl = sdl2::init().unwrap();

    let input_driver = SdlInputDriver::new(&sdl);
    let video_driver = SdlVideoDriver::new(&sdl);

    let mut cpu = chip8::Chip8::new(video_driver, input_driver);

    cpu.run_disk(&args[1]);
}
