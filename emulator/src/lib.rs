mod cartridge;

use cartridge::header::CartridgeError;
use cartridge::Cartridge;

pub fn load_cartridge(data: Vec<u8>) -> Result<Cartridge, CartridgeError> {
    Cartridge::new(&data)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{io::Read, path::Path};

    use super::*;

    #[test]
    fn it_works() {
        let path = Path::new("../roms/01-special.gb").to_owned();
        let mut file = std::fs::File::open(path).unwrap();
        let mut data: Vec<u8> = vec![];

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
