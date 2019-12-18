extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod lib;
use chip8::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.say_hello();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("CHIP-8", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut canvas = _window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    canvas.fill_rect(Rect::new(30, 30, 6, 100)).unwrap();
    canvas.present();

    println!("Lift off!");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                // handle key press
                _ => {
                    // println!("{:?}", event);
                },
            }
        }

        // render game here
    }

}
