pub mod ram;
pub mod rom;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BankError {
    #[error("Address out of range: {0}")]
    AddressOutOfRange(usize),

    #[error("Invalid size: {0}")]
    InvalidSize(u8),
}

pub trait Bank {
    fn read(&self, address: u16) -> Result<u8, BankError>;
    fn swap_bank(&mut self, bank: u16);
}
