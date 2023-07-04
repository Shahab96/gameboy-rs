use crate::utils::traits::Storage;

#[derive(Debug)]
pub struct MemoryBus {
    ram: [u8; 0xFFFF],
}

impl Storage<usize, u8> for MemoryBus {
    fn read(&mut self, src: usize) -> u8 {
        self.ram[src]
    }

    fn write(&mut self, dest: usize, value: u8) {
        self.ram[dest] = value;
    }
}

impl Storage<usize, u16> for MemoryBus {
    fn read(&mut self, src: usize) -> u16 {
        let lower: u8 = self.read(src);
        let upper: u8 = self.read(src + 1);

        u16::from_le_bytes([lower, upper])
    }

    fn write(&mut self, dest: usize, value: u16) {
        let [lower, upper] = value.to_le_bytes();

        self.write(dest, lower);
        self.write(dest + 1, upper);
    }
}

impl MemoryBus {
    pub fn new() -> Self {
        Self { ram: [0; 0xFFFF] }
    }
}
