const MEMORY_SIZE: usize = 4096;
const REGISTERS_COUNT: usize = 16;
const GFX_SIZE: usize = 64 * 32;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;


pub struct CPU {
    memory: [u8; MEMORY_SIZE],
    opcode: u16,
    V: [u8; REGISTERS_COUNT],  // registers
    I: u16,  // Index counter
    pc: u16,  // program counter

    delay_timer: u8,
    sound_timer: u8,

    gfx: [u8; GFX_SIZE],  //  Graphic memory (pixel state)

    stack: [u16; STACK_SIZE],
    sp: u16,  // stack pointer

    key: [u8; KEYPAD_SIZE],  // pressed keys
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


impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
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
        };

        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        cpu
    }

    pub fn say_hello(&self) {
        println!("Hello, World!");
    }

    // pub fn load(&self, )
}
