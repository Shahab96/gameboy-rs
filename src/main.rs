#![allow(dead_code)]

use std::path::Path;

use cartridge::header::CartridgeError;

use crate::cartridge::header::CartridgeHeader;

mod cartridge;
mod cpu;

fn main() -> Result<(), Box<CartridgeError>> {
    let args: Vec<String> = std::env::args().collect();

    let rom_path = Path::new(&args[1]);

    let cartridge = CartridgeHeader::load(rom_path)?;

    println!("{}", cartridge);

    Ok(())
}
