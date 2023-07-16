pub mod mbc3;

use super::banks::BankError;

pub trait AddressMapper {
    fn read(&self, address: u16) -> Result<u8, BankError>;
    fn write(&mut self, address: u16, value: u8);
}
