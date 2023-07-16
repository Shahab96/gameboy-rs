use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use super::super::banks::ram::RamBanks;
use super::super::banks::rom::RomBanks;
use super::super::banks::BankError;
use super::super::header::CartridgeHeader;
use super::AddressMapper;

#[derive(Debug)]
enum WriteMapping {
    Disabled,
    Enabled(usize),
}

impl Into<JsValue> for WriteMapping {
    fn into(self) -> JsValue {
        match self {
            WriteMapping::Disabled => JsValue::from_str("Disabled"),
            WriteMapping::Enabled(map_state) => {
                JsValue::from_str(&format!("Enabled({})", map_state))
            }
        }
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct MBC3 {
    rom_banks: RomBanks,
    ram_banks: Option<RamBanks>,
    battery: bool,
    rtc: bool,
    write_mapping: WriteMapping,
    bank_or_rtc_select: u8,
    clock_registers: Box<[u8]>,
    latched_clock_registers: Box<[u8]>,
    active_clock_register: usize,
}

impl MBC3 {
    pub fn new(cartridge_header: &CartridgeHeader, data: &[u8]) -> Self {
        let rom_banks = RomBanks::new(cartridge_header.rom_size).unwrap_or(RomBanks::from(data));
        let ram_banks = match cartridge_header.cartridge_type {
            0x10 | 0x12 | 0x13 => Some(RamBanks::new(cartridge_header.ram_size).unwrap()),
            _ => None,
        };
        let battery = match cartridge_header.cartridge_type {
            0x0F | 0x10 | 0x13 => true,
            _ => false,
        };
        let rtc = match cartridge_header.cartridge_type {
            0x10 | 0x13 => true,
            _ => false,
        };

        MBC3 {
            rom_banks,
            ram_banks,
            battery,
            rtc,
            write_mapping: WriteMapping::Disabled,
            bank_or_rtc_select: 0,
            clock_registers: [0; 5],
            latched_clock_registers: [0; 5],
            active_clock_register: 0,
        }
    }

    pub fn generate_save_file(&self) -> Option<Vec<Vec<u8>>> {
        if self.battery && self.ram_banks.is_some() {
            let banks = &self.ram_banks.as_ref().unwrap().banks;

            let mut save_file = vec![vec![0; banks.len()]];
            for (i, bank) in banks.iter().enumerate() {
                save_file[i].clone_from_slice(bank);
            }

            return Some(save_file);
        }

        None
    }
}

impl AddressMapper for MBC3 {
    fn read(&self, address: u16) -> Result<u8, BankError> {
        match address {
            0x0000..=0x7FFF => self.rom_banks.read(address),
            0xA000..=0xBFFF => {
                if self.bank_or_rtc_select <= 0x03 && self.ram_banks.is_some() {
                    self.ram_banks.as_ref().unwrap().read(address)
                } else if self.rtc {
                    Ok(self.clock_registers[self.active_clock_register])
                } else {
                    Err(BankError::AddressOutOfRange(address as usize))
                }
            }
            _ => panic!("Invalid address: {:#04}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => match value {
                0x00 => self.write_mapping = WriteMapping::Disabled,
                0x0A => self.write_mapping = WriteMapping::Enabled(0xFF),
                _ => {
                    dbg!("Invalid value written to RAM Timer enable: {:#04}", value);
                }
            },
            0x2000..=0x3FFF => {
                let bank = match value & 0x7F {
                    0x00 => 0x01,
                    value => value,
                };
                self.rom_banks.swap_bank(bank as u16);
            }
            0x6000..=0x7FFF => {
                if self.rtc && value == 0x01 {
                    self.latched_clock_registers
                        .copy_from_slice(&self.clock_registers);
                }
            }
            _ => match self.write_mapping {
                WriteMapping::Enabled(0x00..=0x03) => match address {
                    0xA000..=0xBFFF => {
                        if self.ram_banks.is_some() {
                            self.ram_banks
                                .as_mut()
                                .unwrap()
                                .write(address, value)
                                .unwrap();
                        }
                    }
                    _ => {
                        dbg!("Invalid RAM write address: {:#04}", address);
                    }
                },
                WriteMapping::Enabled(0x08..=0x0C) => match address {
                    0xA000..=0xBFFF => {
                        if self.rtc {
                            self.clock_registers[self.active_clock_register] = value;
                        }
                    }
                    _ => {
                        dbg!("Invalid RTC write address: {:#04}", address);
                    }
                },
                WriteMapping::Enabled(_) => match address {
                    0x4000..=0x5FFF => match value {
                        0x00..=0x03 => {
                            if self.ram_banks.is_some() {
                                self.ram_banks.as_mut().unwrap().swap_bank(value as u16);
                                self.write_mapping = WriteMapping::Enabled(value as usize);
                            }
                        }
                        0x08..=0x0C => {
                            if self.rtc {
                                self.active_clock_register = (value - 0x08) as usize;
                                self.write_mapping = WriteMapping::Enabled(value as usize);
                            }
                        }
                        _ => {
                            dbg!(
                                "Invalid value written to RTC/RAM Bank select: {:#04}",
                                value
                            );
                        }
                    },
                    _ => {
                        dbg!(
                            "Attempted RAM/RTC write without bank selection to address: {:#04}",
                            address
                        );
                    }
                },
                WriteMapping::Disabled => {
                    dbg!("Write attempted to disabled RAM/RTC: {:#04}", address);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::Cartridge;
    use std::{io::Read, path::Path};

    const ROM_PATH: &str = "../../../../roms/01-special.gb";

    fn setup() -> MBC3 {
        let path = Path::new(ROM_PATH).to_owned();
        let mut file = std::fs::File::open(path).unwrap();
        let mut data: Vec<u8> = vec![];

        file.read_to_end(&mut data).unwrap();

        let cartridge = match Cartridge::new(&data) {
            Ok(cartridge) => cartridge,
            Err(err) => panic!("Error loading cartridge: {:?}", err),
        };

        MBC3::new(&cartridge.header, &data)
    }

    #[test]
    fn read_no_mapping() {
        let mbc = setup();
    }
}
