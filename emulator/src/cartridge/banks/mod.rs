pub mod ram;
pub mod rom;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BankError {
    #[error("Invalid bank: {0}")]
    InvalidBank(u16),

    #[error("Address out of range: {0}")]
    AddressOutOfRange(usize),

    #[error("Invalid size: {0}")]
    InvalidSize(u8),
}

trait Bank {
    fn read(&self, address: u16) -> Result<u8, BankError>;
    fn swap_bank(&mut self, bank: u16) -> Result<(), BankError>;
}
