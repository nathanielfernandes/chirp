use macroquad::rand;

use crate::{chip8::Chip8, display::Display};

pub enum PC {
    Next,
    Jump(u16),
    Skip(bool),
}

#[allow(non_snake_case)]
impl Chip8 {
    // clear screen
    pub fn exec_opcode(&mut self, opcode: u16) {
        let i = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match match (i, x, y, n) {
            (0x00, 0x00, 0x0E, 0x00) => self._00E0(), // clear screen
            (0x00, 0x00, 0x0E, 0x0E) => self._00EE(), // return from subroutine

            (0x01, _, _, _) => self._1NNN(nnn),     // jump
            (0x02, _, _, _) => self._2NNN(nnn),     // call subroutine at nnn
            (0x03, _, _, _) => self._3XNN(x, nn),   // skip if Vx == nn
            (0x04, _, _, _) => self._4XNN(x, nn),   // skip if Vx != nn
            (0x05, _, _, 0x00) => self._5XY0(x, y), // skip if Vx == Vy
            (0x06, _, _, _) => self._6XNN(x, nn),   // set register Vx to nn
            (0x07, _, _, _) => self._7XNN(x, nn),   // add value to register Vx

            (0x08, _, _, 0x00) => self._8XY0(x, y), // set Vx to Vy
            (0x08, _, _, 0x01) => self._8XY1(x, y), // set Vx to Vx | Vy
            (0x08, _, _, 0x02) => self._8XY2(x, y), // set Vx to Vx & Vy
            (0x08, _, _, 0x03) => self._8XY3(x, y), // set Vx to Vx ^ Vy
            (0x08, _, _, 0x04) => self._8XY4(x, y), // Add Vy to Vx
            (0x08, _, _, 0x05) => self._8XY5(x, y), // set Vx to Vx - Vy
            (0x08, _, _, 0x06) => self._8XY6(x, y), // shift right Vx by 1
            (0x08, _, _, 0x07) => self._8XY7(x, y), // set Vx to Vy - Vx
            (0x08, _, _, 0x0E) => self._8XYE(x, y), // shift left Vx by 1

            (0x09, _, _, 0x00) => self._9XY0(x, y), // skip if Vx != Vy

            (0x0A, _, _, _) => self._ANNN(nnn), // set index to register I
            (0x0B, _, _, _) => self._BNNN(nnn), // jump to v0 + nnn
            (0x0C, _, _, _) => self._CXNN(x, nn), // set Vx to a random number & nn
            (0x0D, _, _, _) => self._DXYN(x, y, n), // display/draw

            (0x0E, _, 0x09, 0x0E) => self._EX9E(x), // skip if key down
            (0x0E, _, 0x0A, 0x01) => self._EXA1(x), // skip if key not down

            (0x0F, _, 0x00, 0x07) => self._FX07(x), // set Vx to value of delay timer
            (0x0F, _, 0x01, 0x05) => self._FX15(x), // set delay timer to value of Vx
            (0x0F, _, 0x01, 0x08) => self._FX18(x), // set sound timer to value of Vx
            (0x0F, _, 0x01, 0x0E) => self._FX1E(x), // add Vx to I
            (0x0F, _, 0x00, 0x0A) => self._FX0A(x), // get key
            (0x0F, _, 0x02, 0x09) => self._FX29(x), // set I to be the font in Vx
            (0x0F, _, 0x03, 0x03) => self._FX33(x), // get each number place and store in memory
            (0x0F, _, 0x05, 0x05) => self._FX55(x), // store registers to memory
            (0x0F, _, 0x06, 0x05) => self._FX65(x), // load memory to registers

            _ => {
                println!("Unknown Opcode: {:#06X}", opcode);
                PC::Next
            }
        } {
            PC::Next => self.pc += 0x02,
            PC::Jump(nnn) => self.pc = nnn,
            PC::Skip(skip) => {
                if skip {
                    self.pc += 0x04
                } else {
                    self.pc += 0x02
                }
            }
        }
    }

    // Clear the display.
    fn _00E0(&mut self) -> PC {
        self.display.clear();
        PC::Next
    }

    // Jump to location nnn
    fn _1NNN(&mut self, nnn: u16) -> PC {
        PC::Jump(nnn)
    }

    // Set Vx = nn
    fn _6XNN(&mut self, x: usize, nn: u8) -> PC {
        self.v[x] = nn;
        PC::Next
    }

    // Add nn to Vx
    fn _7XNN(&mut self, x: usize, nn: u8) -> PC {
        self.v[x] = self.v[x].wrapping_add(nn);
        PC::Next
    }

    // Set I = nnn
    fn _ANNN(&mut self, nnn: u16) -> PC {
        self.i = nnn;
        PC::Next
    }

    // Display draw
    fn _DXYN(&mut self, x: usize, y: usize, n: u8) -> PC {
        let x = self.v[x] % Display::WIDTH;
        let mut y = self.v[y] % Display::HEIGHT;
        self.v[0x0F] = 0x00;

        for b in 0..n as u16 {
            let sprite_data = self.memory[(self.i + b) as usize];
            let mut x = x;
            for i in (0..8).rev() {
                if ((sprite_data >> i) & 1) != 0 {
                    let prev = self.display.get(x, y);
                    self.display.set(x, y, !prev);
                    if prev {
                        self.v[0x0F] = 0x01;
                    }
                }
                x += 1;
            }
            y += 1;
        }

        PC::Next
    }

