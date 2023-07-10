pub mod base;

use super::super::banks::ram::RamBanks;
use super::super::banks::rom::RomBanks;
use super::super::banks::{Bank, BankError};
use super::super::header::CartridgeHeader;
use super::MemoryBankMapper;

#[derive(Debug)]
enum WriteMapping {
    RAM,
    RTC(usize),
    Disabled,
}

#[derive(Debug)]
pub struct MBC3 {
    rom_banks: RomBanks,
    ram_banks: RamBanks,
    write_mapping: WriteMapping,
    bank_or_rtc_select: u8,
    clock_registers: [u8; 5],
    latched_clock_registers: [u8; 5],
    active_clock_register: usize,
}

impl MBC3 {
    pub fn new(cartridge_header: &CartridgeHeader, data: &[u8]) -> Self {
        let rom_banks = RomBanks::new(cartridge_header.rom_size).unwrap_or(RomBanks::from(data));
        let ram_banks = RamBanks::new(cartridge_header.ram_size).unwrap();

        MBC3 {
            rom_banks,
            ram_banks,
            write_mapping: WriteMapping::Disabled,
            bank_or_rtc_select: 0,
            clock_registers: [0; 5],
            latched_clock_registers: [0; 5],
            active_clock_register: 0,
        }
    }
}

impl MemoryBankMapper for MBC3 {
    fn read(&self, address: u16) -> Result<u8, BankError> {
        match address {
            0x0000..=0x7FFF => self.rom_banks.read(address),
            0xA000..=0xBFFF => {
                if self.bank_or_rtc_select <= 0x03 {
                    self.ram_banks.read(address)
                } else {
                    Ok(self.clock_registers[self.active_clock_register])
                }
            }
            _ => panic!("Invalid address: {:#04}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => match value {
                0x00 => self.write_mapping = WriteMapping::Disabled,
                0x0A => self.write_mapping = WriteMapping::RAM,
                _ => panic!("Invalid value written to RAM Timer enable: {:#04}", value),
            },
            0x2000..=0x3FFF => {
                let bank = match value & 0x7F {
                    0x00 => 0x01,
                    value => value,
                };
                self.rom_banks.swap_bank(bank as u16);
            }
            0x4000..=0x5FFF => match value {
                0x00..=0x03 => {
                    self.ram_banks.swap_bank(value as u16);
                    self.write_mapping = WriteMapping::RAM;
                }
                0x08..=0x0C => {
                    self.active_clock_register = (value - 0x08) as usize;
                    self.write_mapping = WriteMapping::RTC(self.active_clock_register);
                }
                _ => panic!("Invalid value written to RAM bank select: {:#04}", value),
            },
            0x6000..=0x7FFF => self
                .latched_clock_registers
                .copy_from_slice(&self.clock_registers),
            0xA000..=0xBFFF => match self.write_mapping {
                WriteMapping::RAM => {
                    self.ram_banks.write(address, value).unwrap();
                }
                WriteMapping::RTC(register) => self.clock_registers[register] = value,
                WriteMapping::Disabled => (),
            },
            _ => panic!("Invalid address: {:#04}", address),
        }
    }
}
