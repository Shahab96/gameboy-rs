pub mod banks;
pub mod header;
pub mod mbc;

use wasm_bindgen::JsValue;

use self::mbc::mbc3::MBC3;
use self::mbc::AddressMapper;
use header::{CartridgeError, CartridgeHeader};

#[derive(Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub mbc: Box<dyn AddressMapper>,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Result<Self, CartridgeError> {
        let header = header::CartridgeHeader::new(data)?;
        let cartridge_mbc = match header.cartridge_type {
            0x0F..=0x13 => Box::new(MBC3::new(&header, data)),
            _ => {
                eprintln!("Sorry, only MBC3 and variants are supported at the moment, so that's what we'll use.");
                Box::new(MBC3::new(&header, data))
            }
        };

        Ok(Cartridge {
            header,
            mbc: cartridge_mbc,
        })
    }
}

impl Into<JsValue> for Cartridge {
    fn into(self) -> JsValue {
        let mut obj = JsValue::new_object();

        obj.set(&JsValue::from_str("header"), &self.header.into());
        obj.set(&JsValue::from_str("mbc"), &self.mbc.into());

        obj
    }
}
