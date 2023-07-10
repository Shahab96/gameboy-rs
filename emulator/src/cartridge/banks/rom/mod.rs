use super::{Bank, BankError};

#[derive(Debug)]
pub struct RomBanks {
    active_bank: u16,
    pub banks: Vec<[u8; 0x4000]>,
}

impl RomBanks {
    pub fn new(rom_size_byte: u8) -> Result<Self, BankError> {
        if rom_size_byte > 0x08 {
            return Err(BankError::InvalidSize(rom_size_byte));
        }

        let num_banks: usize = 1 << rom_size_byte;

        Ok(Self {
            active_bank: 1,
            banks: vec![[0; 0x4000]; num_banks],
        })
    }
}

impl Bank for RomBanks {
    fn swap_bank(&mut self, bank: u16) {
        if bank < 1 || bank > self.banks.len() as u16 {
            self.active_bank = bank & (self.banks.len() as u16 - 1);
            dbg!("Invalid bank selected, wrapping to {}", self.active_bank);
        } else {
            self.active_bank = bank;
        }
    }

    fn read(&self, address: u16) -> Result<u8, BankError> {
        let address = address as usize;

        if address > 0x3FFF {
            return Err(BankError::AddressOutOfRange(address));
        }

        let bank = self.active_bank as usize;
        let data = self.banks[bank][address];

        Ok(data)
    }
}

impl From<&[u8]> for RomBanks {
    fn from(data: &[u8]) -> Self {
        let rom_size = data.len();
        let num_banks = rom_size / 0x4000;

        let mut banks = vec![[0xFF; 0x4000]; num_banks];

        for (i, byte) in data.iter().enumerate() {
            let bank = i / 0x4000;
            let address = i % 0x4000;

            banks[bank][address] = *byte;
        }

        Self {
            active_bank: 1,
            banks,
        }
    }
}
