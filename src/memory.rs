use std::ops::{Index, IndexMut};

pub struct Memory {
    mem: [u8; 4096],

    #[cfg(target_arch = "wasm32")]
    pub heatmap: [u8; 4096], // for visualization
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; 4096],
            #[cfg(target_arch = "wasm32")]
            heatmap: [0; 4096],
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn tick(&mut self) {
        for (i, m) in self.heatmap.into_iter().enumerate() {
            self.heatmap[i] = m.saturating_sub(1)
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get(&mut self, i: u16) -> u8 {
        let i = i as usize;
        self.heatmap[i] = self.heatmap[i].saturating_add(20);
        self.mem[i]
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set(&mut self, i: u16, v: u8) {
        let i = i as usize;
        self.heatmap[i] = 255;
        self.mem[i] = v;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&mut self, i: u16) -> u8 {
        let i = i as usize;
        self.mem[i]
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set(&mut self, i: u16, v: u8) {
        let i = i as usize;
        self.mem[i] = v;
    }
}

impl Index<&'_ u16> for Memory {
    type Output = u8;

    fn index(&self, pc: &'_ u16) -> &Self::Output {
        &self.mem[*pc as usize]
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, pc: u16) -> &Self::Output {
        &self.mem[pc as usize]
    }
}

impl IndexMut<&'_ u16> for Memory {
    fn index_mut(&mut self, pc: &'_ u16) -> &mut u8 {
        &mut self.mem[*pc as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, pc: u16) -> &mut u8 {
        &mut self.mem[pc as usize]
    }
}

// impl Index<&'_ usize> for Memory {
//     type Output = u8;

//     fn index(&self, pc: &'_ usize) -> &Self::Output {
//         &self.mem[*pc]
//     }
// }

// impl Index<usize> for Memory {
//     type Output = u8;

//     fn index(&self, pc: usize) -> &Self::Output {
//         &self.mem[pc]
//     }
// }

// impl IndexMut<&'_ usize> for Memory {
//     fn index_mut(&mut self, pc: &'_ usize) -> &mut u8 {
//         &mut self.mem[*pc]
//     }
// }

// impl IndexMut<usize> for Memory {
//     fn index_mut(&mut self, pc: usize) -> &mut u8 {
//         &mut self.mem[pc]
//     }
// }
