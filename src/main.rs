extern crate sdl2;

mod input_driver;
use input_driver::InputDriver;

mod video_driver;
use video_driver::VideoDriver;

mod chip8;

fn main() {
    let sdl = sdl2::init().unwrap();
    
    let input_driver = InputDriver::new(&sdl);
    let video_driver = VideoDriver::new(&sdl);

    let mut cpu = chip8::Chip8::new(video_driver, input_driver);
    println!("Lift off!");
    
    // cpu.run_disk("disks/MAZE".to_string());
    // cpu.run_disk("disks/PICTURE".to_string());
    cpu.run_disk("disks/PONG".to_string());
}
