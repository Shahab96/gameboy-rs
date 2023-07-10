use super::super::super::{
    banks::{rom::RomBanks, Bank, BankError},
    header::CartridgeHeader,
    mbc::MemoryBankMapper,
};

#[derive(Debug)]
pub struct MBC3 {
    rom_banks: RomBanks,
}

impl MBC3 {
    pub fn new(cartridge_header: &CartridgeHeader, data: &[u8]) -> Self {
        let rom_banks = RomBanks::new(cartridge_header.rom_size).unwrap_or(RomBanks::from(data));

        MBC3 { rom_banks }
    }
}

impl MemoryBankMapper for MBC3 {
    fn read(&self, address: u16) -> Result<u8, BankError> {
        match address {
            0x0000..=0x7FFF => self.rom_banks.read(address),
            _ => Err(BankError::AddressOutOfRange(address as usize)),
        }
    }

    // This is a no-op for the base MBC3
    fn write(&mut self, _: u16, _: u8) {}
}
