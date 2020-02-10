extern crate sdl2;

use std::thread;
use std::time::Duration;
use std::io::Read;
use std::fs::File;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rand;

mod keypad;
use keypad::Keypad;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const GFX_WIDTH: usize = 64;
const GFX_HEIGHT: usize = 32;

pub struct VideoDriver {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    gfx: [[u8; GFX_WIDTH]; GFX_HEIGHT],
    draw_flag: bool,
}

impl VideoDriver {
    pub fn new(sdl: sdl2::Sdl) -> VideoDriver {
        let video_subsystem = sdl.video().unwrap();
        let _window = video_subsystem
            .window("CHIP-8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let event_pump = sdl.event_pump().unwrap();

        let canvas = _window.into_canvas().build().unwrap();
        VideoDriver {
            canvas,
            event_pump,
            gfx: [[0; GFX_WIDTH]; GFX_HEIGHT],
            draw_flag: false,
        }
    }

    pub fn update_screen(&mut self) {
        if !self.draw_flag {
            return;
        }

        let scale: i32 = 10;
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        let main_color = Color::RGB(255, 255, 255);
        self.canvas.set_draw_color(main_color);
        for y in 0..GFX_HEIGHT {
            for x in 0..GFX_WIDTH {
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

    pub fn reinitialize_screen(&mut self) {
        self.gfx = [[0; GFX_WIDTH]; GFX_HEIGHT];
        self.draw_flag = true;
    }

    pub fn pixel_state(&self, y: usize, x: usize) -> u8 {
        self.gfx[y][x]
    }

    pub fn toggle_pixel_state(&mut self, y: usize, x: usize, color: u8) {
        self.gfx[y][x] ^= color;
    }

    pub fn set_draw_flag(&mut self, flag: bool) {
        self.draw_flag = flag;
    }
}


pub struct Chip8 {
    opcode: u16,

    memory: [u8; MEMORY_SIZE],
    registers: [u8; REGISTERS_COUNT],
    index_counter: usize,

    pc: usize, // program counter

    delay_timer: u8,
    sound_timer: u8,

    stack: [usize; STACK_SIZE],
    stack_pointer: usize,

    pub keypad: Keypad,

    video_driver: VideoDriver,
}

const FONTSET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80  // F
];

impl Chip8 {
    pub fn new(video_driver: VideoDriver, keypad: Keypad) -> Chip8 {
        let mut cpu = Chip8 {
            memory: [0; MEMORY_SIZE],
            opcode: 0,
            registers: [0; REGISTERS_COUNT],
            index_counter: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            keypad,
            video_driver,
        };

        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

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

    fn get_x(&self) -> usize {
        ((self.opcode & 0x0f00) >> 8) as usize
    }

    fn get_y(&self) -> usize {
        ((self.opcode & 0x00f0) >> 4) as usize
    }

    fn get_n(&self) -> usize {
        (self.opcode & 0x000f) as usize
    }

    fn get_kk(&self) -> u8 {
        ((self.opcode & 0x00ff) as u8)
    }

    fn get_nnn(&self) -> u16 {
        (self.opcode & 0x0fff)
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        match self.opcode & 0xf000 {
            0x0000 => self.handle_0xxx(),
            0x1000 => self.handle_1xxx(),
            0x2000 => self.handle_2xxx(),
            0x3000 => self.handle_3xxx(),
            0x4000 => self.handle_4xxx(),
            0x6000 => self.handle_6xxx(),
            0x7000 => self.handle_7xxx(),
            0x8000 => self.handle_8xxx(),
            0xa000 => self.handle_axxx(),
            0xc000 => self.handle_cxxx(),
            0xd000 => self.handle_dxxx(),
            0xe000 => self.handle_exxx(),
            0xf000 => self.handle_fxxx(),
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

        self.video_driver.update_screen();
    }

    fn handle_0xxx(&mut self) {
        match self.opcode & 0x00ef {
            0x00e0 => self.handle_00e0(),
            0x00ee => self.handle_00ee(),
            _ => {
                panic!("Not handled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_00e0(&mut self) {
        self.video_driver.reinitialize_screen();
        self.pc += 2;
    }

    fn handle_00ee(&mut self) {
        self.stack_pointer -= 1;
        self.pc = self.stack[self.stack_pointer];
    }

    fn handle_1xxx(&mut self) {
        self.pc = self.get_nnn() as usize;
    }

    fn handle_2xxx(&mut self) {
        self.stack[self.stack_pointer] = self.pc + 2;
        self.stack_pointer += 1;
        self.pc = self.get_nnn() as usize;
    }

    fn handle_3xxx(&mut self) {
        if self.registers[self.get_x()] == self.get_kk() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_4xxx(&mut self) {
        if self.registers[self.get_x()] != self.get_kk() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_6xxx(&mut self) {
        self.registers[self.get_x()] = self.get_kk();
        self.pc += 2
    }

    fn handle_7xxx(&mut self) {
        let x = self.get_x();
        let vx = self.registers[x] as u16;
        let kk = self.get_kk() as u16;
        self.registers[x] = (vx + kk) as u8;
        self.pc += 2
    }

    fn handle_8xxx(&mut self) {
        match self.opcode & 0xf {
            0x0 => self.handle_8xx0(),
            0x2 => self.handle_8xx2(),
            0x4 => self.handle_8xx4(),
            0x5 => self.handle_8xx5(),
            _ => {
                panic!("Unhandled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_8xx0(&mut self) {
        self.registers[self.get_x()] = self.registers[self.get_y()];
        self.pc += 2;
    }

    fn handle_8xx2(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        self.registers[x] = self.registers[x] & self.registers[y];
        self.pc += 2;
    }

    fn handle_8xx4(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        let result = (self.registers[x] as u16) + (self.registers[y] as u16);
        self.registers[x] = (result & 0xff) as u8;
        self.registers[0xf] = if result > 0xff { 1 } else { 0 };
        self.pc += 2;
    }

    fn handle_8xx5(&mut self) {
        let x = self.get_x();
        let y = self.get_y();
        self.registers[0xf] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
        self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
        self.pc += 2;
    }

    fn handle_axxx(&mut self) {
        self.index_counter = self.get_nnn() as usize;
        self.pc += 2;
    }

    fn handle_cxxx(&mut self) {
        let randomized_value = rand::random::<u8>();
        self.registers[self.get_x()] = randomized_value & self.get_kk();
        self.pc += 2;
    }

    fn handle_dxxx(&mut self) {
        self.registers[0xf] = 0;
        let n = self.get_n();
        for byte in 0..n {
            let y = (self.registers[self.get_y()] as usize + byte) % GFX_HEIGHT;
            for bit in 0..8 {
                let x = (self.registers[self.get_x()] as usize + bit) % GFX_WIDTH;
                let color = (self.memory[self.index_counter + byte] >> (7 - bit)) & 1;
                self.registers[0xf] |= color & self.video_driver.pixel_state(y, x);
                self.video_driver.toggle_pixel_state(y, x, color);
            }
        }
        self.video_driver.set_draw_flag(true);
        self.pc += 2;
    }

    fn handle_exxx(&mut self) {
        match self.opcode & 0xff {
            0x9e => self.handle_ex9e(),
            0xa1 => self.handle_exa1(),
            _ => {
                panic!("Not handled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_ex9e(&mut self) {
        let key = self.registers[self.get_x()] as usize;
        if self.keypad.is_pressed(key) {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_exa1(&mut self) {
        let key = self.registers[self.get_x()] as usize;
        if !self.keypad.is_pressed(key) {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn handle_fxxx(&mut self) {
        match self.opcode & 0xff {
            0x07 => self.handle_fx07(),
            0x15 => self.handle_fx15(),
            0x18 => self.handle_fx18(),
            0x29 => self.handle_fx29(),
            0x33 => self.handle_fx33(),
            0x65 => self.handle_fx65(),
            _ => {
                panic!("Not handled opcode {:x}", self.opcode);
            }
        }
    }

    fn handle_fx07(&mut self) {
        self.registers[self.get_x()] = self.delay_timer;
        self.pc += 2;
    }

    fn handle_fx15(&mut self) {
        self.delay_timer = self.registers[self.get_x()];
        self.pc += 2;
    }

    fn handle_fx18(&mut self) {
        self.sound_timer = self.registers[self.get_x()];
        self.pc += 2;
    }

    fn handle_fx29(&mut self) {
        self.index_counter = self.registers[self.get_x()] as usize * 5;
        self.pc += 2;
    }

    fn handle_fx33(&mut self) {
        let x = self.get_x();
        self.memory[self.index_counter] = self.registers[x] / 100;
        self.memory[self.index_counter + 1] = (self.registers[x] % 100) / 10;
        self.memory[self.index_counter + 2] = self.registers[x] % 10;
        self.pc += 2;
    }

    fn handle_fx65(&mut self) {
        let x = self.get_x();
        for i in 0..x + 1 {
            self.registers[i] = self.memory[self.index_counter + i];
        }
        self.pc += 2;
    }

    fn run_disk(&mut self, disk: String) {
        self.load_game(disk);

        'main: loop {
            for event in self.video_driver.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::KeyDown { keycode: Some(kc), .. } => self.keypad.press_key(kc),
                    sdl2::event::Event::KeyUp { keycode: Some(kc), .. } => self.keypad.unpress_key(kc),
                    _ => {}
                }
            }
    
            self.emulate_cycle();
    
            thread::sleep(Duration::from_millis(1)); // don't be too fast
        }
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    

    let keypad = Keypad::new();


    let video_driver = VideoDriver::new(sdl);

    let mut cpu = Chip8::new(video_driver, keypad);
    // cpu.load_game("disks/MAZE".to_string());
    // cpu.load_game("disks/PICTURE".to_string());
    // cpu.load_game("disks/PONG".to_string());
    println!("Lift off!");

    cpu.run_disk("disks/PONG".to_string());
}
