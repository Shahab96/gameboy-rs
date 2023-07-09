pub mod mbc3;

pub trait MBC {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
