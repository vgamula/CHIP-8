use sdl2::keyboard::Keycode;

const KEYPAD_SIZE: usize = 16;

pub enum EventProcessingState {
    Quit,
    Void,
}

pub trait InputDriver {
    fn press_key(&mut self, key: usize);
    fn unpress_key(&mut self, key: usize);
    fn is_pressed(&self, key: usize) -> bool;
    fn process_events(&mut self) -> EventProcessingState;
}

// Sdl input driver implementation

pub struct SdlInputDriver {
    keyboard: [bool; KEYPAD_SIZE],
    event_pump: sdl2::EventPump,
}

impl SdlInputDriver {
    pub fn new(sdl: &sdl2::Sdl) -> SdlInputDriver {
        let event_pump = sdl.event_pump().unwrap();

        SdlInputDriver {
            keyboard: [false; KEYPAD_SIZE],
            event_pump,
        }
    }

    fn set_key_state(&mut self, key: usize, pressed: bool) {
        if 1 <= key && key <= 0xf {
            self.keyboard[key] = pressed
        }
    }
}

fn sdl_keycode_to_chip8_keycode(key: Keycode) -> usize {
    match key {
        Keycode::Num1 => 0x1,
        Keycode::Num2 => 0x2,
        Keycode::Num3 => 0x3,
        Keycode::Num4 => 0xc,

        Keycode::Q => 0x4,
        Keycode::W => 0x5,
        Keycode::E => 0x6,
        Keycode::R => 0xd,

        Keycode::A => 0x7,
        Keycode::S => 0x8,
        Keycode::D => 0x9,
        Keycode::F => 0xe,

        Keycode::Z => 0xa,
        Keycode::X => 0x0,
        Keycode::C => 0xb,
        Keycode::V => 0xf,
        _ => 0,
    }
}

impl InputDriver for SdlInputDriver {
    fn press_key(&mut self, key: usize) {
        self.set_key_state(key, true);
    }

    fn unpress_key(&mut self, key: usize) {
        self.set_key_state(key, false);
    }

    fn is_pressed(&self, code: usize) -> bool {
        self.keyboard[code]
    }

    fn process_events(&mut self) -> EventProcessingState {
        let iter: Vec<sdl2::event::Event> = self.event_pump.poll_iter().collect();

        for event in iter {
            match event {
                sdl2::event::Event::Quit { .. } => return EventProcessingState::Quit,
                sdl2::event::Event::KeyDown {
                    keycode: Some(kc), ..
                } => self.press_key(sdl_keycode_to_chip8_keycode(kc)),
                sdl2::event::Event::KeyUp {
                    keycode: Some(kc), ..
                } => self.unpress_key(sdl_keycode_to_chip8_keycode(kc)),
                _ => {}
            }
        }
        EventProcessingState::Void
    }
}
