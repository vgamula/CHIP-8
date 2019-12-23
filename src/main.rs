extern crate sdl2;

use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const GFX_SIZE: usize = 64 * 32;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;


pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    opcode: u16,
    V: [u8; REGISTERS_COUNT],
    // registers
    I: u16,
    // Index counter
    pc: u16,  // program counter

    delay_timer: u8,
    sound_timer: u8,

    gfx: [u8; GFX_SIZE],  //  Graphic memory (pixel state)

    stack: [u16; STACK_SIZE],
    sp: u16,  // stack pointer

    key: [u8; KEYPAD_SIZE],  // pressed keys

    canvas: Canvas<Window>,
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Chip8 {
    pub fn new(canvas: Canvas<Window>) -> Chip8 {
        let mut cpu = Chip8 {
            memory: [0; MEMORY_SIZE],
            opcode: 0,
            V: [0; REGISTERS_COUNT],
            I: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            gfx: [0; GFX_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
            key: [0; KEYPAD_SIZE],
            canvas,
        };

        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        cpu
    }

    pub fn test_draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.fill_rect(Rect::new(30, 30, 6, 100)).unwrap();
        self.canvas.present();
    }

    pub fn say_hello(&self) {
        println!("Hello, World!");
    }

    // pub fn load(&self, )
}

fn main() {


    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("CHIP-8", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let mut canvas = _window.into_canvas().build().unwrap();

    let mut cpu = Chip8::new(canvas);
    cpu.say_hello();

    cpu.test_draw();

    println!("Lift off!");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                // handle key press
                _ => {
                    println!("{:?}", event);
                }
            }
        }

        thread::sleep(time::Duration::from_millis(50));
        println!("Test");

        
        // render game here
    }
}
