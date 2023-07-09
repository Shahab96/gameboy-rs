pub mod banks;
pub mod header;
pub mod mbc;

use self::header::{CartridgeError, CartridgeHeader};
use self::mbc::mbc3::MBC3;
use self::mbc::MBC;

pub struct Cartridge {
    pub header: CartridgeHeader,
    pub mbc: MBC,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Result<Self, CartridgeError> {
        let header = header::CartridgeHeader::new(data)?;
        let mbc = match header.cartridge_type {
            0x0F..=0x13 => MBC::MBC3(MBC3::new(&header, data)),
            _ => unimplemented!("Sorry, only MBC3 and variants are supported at the moment."),
        };

        Ok(Cartridge { header, mbc })
    }
}
