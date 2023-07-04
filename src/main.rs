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

    let cartridge = CartridgeHeader::load(rom_path)?;

    // println!("{}", cartridge.into);

    let mut cpu = CPU::new();
    cpu.load_cartridge(cartridge.into());

    // println!("{:?}", cpu);

    loop {
        cpu.step();
    }

    // Ok(())
}
