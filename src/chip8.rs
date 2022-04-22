use crate::{display::Display, keypad::KeyPad, memory::Memory, stack::Stack};

pub struct Chip8 {
    pub i: u16,           // index register
    pub pc: u16,          // program counter
    pub opcode: u16,      // current opcode
    pub v: [u8; 16],      // v registers 0x0-0xE
    pub delay_timer: u8,  // delay timer
    pub sound_timer: u8,  // sound timer
    pub keypad: KeyPad,   // keypad
    pub display: Display, // display buffer
    pub stack: Stack,     // 16 frame stack
    pub memory: Memory,   /* Memory Map
                           0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
                           0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
                           0x200-0xFFF - Program ROM and work RAM
                          */
}

impl Chip8 {
    pub fn init() -> Self {
        Self {
            i: 0,
            pc: 0x200,
            opcode: 0,
            v: [0x0; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad: KeyPad::new(),
            display: Display::new(),
            stack: Stack::new(),
            memory: Memory::new(),
        }
    }

    pub fn fetch(&self, pc: u16) -> u16 {
        ((self.memory[pc] as u16) << 8) | (self.memory[pc + 1] as u16)
    }

    pub fn load(&mut self, pos: u16, bytes: Vec<u8>) {
        for (i, &b) in bytes.iter().enumerate() {
            self.memory[pos + (i as u16)] = b;
        }
    }

    pub fn cycle(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);

        if let Some((key, dest)) = self.keypad.wait_cycle() {
            self.v[dest] = key;
        } else {
            let opcode = self.fetch(self.pc);
            self.exec_opcode(opcode);
        }

        // if self.sound_timer > 0 {
        //     // play a beep sound
        // }
    }

    const FONT: [u8; 80] = [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    pub fn load_font(&mut self, pos: u16) {
        for (i, &f) in Self::FONT.iter().enumerate() {
            self.memory[pos + (i as u16)] = f
        }
    }
}
