use super::super::banks::ram::RamBanks;
use super::super::banks::rom::RomBanks;
use super::super::header::CartridgeHeader;
use super::MBC;

pub struct MBC3 {
    rom_banks: RomBanks,
    ram_banks: RamBanks,
    ram_timer_enabled: bool,
    bank_or_rtc_select: u8,
    clock_registers: [u8; 5],
    active_clock_register: usize,
}

impl MBC3 {
    pub fn new(cartridge_header: &CartridgeHeader, data: &[u8]) -> Self {
        let rom_banks = RomBanks::new(cartridge_header.rom_size).unwrap_or(RomBanks::from(data));
        let ram_banks = RamBanks::new(cartridge_header.ram_size).unwrap();

        MBC3 {
            rom_banks,
            ram_banks,
            ram_timer_enabled: false,
            bank_or_rtc_select: 0,
            clock_registers: [0; 5],
            active_clock_register: 0,
        }
    }
}

impl MBC for MBC3 {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom_banks.read(address),
            0xA000..=0xBFFF => {
                if self.bank_or_rtc_select <= 0x03 {
                    self.ram_banks.read(address)
                } else {
                    self.clock_registers[self.active_clock_register]
                }
            }
            _ => panic!("Invalid address: {:#04}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => match value {
                0x00 => self.ram_timer_enabled = false,
                0x0A => self.ram_timer_enabled = true,
                _ => panic!("Invalid value written to RAM Timer enable: {:#04}", value),
            },
            0x2000..=0x3FFF => {
                self.rom_banks
                    .swap_bank(value & 0x7F)
                    .expect("Invalid ROM bank selected");
            }
        }
    }
}
