use crate::emulator::cpu::context::CpuContext;

pub trait In8<T> {
    fn read<C: CpuContext>(&mut self, src: T, ctx: &mut C) -> u8;
}

pub trait Out8<T> {
    fn write<C: CpuContext>(&mut self, src: T, data: u8, ctx: &mut C);
}
