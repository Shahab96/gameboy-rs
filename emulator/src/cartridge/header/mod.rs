use std::fmt::Display;

use thiserror::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Error, Debug)]
pub enum CartridgeError {
    #[error("Invalid cartridge size: {0}")]
    InvalidSize(usize),

    #[error("Invalid header checksum: expected {expected:?}, actual {actual:?}")]
    InvalidHeaderChecksum { expected: u8, actual: u8 },

    #[error("Invalid global checksum: expected {expected:?}, actual {actual:?}")]
    InvalidGlobalChecksum { expected: u16, actual: u16 },
}

impl Into<JsValue> for CartridgeError {
    fn into(self) -> JsValue {
        JsValue::from_str(self)
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct CartridgeHeader {
    pub entry: Box<[u8]>,
    pub title: String,
    pub manufacturer_code: String,
    pub cgb_flag: u8,
    pub new_licensee_code: String,
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_licensee_code: u8,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl CartridgeHeader {
    pub fn new(data: &[u8]) -> Result<CartridgeHeader, CartridgeError> {
        let size = data.len();

        if size < 0x8000 || size % 0x4000 != 0 {
            return Err(CartridgeError::InvalidSize(size));
        }

        let header_checksum = data[0x014D];
        let global_checksum = u16::from_be_bytes([data[0x014E], data[0x014F]]);

        Self::header_checksum(data, header_checksum)?;
        Self::global_checksum(data, global_checksum)?;

        Ok(CartridgeHeader {
            entry: Box::new([data[0x0100], data[0x0101], data[0x0102], data[0x0103]]),
            title: String::from_utf8(data[0x0134..0x0143].to_vec()).unwrap(),
            manufacturer_code: String::from_utf8(data[0x013F..0x0143].to_vec()).unwrap(),
            cgb_flag: data[0x0143],
            new_licensee_code: String::from_utf8(data[0x0144..0x0146].to_vec()).unwrap(),
            sgb_flag: data[0x0146],
            cartridge_type: data[0x0147],
            rom_size: data[0x0148],
            ram_size: data[0x0149],
            destination_code: data[0x014A],
            old_licensee_code: data[0x014B],
            mask_rom_version_number: data[0x014C],
            header_checksum,
            global_checksum,
        })
    }

    fn header_checksum(data: &[u8], checksum: u8) -> Result<(), CartridgeError> {
        let mut sum = 0u8;

        for i in 0x134..0x14D {
            sum = sum.wrapping_sub(data[i]).wrapping_sub(1);
        }

        if sum != checksum {
            return Err(CartridgeError::InvalidHeaderChecksum {
                expected: checksum,
                actual: sum,
            });
        }

        Ok(())
    }

    fn global_checksum(data: &[u8], checksum: u16) -> Result<(), CartridgeError> {
        let mut sum = 0u16;

        for i in 0..data.len() {
            if i == 0x14E || i == 0x14F {
                continue;
            }

            sum = sum.wrapping_add(data[i] as u16);
        }

        if sum != u16::from_be_bytes([data[0x014E], data[0x014F]]) {
            return Err(CartridgeError::InvalidGlobalChecksum {
                expected: checksum,
                actual: sum,
            });
        }

        Ok(())
    }
}

impl Display for CartridgeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CartridgeHeader{{\n\tEntry: {:02X?}\n\tTitle: {:?}\n\tManufacturer Code: {:?}\n\tCGB Flag: {:?}\n\tNew Licensee Code: {:?}\n\tSGB Flag: {:?}\n\tCartridge Type: {:?}\n\tROM Size: {:?}\n\tRAM Size: {:?}\n\tDestination Code: {:?}\n\tOld Licensee Code: {:?}\n\tMask ROM Version Number: {:?}\n\tHeader Checksum: {:?}\n\tGlobal Checksum: {:?}\n}}",
            self.entry,
            self.title,
            self.manufacturer_code,
            self.cgb_flag,
            self.new_licensee_code,
            self.sgb_flag,
            self.cartridge_type,
            self.rom_size,
            self.ram_size,
            self.destination_code,
            self.old_licensee_code,
            self.mask_rom_version_number,
            self.header_checksum,
            self.global_checksum,
        )
    }
}
