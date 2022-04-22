use macroquad::prelude::{is_key_down, KeyCode};

pub struct KeyPad {
    pub waiting: bool,        // is the keypad waiting for an input
    pub dest_register: usize, // destination v register
}

impl KeyPad {
    pub fn new() -> Self {
        Self {
            waiting: false,
            dest_register: 0,
        }
    }

    const KEYPAD_MAP: [(KeyCode, u8); 16] = [
        (KeyCode::X, 0x00),
        (KeyCode::Key1, 0x01),
        (KeyCode::Key2, 0x02),
        (KeyCode::Key3, 0x03),
        (KeyCode::Q, 0x04),
        (KeyCode::W, 0x05),
        (KeyCode::E, 0x06),
        (KeyCode::A, 0x07),
        (KeyCode::S, 0x08),
        (KeyCode::D, 0x09),
        (KeyCode::Z, 0x0A),
        (KeyCode::C, 0x0B),
        (KeyCode::Key4, 0x0C),
        (KeyCode::R, 0x0D),
        (KeyCode::F, 0x0E),
        (KeyCode::V, 0x0F),
    ];
    pub fn get_key(&mut self) -> Option<(u8, usize)> {
        for (key_code, value) in Self::KEYPAD_MAP {
            if is_key_down(key_code) {
                self.waiting = false;
                return Some((value, self.dest_register));
            }
        }
        None
    }

    pub fn wait_for_key(&mut self, dest: usize) {
        self.waiting = true;
        self.dest_register = dest;
    }

    pub fn is_key_down(&self, x: u8) -> bool {
        is_key_down(Self::KEYPAD_MAP[x as usize].0)
    }
}
