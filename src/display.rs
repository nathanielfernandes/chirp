use macroquad::prelude::*;

type GfxBuffer = [bool; 64 * 32];

pub struct Display {
    pub buffer: GfxBuffer,
    pub width_ratio: f32,
    pub height_ratio: f32,
}

impl Display {
    pub const WIDTH: u8 = 64;
    pub const HEIGHT: u8 = 32;

    pub const WIDTH_F32: f32 = Self::WIDTH as f32;
    pub const HEIGHT_F32: f32 = Self::HEIGHT as f32;

    pub const CLEAR: GfxBuffer = [false; (Self::WIDTH as usize) * (Self::HEIGHT as usize)];

    pub fn new() -> Self {
        Self {
            buffer: Self::CLEAR,
            width_ratio: 0.0,
            height_ratio: 0.0,
        }
    }

    pub fn update_screen_size(&mut self) {
        self.width_ratio = screen_width() / Self::WIDTH_F32;
        self.height_ratio = screen_height() / Self::HEIGHT_F32;
    }

    pub fn clear(&mut self) {
        self.buffer = Self::CLEAR
    }

    #[inline(always)]
    pub fn i(x: u8, y: u8) -> usize {
        (x as usize) + (Self::WIDTH as usize) * (y as usize)
    }

    #[inline(always)]
    pub fn set(&mut self, x: u8, y: u8, b: bool) {
        self.buffer[Self::i(x, y)] = b
    }

    #[inline(always)]
    pub fn get(&self, x: u8, y: u8) -> bool {
        self.buffer[Self::i(x, y)]
    }

    pub fn get_u8(&self, x: u8, y: u8) -> u8 {
        if self.buffer[Self::i(x, y)] {
            1
        } else {
            0
        }
    }

    pub fn draw(&mut self) {
        self.update_screen_size();
        clear_background(BLACK);
        for y in 0..32 {
            for x in 0..64 {
                if self.get(x, y) {
                    draw_rectangle(
                        self.width_ratio * x as f32,
                        self.height_ratio * y as f32,
                        self.width_ratio as f32,
                        self.height_ratio as f32,
                        WHITE,
                    );
                }
            }
        }
    }
}
