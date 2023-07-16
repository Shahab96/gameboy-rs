use wasm_bindgen::JsValue;

use super::{BankError, MemoryBank};

#[derive(Debug)]
pub struct RamBanks {
    active_bank: u8,
    pub banks: Box<[MemoryBank]>,
}

impl RamBanks {
    pub fn new(ram_size_byte: u8) -> Result<Self, BankError> {
        if ram_size_byte > 0x05 {
            return Err(BankError::InvalidSize(ram_size_byte));
        }

        match ram_size_byte {
            0x00 | 0x01 => Ok(Self {
                active_bank: 0,
                banks: vec![vec![0; 0x2000]; 0],
            }),
            0x02 => Ok(Self {
                active_bank: 1,
                banks: vec![vec![0; 0x2000]; 1],
            }),
            0x03 => Ok(Self {
                active_bank: 1,
                banks: vec![vec![0; 0x2000]; 4],
            }),
            0x04 => Ok(Self {
                active_bank: 1,
                banks: vec![vec![0; 0x2000]; 16],
            }),
            0x05 => Ok(Self {
                active_bank: 1,
                banks: vec![vec![0; 0x2000]; 8],
            }),
            _ => Err(BankError::InvalidSize(ram_size_byte)),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) -> Result<(), BankError> {
        let address = address as usize;

        if address > 0x1FFF {
            return Err(BankError::AddressOutOfRange(address));
        }

        let bank = self.active_bank as usize;
        self.banks[bank][address] = data;

        Ok(())
    }

    pub fn read(&self, address: u16) -> Result<u8, BankError> {
        let address = address as usize;

        if address > 0x1FFF {
            return Err(BankError::AddressOutOfRange(address));
        }

        let bank = self.active_bank as usize;
        let data = self.banks[bank][address];

        Ok(data)
    }

    pub fn swap_bank(&mut self, bank: u16) {
        if bank < 1 || bank > self.banks.len() as u16 {
            self.active_bank = (bank & (self.banks.len() as u16 - 1)) as u8;
            dbg!("Invalid bank selected, wrapping to {}", self.active_bank);
        } else {
            self.active_bank = bank as u8;
        }
    }
}

impl Into<JsValue> for RamBanks {
    fn into(self) -> JsValue {
        match serde_wasm_bindgen::to_value(&self) {
            Ok(value) => value,
            Err(err) => {
                dbg!(err);
                JsValue::UNDEFINED
            }
        }
    }
}
