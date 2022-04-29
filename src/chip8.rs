use crate::{display::Display, keypad::KeyPad, memory::Memory, stack::Stack};

pub struct Chip8 {
    hz: i32,   // assumed frequency of cpu
    tick: i32, // current tick

    pub i: u16,           // index register
    pub pc: u16,          // program counter
    pub delay_timer: u8,  // delay timer
    pub sound_timer: u8,  // sound timer
    pub v: [u8; 16],      // v registers 0x0-0xE
    pub keypad: KeyPad,   // keypad
    pub display: Display, // display buffer
    pub stack: Stack,     // 16 frame stack
    pub memory: Memory,   /* Memory Map
                           0x000-0x1FF - Chip 8 interpreter
                           0x00-0x50 - Used for the built in 4x5 pixel font set (0-F)
                           0x200-0xFFF - Program ROM and work RAM
                          */
}

impl Chip8 {
    pub fn init(hz: i32) -> Self {
        Self {
            hz,
            tick: 0,
            i: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            v: [0x0; 16],
            keypad: KeyPad::new(),
            display: Display::new(),
            stack: Stack::new(),
            memory: Memory::new(),
        }
    }

    pub fn set_hz(&mut self, hz: i32) {
        self.hz = hz.max(60);
    }

    pub fn reset(&mut self) {
        self.tick = 0;
        self.i = 0;
        self.pc = 0x200;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.v = [0; 16];
        self.keypad = KeyPad::new();
        self.display = Display::new();
        self.memory = Memory::new();
    }

    pub fn fetch(&mut self, pc: u16) -> u16 {
        ((self.memory.get(pc) as u16) << 8) | (self.memory.get(pc + 1) as u16)
    }

    pub fn load(&mut self, pos: u16, bytes: Vec<u8>) {
        for (i, &b) in bytes.iter().enumerate() {
            self.memory.set(pos + (i as u16), b);
        }
    }

    pub fn tick_timers(&mut self) {
        self.tick += 1;
        if self.tick % (self.hz / 60) == 0 {
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);

            #[cfg(target_arch = "wasm32")]
            self.send_state(); // for front-end
        }

        if self.tick >= self.hz {
            self.tick = 0;
        }
    }

    pub fn cycle(&mut self) {
        self.tick_timers();

        if self.keypad.waiting {
            if let Some((key, dest)) = self.keypad.get_key() {
                self.v[dest] = key;
            }
        } else {
            let opcode = self.fetch(self.pc);
            self.exec_opcode(opcode);
        }

        // if self.sound_timer > 0 {
        //     // play a beep sound
        // }
    }

    pub fn sync_cycle(&mut self, fps: i32) {
        for _ in 0..((self.hz as f32 / fps.max(1) as f32).round() as i32) {
            self.cycle();
        }
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
            self.memory.set(pos + (i as u16), f)
        }
    }
}
