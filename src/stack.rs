pub struct Stack {
    stack: [u16; 16],
    pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0x00_u16; 16],
            pointer: 0,
        }
    }

    /* decerement stack pointer, and return value
     */
    pub fn pop(&mut self) -> u16 {
        self.pointer -= 1;
        self.stack[self.pointer]
    }
    /* set to current stack pointer then increment
     */
    pub fn push(&mut self, v: u16) {
        self.stack[self.pointer] = v;
        self.pointer += 1;
    }

    // fn is_empty(&self) -> bool {
    //     self.stack.is_empty()
    // }

    // fn peek(&self) -> Option<&u16> {
    //     self.stack.last()
    // }
}
