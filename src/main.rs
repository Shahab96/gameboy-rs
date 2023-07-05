#![allow(dead_code)]

use std::path::Path;

mod cartridge;
mod cpu;
mod memory;
mod utils;

use cartridge::header::{CartridgeError, CartridgeHeader};
use cpu::CPU;
use memory::bus::MemoryBus;
use utils::traits::Storage;

fn main() -> Result<(), Box<CartridgeError>> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            println!("Usage: {} <rom_path>", args[1]);
        }
        _ => panic!("Invalid number of arguments"),
    }

    let rom_path = Path::new(&args[1]);

    let cartridge: Vec<u8> = CartridgeHeader::load(rom_path)?.into();

    let mut memory_bus = MemoryBus::new();

    for (i, byte) in cartridge.iter().enumerate() {
        memory_bus.write(i, *byte);
    }

    let mut cpu = CPU::new(&mut memory_bus);

    loop {
        cpu.step();

        // // Uncomment to step through the program
        // std::io::stdin().read_line(&mut String::new()).unwrap();
    }
}
