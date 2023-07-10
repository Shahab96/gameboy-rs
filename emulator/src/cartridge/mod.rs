pub mod banks;
pub mod header;
pub mod mbc;

use header::{CartridgeError, CartridgeHeader};
use mbc::mbc3::MBC3;
use mbc::MBC;

#[derive(Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub mbc: MBC,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Result<Self, CartridgeError> {
        let header = header::CartridgeHeader::new(data)?;
        let cartridge_mbc = match header.cartridge_type {
            0x0F..=0x13 => MBC::MBC3(MBC3::new(&header, data)),
            _ => {
                eprintln!("Sorry, only MBC3 and variants are supported at the moment, so that's what we'll use.");
                MBC::MBC3(MBC3::new(&header, data))
            }
        };

        Ok(Cartridge {
            header,
            mbc: cartridge_mbc,
        })
    }
}
