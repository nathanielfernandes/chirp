use crate::{chip8::Chip8, display::Display};

pub enum PC {
    Next,
    Jump(u16),
    Skip,
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
            (0x01, _, _, _) => self._1NNN(nnn),       // jump
            (0x06, _, _, _) => self._6XNN(x, nn),     // set register Vx
            (0x07, _, _, _) => self._7XNN(x, nn),     // add value to register Vx
            (0x0A, _, _, _) => self._ANNN(nnn),       // set index to register I
            (0x0D, _, _, _) => self._DXYN(x, y, n),   // display/draw
            _ => PC::Next,
        } {
            PC::Next => self.pc += 0x02,
            PC::Jump(nnn) => self.pc = nnn,
            PC::Skip => self.pc += 0x04,
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
        self.v[x] = ((self.v[x] as u16) + (nn as u16)) as u8;
        PC::Next
    }

    // Set I = nnn
    fn _ANNN(&mut self, nnn: u16) -> PC {
        self.i = nnn;
        PC::Next
    }

    // Display draw
    fn _DXYN(&mut self, x: usize, y: usize, n: u8) -> PC {
        let x = self.v[x] & (Display::WIDTH - 1);
        let mut y = self.v[y] & (Display::HEIGHT - 1);
        self.v[0x0F] = 0x00;

        for i in 0..n as u16 {
            let mut x = x;
            let sprite_data = self.memory[self.i + i];
            for b in (0_u8..8_u8).rev() {
                let bit = (sprite_data >> b) & 1;
                // set v15 if a pixel got flipped
                self.v[0x0F] |= bit & self.display.get_u8(x, y);
                self.display.set(x, y, bit != 0);
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

    // // Call subroutine at nnn
    // fn _2NNN(&mut self, nnn: u16) ->
}
