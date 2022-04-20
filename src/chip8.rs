use crate::{display::Display, memory::Memory, stack::Stack};

pub struct Chip8 {
    pub i: u16,           // index register
    pub pc: u16,          // program counter
    pub opcode: u16,      // current opcode
    pub v: [u8; 16],      // v registers 0x0-0xE
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
        let opcode = self.fetch(self.pc);
        self.exec_opcode(opcode);
    }
}
