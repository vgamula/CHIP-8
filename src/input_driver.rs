use sdl2::keyboard::Keycode;

const KEYPAD_SIZE: usize = 16;

pub enum EventProcessingState {
    Quit,
    Void,
}

pub struct InputDriver {
    keyboard: [bool; KEYPAD_SIZE],
    event_pump: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl: &sdl2::Sdl) -> InputDriver {
        let event_pump = sdl.event_pump().unwrap();

        InputDriver {
            keyboard: [false; KEYPAD_SIZE],
            event_pump,
        }
    }

    fn set_key_state(&mut self, key: Keycode, pressed: bool) {
        match key {
            Keycode::Num1 => self.keyboard[0x1] = pressed,
            Keycode::Num2 => self.keyboard[0x2] = pressed,
            Keycode::Num3 => self.keyboard[0x3] = pressed,
            Keycode::Num4 => self.keyboard[0xc] = pressed,
            
            Keycode::Q => self.keyboard[0x4] = pressed,
            Keycode::W => self.keyboard[0x5] = pressed,
            Keycode::E => self.keyboard[0x6] = pressed,
            Keycode::R => self.keyboard[0xd] = pressed,
            
            Keycode::A => self.keyboard[0x7] = pressed,
            Keycode::S => self.keyboard[0x8] = pressed,
            Keycode::D => self.keyboard[0x9] = pressed,
            Keycode::F => self.keyboard[0xe] = pressed,

            Keycode::Z => self.keyboard[0xa] = pressed,
            Keycode::X => self.keyboard[0x0] = pressed,
            Keycode::C => self.keyboard[0xb] = pressed,
            Keycode::V => self.keyboard[0xf] = pressed,
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

    pub fn process_events(&mut self) -> EventProcessingState {
        let iter: Vec<sdl2::event::Event> = self.event_pump.poll_iter().collect();

        for event in iter {
            match event {
                sdl2::event::Event::Quit { .. } => return EventProcessingState::Quit,
                sdl2::event::Event::KeyDown { keycode: Some(kc), .. } => self.press_key(kc),
                sdl2::event::Event::KeyUp { keycode: Some(kc), .. } => self.unpress_key(kc),
                _ => {}
            }
        }
        EventProcessingState::Void
    }
}
