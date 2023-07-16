pub mod ram;
pub mod rom;

use thiserror::Error;
use wasm_bindgen::JsValue;

type MemoryBank = Box<[u8]>;

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

impl Into<JsValue> for BankError {
    fn into(self) -> JsValue {
        JsValue::from_str(self)
    }
}
