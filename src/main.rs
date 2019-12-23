extern crate sdl2;

use std::{thread, time};
use std::io::Read;
use std::fs::File;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand::random;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const GFX_SIZE: usize = 64 * 32;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;


pub struct Chip8 {
    opcode: u16,

    memory: [u8; MEMORY_SIZE],
    V: [u8; REGISTERS_COUNT], // registers
    I: u16, // Index counter

    // pc: u16
    pc: usize,// program counter

    delay_timer: u8,
    sound_timer: u8,

    gfx: [u8; GFX_SIZE],  //  Graphic memory (pixel state)
    draw_flag: bool,

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
            draw_flag: false,
            stack: [0; STACK_SIZE],
            sp: 0,
            key: [0; KEYPAD_SIZE],
            canvas,
        };

        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }
        cpu.canvas.set_draw_color(Color::RGB(255, 255, 255));

        cpu
    }

    pub fn load_game(&mut self, game_name: String) {
        let mut f = File::open(game_name).unwrap();

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        for i in 0..buffer.len() {
            self.memory[0x200 + i] = buffer[i];
        }
    }

    pub fn test_draw(&mut self) {
        self.canvas.fill_rect(Rect::new(30, 30, 6, 100)).unwrap();
        self.canvas.present();
    }

    pub fn get_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }

    pub fn get_kk(&self) -> u8 {
        ((self.opcode & 0x00FF) as u8)
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        match self.opcode & 0xF000 {
            0x3000 => self.handle_3XXX(),
            0xA000 => self.handle_AXXX(),
            0xC000 => self.handle_CXXX(),
            0xD000 => self.handle_DXXX(),
            _ => {
                panic!("Unhandled opcode {:x}", self.opcode);
            }
        }
    }

    pub fn handle_3XXX(&mut self) {
        if self.V[self.get_x()] == self.get_kk() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    pub fn handle_AXXX(&mut self) {
        self.I = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    pub fn handle_CXXX(&mut self) {
        let randomized_value = rand::random::<u8>();
        self.V[self.get_x()] = randomized_value & self.get_kk();
        self.pc += 2;
    }

    pub fn handle_DXXX(&mut self) {

        self.pc += 2;
    }
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

    let canvas = _window.into_canvas().build().unwrap();

    let mut cpu = Chip8::new(canvas);
    cpu.load_game("disks/MAZE".to_string());
    cpu.test_draw();

    println!("Lift off!");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                // handle key press
                _ => {
                    // println!("{:?}", event);
                }
            }
        }

        cpu.emulate_cycle();

        thread::sleep(time::Duration::from_millis(16));
    }
}
