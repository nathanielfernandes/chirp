#![allow(non_upper_case_globals)]

pub type ROM = &'static [u8];

pub const Tetris: ROM = include_bytes!("roms/tetris.ch8");
pub const Brix: ROM = include_bytes!("roms/brix.ch8");
pub const Pong: ROM = include_bytes!("roms/pong2.ch8");
pub const KeyPad: ROM = include_bytes!("roms/keypad_test.ch8");
pub const Invaders: ROM = include_bytes!("roms/invaders.ch8");
pub const TicTacToe: ROM = include_bytes!("roms/tictactoe.ch8");
pub const IBMLogo: ROM = include_bytes!("roms/IBMLogo.ch8");
pub const Particles: ROM = include_bytes!("roms/particle.ch8");

pub const Test: ROM = include_bytes!("roms/test_opcode.ch8");

pub fn get_rom(name: &String) -> ROM {
    match name.as_str() {
        "tetris" => Tetris,
        "brix" => Brix,
        "pong" => Pong,
        "keypad" => KeyPad,
        "invaders" => Invaders,
        "tictactoe" => TicTacToe,
        "ibmlogo" => IBMLogo,
        "particles" => Particles,
        _ => Test,
    }
}
