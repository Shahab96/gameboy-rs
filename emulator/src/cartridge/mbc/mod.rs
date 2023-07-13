use super::banks::BankError;
use enum_dispatch::enum_dispatch;

pub mod mbc3;

#[enum_dispatch]
pub trait AddressMapper {
    fn read(&self, address: u16) -> Result<u8, BankError>;
    fn write(&mut self, address: u16, value: u8);
}

#[enum_dispatch(AddressMapper)]
#[derive(Debug)]
pub enum MBC {
    MBC3(mbc3::MBC3),
}
