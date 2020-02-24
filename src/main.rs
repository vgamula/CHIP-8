extern crate sdl2;

mod input_driver;
use input_driver::SdlInputDriver;

mod video_driver;
use video_driver::SdlVideoDriver;

mod chip8;

fn main() {
    let sdl = sdl2::init().unwrap();

    let input_driver = SdlInputDriver::new(&sdl);
    let video_driver = SdlVideoDriver::new(&sdl);

    let mut cpu = chip8::Chip8::new(video_driver, input_driver);
    println!("Lift off!");

    // cpu.run_disk("disks/MAZE".to_string());
    // cpu.run_disk("disks/PICTURE".to_string());
    cpu.run_disk("disks/PONG".to_string());
}
