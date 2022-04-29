#![cfg(target_arch = "wasm32")]

use crate::chip8::Chip8;
use crate::roms::get_rom;

use sapp_jsutils::JsObject;
use std::sync::Mutex;

const CHIP_HZ: i32 = 500;
pub static mut BLOOM: bool = false;

lazy_static! {
    pub static ref CHIP: Mutex<Chip8> = Mutex::new(Chip8::init(CHIP_HZ));
}

extern "C" {
    pub fn draw_memory(js_object: JsObject);
    pub fn draw_registers(js_object: JsObject);

}

#[no_mangle]
fn toggle_bloom() {
    unsafe { BLOOM = !BLOOM }
}

#[no_mangle]
fn load_rom(js_object: JsObject) {
    let mut name = String::new();
    js_object.to_string(&mut name);

    let rom = get_rom(&name);
    let mut chippy = CHIP.lock().unwrap();

    chippy.reset();
    chippy.load_font(0x00);
    chippy.load(0x200, rom.to_vec());
}

#[no_mangle]
fn update_hz(js_object: JsObject) {
    let new_hz = js_object.field_u32("new_hz");
    CHIP.lock().unwrap().set_hz(new_hz as i32);
}

impl Chip8 {
    pub fn send_state(&mut self) {
        unsafe {
            self.memory.tick(); // for visual heatmap
            draw_memory(JsObject::buffer(&self.memory.heatmap));
            draw_registers(JsObject::buffer(&self.v));
        }
    }
}
