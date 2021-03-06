use macroquad::prelude::*;

use crate::{postprocessing::GfxPipeline, shaders::Bloom}; // ChromaticAberration, GaussianBlur

type GfxBuffer = [bool; 64 * 32];

pub struct Display {
    pub buffer: GfxBuffer,
    width_ratio: f32,
    height_ratio: f32,
    post_processing: GfxPipeline<1>, // never ended up using this
}

impl Display {
    pub const WIDTH: u8 = 64;
    pub const HEIGHT: u8 = 32;

    pub const WIDTH_F32: f32 = Self::WIDTH as f32;
    pub const HEIGHT_F32: f32 = Self::HEIGHT as f32;

    pub const LENGTH: usize = (Self::WIDTH as usize) * (Self::HEIGHT as usize);

    pub const CLEAR: GfxBuffer = [false; Self::LENGTH];

    pub fn new() -> Self {
        let width = screen_width();
        let height = screen_height();
        Self {
            buffer: Self::CLEAR,
            width_ratio: screen_width() / Self::WIDTH_F32,
            height_ratio: screen_height() / Self::HEIGHT_F32,
            post_processing: GfxPipeline::new(width, height, &[(*Bloom, true)]),
        }
    }

    pub fn update_screen_size(&mut self) {
        let width = screen_width();
        let height = screen_height();

        let new_wr = width / Self::WIDTH_F32;
        let new_hr = height / Self::HEIGHT_F32;

        if new_wr != self.width_ratio || new_hr != self.height_ratio {
            self.width_ratio = new_wr;
            self.height_ratio = new_hr;
            self.post_processing.update_dimensions(width, height);
        }
    }

    pub fn clear(&mut self) {
        self.buffer = Self::CLEAR;
        // for i in 0..Self::LENGTH {
        //     self.buffer[i] = false;
        // }
    }

    #[inline(always)]
    pub fn i(x: u8, y: u8) -> usize {
        let mut x = x;
        if x >= Self::WIDTH {
            x -= Self::WIDTH;
        }

        let mut y = y;
        if y >= Self::HEIGHT {
            y -= Self::HEIGHT;
        }

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

    pub fn draw(&mut self, post: bool) {
        self.update_screen_size();

        const DRAW_COLOR: Color = Color {
            r: 1.0,
            g: 0.4,
            b: 0.78823525,
            a: 1.0,
        };

        let draw = &|| {
            for y in 0..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    if self.get(x, y) {
                        draw_rectangle(
                            self.width_ratio * x as f32,
                            self.height_ratio * y as f32,
                            self.width_ratio as f32,
                            self.height_ratio as f32,
                            DRAW_COLOR,
                        );
                    }
                }
            }
        };

        if post {
            self.post_processing.pipe(&draw);
        } else {
            draw()
        }
    }
}
