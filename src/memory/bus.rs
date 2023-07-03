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
}
