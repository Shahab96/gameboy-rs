use crate::emulator::{
    cpu::context::CpuContext,
    utils::io::{In8, Out8},
};

pub struct Memory {
    pub data: [u8; 0xFFFF],
}

impl In8<u16> for Memory {
    fn read<C: CpuContext>(&mut self, src: u16, _: &mut C) -> u8 {
        self.data[src as usize]
    }
}

impl Out8<u16> for Memory {
    fn write<C: CpuContext>(&mut self, src: u16, data: u8, _: &mut C) {
        self.data[src as usize] = data;
    }
}