    // Return from subroutine
    fn _00EE(&mut self) -> PC {
        PC::Jump(self.stack.pop())
    }

    // Call subroutine at nnn
    fn _2NNN(&mut self, nnn: u16) -> PC {
        // push the incremented pc so that, the next opcode is called instead of looping
        self.stack.push(self.pc + 0x02);
        PC::Jump(nnn)
    }

    // Skip if Vx == nn
    fn _3XNN(&mut self, x: usize, nn: u8) -> PC {
        PC::Skip(self.v[x] == nn)
    }

    // Skip if Vx != nn
    fn _4XNN(&mut self, x: usize, nn: u8) -> PC {
        PC::Skip(self.v[x] != nn)
    }

    // Skip if Vx == Vy
    fn _5XY0(&mut self, x: usize, y: usize) -> PC {
        PC::Skip(self.v[x] == self.v[y])
    }

    // Skip if Vx != Vy
    fn _9XY0(&mut self, x: usize, y: usize) -> PC {
        PC::Skip(self.v[x] != self.v[y])
    }

    // Set Vx to Vy
    fn _8XY0(&mut self, x: usize, y: usize) -> PC {
        self.v[x] = self.v[y];
        PC::Next
    }

    // Set Vx to Vx | Vy
    fn _8XY1(&mut self, x: usize, y: usize) -> PC {
        self.v[x] |= self.v[y];
        PC::Next
    }

    // Set Vx to Vx & Vy
    fn _8XY2(&mut self, x: usize, y: usize) -> PC {
        self.v[x] &= self.v[y];
        PC::Next
    }

    // Set Vx to Vx ^ Vy
    fn _8XY3(&mut self, x: usize, y: usize) -> PC {
        self.v[x] ^= self.v[y];
        PC::Next
    }

    // Add Vy to Vx
    fn _8XY4(&mut self, x: usize, y: usize) -> PC {
        let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = sum;
        self.v[0x0F] = overflow as u8;
        PC::Next
    }

    // Set Vx to Vx - Vy
    fn _8XY5(&mut self, x: usize, y: usize) -> PC {
        let (diff, overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = diff;
        self.v[0x0F] = !overflow as u8;
        PC::Next
    }

    // Set Vx to Vy - Vx
    fn _8XY7(&mut self, x: usize, y: usize) -> PC {
        let (diff, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = diff;
        self.v[0x0F] = !overflow as u8;
        PC::Next
    }

    // Shift right Vx by 1
    fn _8XY6(&mut self, x: usize, _y: usize) -> PC {
        self.v[0x0F] = self.v[x] & 1;
        self.v[x] >>= 1;
        PC::Next
    }

    // Shift left Vx by 1
    fn _8XYE(&mut self, x: usize, _y: usize) -> PC {
        self.v[0x0F] = (self.v[x] & 255) >> 7;
        self.v[x] <<= 1;
        PC::Next
    }

    // Jump to V0 + nnn
    fn _BNNN(&mut self, nnn: u16) -> PC {
        PC::Jump(self.v[0] as u16 + nnn)
    }

    // Set Vx to a random number & nn
    fn _CXNN(&mut self, x: usize, nn: u8) -> PC {
        self.v[x] = rand::gen_range(0, 255) & nn;
        PC::Next
    }

    // Skip if key (Vx) down
    fn _EX9E(&mut self, x: usize) -> PC {
        PC::Skip(self.keypad.is_key_down(self.v[x]))
    }

    // Skip if key (Vx) not down
    fn _EXA1(&mut self, x: usize) -> PC {
        PC::Skip(!self.keypad.is_key_down(self.v[x]))
    }

    // Set Vx to value of delay timer
    fn _FX07(&mut self, x: usize) -> PC {
        self.v[x] = self.delay_timer;
        PC::Next
    }

    // Set delay timer to value of Vx
    fn _FX15(&mut self, x: usize) -> PC {
        self.delay_timer = self.v[x];
        PC::Next
    }

    // Set sound timer to value of Vx
    fn _FX18(&mut self, x: usize) -> PC {
        self.sound_timer = self.v[x];
        PC::Next
    }

    // Add Vx to I
    fn _FX1E(&mut self, x: usize) -> PC {
        let (sum, overflow) = self.i.overflowing_add(self.v[x] as u16);
        self.i = sum;
        self.v[0x0F] = overflow as u8;
        PC::Next
    }

    // Get key
    fn _FX0A(&mut self, x: usize) -> PC {
        self.keypad.wait_for_key(x);
        PC::Next
    }

    // Set I to be the font in Vx
    fn _FX29(&mut self, x: usize) -> PC {
        self.i = (self.v[x] as u16) * 5; // *5 to get the last nibble
        PC::Next
    }

    // get each number place and store in memory
    fn _FX33(&mut self, x: usize) -> PC {
        let vx = self.v[x];
        self.memory[self.i] = vx / 100;
        self.memory[self.i + 1] = (vx / 10) % 10;
        self.memory[self.i + 2] = (vx % 100) % 10;

        PC::Next
    }

    // store registers to memory
    fn _FX55(&mut self, x: usize) -> PC {
        for i in 0..x + 1 {
            self.memory[self.i + (i as u16)] = self.v[i];
        }
        PC::Next
    }

    // load memory to registers
    fn _FX65(&mut self, x: usize) -> PC {
        for i in 0..x + 1 {
            self.v[i] = self.memory[self.i + (i as u16)];
        }
        PC::Next
    }
}
