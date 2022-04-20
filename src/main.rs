use chip8::Chip8;
use macroquad::prelude::*;
use std::{thread::sleep, time::Duration};

pub mod chip8;
pub mod display;
pub mod memory;
pub mod opcodes;
pub mod stack;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_width: 1000,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tickrate = 24.0;
    let mut chippy = Chip8::init();
    chippy.load(0x200, include_bytes!("IBMLogo.ch8").to_vec());

    loop {
        chippy.cycle();
        sleep(Duration::from_secs_f64(1.0 / tickrate as f64));

        chippy.display.draw();

        next_frame().await;
    }

    // println!("{:#06X}", chippy.fetch(0x200_u16));
}
