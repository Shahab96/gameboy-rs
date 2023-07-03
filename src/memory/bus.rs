pub struct MemoryBus {
    ram: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self { ram: [0u8; 0xFFFF] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn read_word(&self, lower: u16, upper: u16) -> u16 {
        let lower = self.read_byte(lower);
        let upper = self.read_byte(upper);

        ((upper as u16) << 8) | (lower as u16)
    }
}
