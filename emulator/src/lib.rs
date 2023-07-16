mod cartridge;

use wasm_bindgen::prelude::wasm_bindgen;

use cartridge::header::CartridgeError;
use cartridge::Cartridge;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn load_cartridge(data: &[u8]) -> Result<Cartridge, CartridgeError> {
    Cartridge::new(&data)
}

#[cfg(test)]
mod tests {
    use std::{io::Read, path::Path};

    use super::*;

    #[test]
    fn it_works() {
        let path = Path::new("../roms/01-special.gb").to_owned();
        let mut file = std::fs::File::open(path).unwrap();
        let mut data: &[u8] = vec![].as_slice();

        file.read_to_end(&mut data).unwrap();

        let cartridge = match load_cartridge(data) {
            Ok(cartridge) => cartridge,
            Err(err) => panic!("Error loading cartridge: {:?}", err),
        };

        println!("{:?}", cartridge.header);
        println!("{:?}", cartridge.mbc);

        assert_eq!(1, 1)
    }
}
