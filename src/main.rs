#[macro_use]
extern crate lazy_static;

use macroquad::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use chip8::Chip8;
#[cfg(not(target_arch = "wasm32"))]
use roms::get_rom;
#[cfg(target_arch = "wasm32")]
use wasm::{BLOOM, CHIP};

pub mod chip8;
pub mod display;
pub mod keypad;
pub mod memory;
pub mod opcodes;
pub mod postprocessing;
pub mod roms;
pub mod shaders;
pub mod stack;
pub mod wasm;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_width: 1600,
        window_height: 800,
        ..Default::default()
    }
}

#[cfg(target_arch = "wasm32")]
#[macroquad::main(window_conf)]
async fn main() {
    {
        let mut chippy = CHIP.lock().unwrap();
        chippy.load_font(0x00);
        chippy.load(0x200, include_bytes!("roms/brix.ch8").to_vec());
    }

    loop {
        let fps = get_fps();

        unsafe {
            let mut chippy = CHIP.lock().unwrap();
            chippy.sync_cycle(fps);
            chippy.display.draw(BLOOM);
        }
        next_frame().await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[macroquad::main(window_conf)]
async fn main() {
    let mut chippy = Chip8::init(500);
    chippy.load_font(0x00);
    chippy.load(0x200, get_rom(&String::from("brix")).to_vec());

    loop {
        let fps = get_fps();

        chippy.sync_cycle(fps);
        chippy.display.draw(false);

        // draw_text(&format!("fps: {:?}", fps), 2.0, 20.0, 30.0, GREEN);
        next_frame().await;
    }
}
