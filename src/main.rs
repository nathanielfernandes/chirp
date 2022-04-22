#[macro_use]
extern crate lazy_static;

use chip8::Chip8;
use macroquad::prelude::*;
use std::{thread::sleep, time::Duration};

pub mod chip8;
pub mod display;
pub mod keypad;
pub mod memory;
pub mod opcodes;
pub mod postprocessing;
pub mod shaders;
pub mod stack;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_width: 1600,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let target_hz: i32 = 500;
    let mut chippy = Chip8::init();

    chippy.load_font(0x00);
    chippy.load(0x200, include_bytes!("roms/brix.ch8").to_vec());

    loop {
        let fps = get_fps();
        for _ in 0..(target_hz / fps) {
            chippy.cycle();
        }
        chippy.display.draw();

        draw_text(&format!("fps: {:?}", fps), 2.0, 20.0, 30.0, GREEN);
        next_frame().await;
    }
}
