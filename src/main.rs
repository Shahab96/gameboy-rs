#![allow(dead_code)]

use std::path::Path;

use cartridge::header::CartridgeError;

use crate::cartridge::header::CartridgeHeader;

mod cartridge;
mod cpu;
mod memory;

use cpu::CPU;

fn main() -> Result<(), Box<CartridgeError>> {
    let args: Vec<String> = std::env::args().collect();

    let rom_path = Path::new(&args[1]);

    let cartridge: Vec<u8> = CartridgeHeader::load(rom_path)?.into();

    let mut cpu = CPU::new();

    for (i, byte) in cartridge.iter().enumerate() {
        cpu.bus.write_byte(i as u16, *byte);
    }

    loop {
        cpu.step();

        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
}
