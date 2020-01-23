extern crate sdl2;

use std::{thread, time};
use std::io::Read;
use std::fs::File;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use rand;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
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

    stack: [usize; STACK_SIZE],
    sp: usize,  // stack pointer

    key: [bool; KEYPAD_SIZE],  // pressed keys

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
            key: [false; KEYPAD_SIZE],
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
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(Rect::new(0, 0, 640, 320)).unwrap();
        let main_color = Color::RGB(255, 255, 255);
        self.canvas.set_draw_color(main_color);
        for y in 0..32 {
            for x in 0..64 {
                if self.gfx[y][x] != 1 {
                    continue;
                }
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
            0x2000 => self.handle_2XXX(),
            0x3000 => self.handle_3XXX(),
            0x4000 => self.handle_4XXX(),
            0x6000 => self.handle_6XXX(),
            0x7000 => self.handle_7XXX(),
            0x8000 => self.handle_8XXX(),
            0xA000 => self.handle_AXXX(),
            0xC000 => self.handle_CXXX(),
            0xD000 => self.handle_DXXX(),
            0xE000 => self.handle_EXXX(),
            0xF000 => self.handle_FXXX(),
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

        // println!("{:?}", self.delay_timer);
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
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn handle_1XXX(&mut self) {
        self.pc = self.get_nnn() as usize;
    }

    fn handle_2XXX(&mut self) {
        self.stack[self.sp] = self.pc + 2;
        self.sp += 1;
        self.pc = self.get_nnn() as usize;
    }

    fn handle_3XXX(&mut self) {
        if self.V[self.get_x()] == self.get_kk() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_4XXX(&mut self) {
        if self.V[self.get_x()] != self.get_kk() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_6XXX(&mut self) {
        self.V[self.get_x()] = self.get_kk();
        self.pc += 2
    }

    fn handle_7XXX(&mut self) {
        let x = self.get_x();
        let vx = self.V[x] as u16;
        let kk = self.get_kk() as u16;
        self.V[x] = (vx + kk) as u8;
        self.pc += 2
    }

    fn handle_8XXX(&mut self) {
        match self.opcode & 0xF {
            0x0 => self.handle_8XX0(),
            0x2 => self.handle_8XX2(),
            0x4 => self.handle_8XX4(),
            0x5 => self.handle_8XX5(),
            _ => {
                panic!("Unhandled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_8XX0(&mut self) {
        self.V[self.get_x()] = self.V[self.get_y()];
        self.pc += 2;
    }

    fn handle_8XX2(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        self.V[x] = self.V[x] & self.V[y];
        self.pc += 2;
    }

    fn handle_8XX4(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        let result = (self.V[x] as u16) + (self.V[y] as u16);
        self.V[x] = (result & 0xFF) as u8;
        self.V[0xF] = if result > 0xFF { 1 } else { 0 };
        self.pc += 2;
    }

    fn handle_8XX5(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        self.V[0xF] = if self.V[x] > self.V[y] { 1 } else { 0 };
        self.V[x] = self.V[x].wrapping_sub(self.V[y]);
        self.pc += 2;
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

    fn handle_EXXX(&mut self) {
        match self.opcode & 0xFF {
            0x9E => self.handle_EX9E(),
            0xA1 => self.handle_EXA1(),
            _ => {
                panic!("Not handled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_EX9E(&mut self) {
        if self.key[self.V[self.get_x()] as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_EXA1(&mut self) {
        if !self.key[self.V[self.get_x()] as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_FXXX(&mut self) {
        match self.opcode & 0xFF {
            0x07 => self.handle_FX07(),
            0x15 => self.handle_FX15(),
            0x18 => self.handle_FX18(),
            0x29 => self.handle_FX29(),
            0x33 => self.handle_FX33(),
            0x65 => self.handle_FX65(),
            _ => {
                panic!("Not handled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_FX07(&mut self) {
        self.V[self.get_x()] = self.delay_timer;
        self.pc += 2;
    }

    fn handle_FX15(&mut self) {
        self.delay_timer = self.V[self.get_x()];
        self.pc += 2;
    }

    fn handle_FX18(&mut self) {
        self.sound_timer = self.V[self.get_x()];
        self.pc += 2;
    }

    fn handle_FX29(&mut self) {
        self.I = self.V[self.get_x()] as usize * 5;
        self.pc += 2;
    }

    fn handle_FX33(&mut self) {
        let x = self.get_x();
        self.memory[self.I] = self.V[x] / 100;
        self.memory[self.I + 1] = (self.V[x] % 100) / 10;
        self.memory[self.I + 2] = self.V[x] % 10;
        self.pc += 2;
    }

    fn handle_FX65(&mut self) {
        let x = self.get_x();
        for i in 0..x + 1 {
            self.V[i] = self.memory[self.I + i];
        }
        self.pc += 2;
    }

    fn set_key_state(&mut self, key: Keycode, pressed: bool) {
        match key {
            Keycode::Num1 => self.key[0x1] = pressed,
            Keycode::Num2 => self.key[0x2] = pressed,
            Keycode::Num3 => self.key[0x3] = pressed,
            Keycode::Num4 => self.key[0xC] = pressed,
            
            Keycode::Q => self.key[0x4] = pressed,
            Keycode::W => self.key[0x5] = pressed,
            Keycode::E => self.key[0x6] = pressed,
            Keycode::R => self.key[0xD] = pressed,
            
            Keycode::A => self.key[0x7] = pressed,
            Keycode::S => self.key[0x8] = pressed,
            Keycode::D => self.key[0x9] = pressed,
            Keycode::F => self.key[0xE] = pressed,

            Keycode::Z => self.key[0xA] = pressed,
            Keycode::X => self.key[0x0] = pressed,
            Keycode::C => self.key[0xB] = pressed,
            Keycode::V => self.key[0xF] = pressed,
            _ => ()
        }
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
    // cpu.load_game("disks/MAZE".to_string());
    // cpu.load_game("disks/PICTURE".to_string());
    cpu.load_game("disks/PONG".to_string());
    // cpu.test_draw();

    println!("Lift off!");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(kc), .. } => cpu.set_key_state(kc, true),
                sdl2::event::Event::KeyUp { keycode: Some(kc), .. } => cpu.set_key_state(kc, false),
                // handle key press
                _ => {
                    // println!("{:?}", event);
                }
            }
        }

        cpu.emulate_cycle();

        // thread::sleep(time::Duration::from_millis(8));
    }
}
