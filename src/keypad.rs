use sdl2::keyboard::Keycode;

const KEYPAD_SIZE: usize = 16;

pub struct Keypad {
    keyboard: [bool; KEYPAD_SIZE],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keyboard: [false; KEYPAD_SIZE],
        }
    }

    fn set_key_state(&mut self, key: Keycode, pressed: bool) {
        match key {
            Keycode::Num1 => self.keyboard[0x1] = pressed,
            Keycode::Num2 => self.keyboard[0x2] = pressed,
            Keycode::Num3 => self.keyboard[0x3] = pressed,
            Keycode::Num4 => self.keyboard[0xC] = pressed,
            
            Keycode::Q => self.keyboard[0x4] = pressed,
            Keycode::W => self.keyboard[0x5] = pressed,
            Keycode::E => self.keyboard[0x6] = pressed,
            Keycode::R => self.keyboard[0xD] = pressed,
            
            Keycode::A => self.keyboard[0x7] = pressed,
            Keycode::S => self.keyboard[0x8] = pressed,
            Keycode::D => self.keyboard[0x9] = pressed,
            Keycode::F => self.keyboard[0xE] = pressed,

            Keycode::Z => self.keyboard[0xA] = pressed,
            Keycode::X => self.keyboard[0x0] = pressed,
            Keycode::C => self.keyboard[0xB] = pressed,
            Keycode::V => self.keyboard[0xF] = pressed,
            _ => ()
        }
    }

    pub fn press_key(&mut self, key: Keycode) {
        self.set_key_state(key, true)
    }

    pub fn unpress_key(&mut self, key: Keycode) {
        self.set_key_state(key, false)
    }

    pub fn is_pressed(&self, code: usize) -> bool {
        self.keyboard[code]
    }
}
