extern crate sdl2;

use std::{thread, time};
use std::io::Read;
use std::fs::File;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const GFX_SIZE: usize = 64 * 32;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;


pub struct Chip8 {
    opcode: u16,

    memory: [u8; MEMORY_SIZE],
    V: [u8; REGISTERS_COUNT], // registers
    I: usize, // Index counter

    // pc: u16
    pc: usize,// program counter

    delay_timer: u8,
    sound_timer: u8,

    // gfx: [u8; GFX_SIZE],  //  Graphic memory (pixel state)
    gfx: [[u8; 64]; 32],
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
            gfx: [[0; 64]; 32],
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

    fn update_screen(&mut self) {
        if !self.draw_flag {
            return;
        }

        let scale: i32 = 10;
        for y in 0..32 {
            for x in 0..64 {
                let pos = y * 32 + x;
                let pixel = if self.gfx[y][x] == 1 { 255 } else { 0 };
                self.canvas.set_draw_color(Color::RGB(pixel, pixel, pixel));
                // refactor this mess
                let rect = Rect::new(
                    x as i32 * scale,
                    y as i32 * scale,
                    scale as u32,
                    scale as u32,
                );
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.canvas.present();

        self.draw_flag = false;
    }

    fn get_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }

    fn get_y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    fn get_n(&self) -> usize {
        (self.opcode & 0x000F) as usize
    }

    fn get_kk(&self) -> u8 {
        ((self.opcode & 0x00FF) as u8)
    }

    fn get_nnn(&self) -> u16 {
        (self.opcode & 0x0FFF)
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        match self.opcode & 0xF000 {
            0x0000 => self.handle_0XXX(),
            0x1000 => self.handle_1XXX(),
            0x3000 => self.handle_3XXX(),
            0x6000 => self.handle_6XXX(),
            0x7000 => self.handle_7XXX(),
            0xA000 => self.handle_AXXX(),
            0xC000 => self.handle_CXXX(),
            0xD000 => self.handle_DXXX(),
            _ => {
                panic!("Unhandled opcode {:x}", self.opcode);
            }
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        self.update_screen();
    }

    fn handle_0XXX(&mut self) {
        match self.opcode & 0x00EF {
            0x00E0 => self.handle_00E0(),
            0x00EE => self.handle_00EE(),
            _ => {
                panic!("Not handled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_00E0(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_flag = true;
        self.pc += 2;
    }

    fn handle_00EE(&mut self) {
        panic!("Not handled 00EE");
    }

    fn handle_1XXX(&mut self) {
        self.pc = self.get_nnn() as usize;
    }

    fn handle_3XXX(&mut self) {
        if self.V[self.get_x()] == self.get_kk() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_6XXX(&mut self) {
        self.V[self.get_x()] = self.get_kk();
        self.pc += 2
    }

    fn handle_7XXX(&mut self) {
        self.V[self.get_x()] += self.get_kk();
        self.pc += 2
    }

    fn handle_AXXX(&mut self) {
        self.I = self.get_nnn() as usize;
        self.pc += 2;
    }

    fn handle_CXXX(&mut self) {
        let randomized_value = rand::random::<u8>();
        self.V[self.get_x()] = randomized_value & self.get_kk();
        self.pc += 2;
    }

    fn handle_DXXX(&mut self) {
        self.V[0xF] = 0;
        let n = self.get_n();
        for byte in 0..n {
            let y = (self.V[self.get_y()] as usize + byte) % 32;
            for bit in 0..8 {
                let x = (self.V[self.get_x()] as usize + bit) % 64;
                let color = (self.memory[self.I + byte] >> (7 - bit)) & 1;
                self.V[0xF] |= color & self.gfx[y][x];
                self.gfx[y][x] ^= color;
            }
        }

        self.draw_flag = true;
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
    // cpu.load_game("disks/PICTURE".to_string());
    // cpu.test_draw();

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
