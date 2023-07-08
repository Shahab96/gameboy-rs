use super::{cpu::CPU, register::Flags};

pub struct ALU;

impl ALU {
    pub fn add_8(&self, a: u8, b: u8, caller: &mut CPU) -> u8 {
        let (result, overflow) = a.overflowing_add(b);

        let z = result == 0;
        let h = (a & 0xF) + (b & 0xF) > 0xF;
        let c = overflow;

        caller.registers.set_flags(Flags { z, n: false, h, c });

        result
    }

    pub fn sub_8(&self, a: u8, b: u8, caller: &mut CPU) -> u8 {
        let (result, overflow) = a.overflowing_sub(b);

        let z = result == 0;
        let h = (a & 0xF) < (b & 0xF);
        let c = overflow;

        caller.registers.set_flags(Flags { z, n: true, h, c });

        result
    }
}
