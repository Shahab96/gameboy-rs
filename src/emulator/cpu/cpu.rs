use crate::emulator::{
    cpu::context::CpuContext,
    utils::io::{In8, Out8},
};

use super::{
    instruction::{Addr, Immediate8},
    register::{Reg16, Reg8, Registers},
};

pub struct CPU {
    pub registers: Registers,
    pub ime: bool,
    pub next_instruction: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            ime: false,
            next_instruction: 0,
        }
    }

    pub fn next_byte<C: CpuContext>(&mut self, ctx: &mut C) -> u8 {
        let pc = self.registers.pc;
        self.registers.pc = self.registers.pc.wrapping_add(1);

        ctx.read_cycle(pc)
    }

    pub fn push<C: CpuContext>(&mut self, value: u16, ctx: &mut C) {
        let [low, high] = value.to_le_bytes();
        ctx.tick_cycle();

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        ctx.write_cycle(self.registers.sp, high);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        ctx.write_cycle(self.registers.sp, low);
    }

    pub fn pop<C: CpuContext>(&mut self, ctx: &mut C) -> u16 {
        let low = ctx.read_cycle(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let high = ctx.read_cycle(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        u16::from_le_bytes([low, high])
    }
}

impl In8<Reg8> for CPU {
    fn read<C: CpuContext>(&mut self, src: Reg8, _: &mut C) -> u8 {
        self.registers.read_8(src)
    }
}

impl In8<Immediate8> for CPU {
    fn read<C: CpuContext>(&mut self, _: Immediate8, ctx: &mut C) -> u8 {
        self.next_byte(ctx)
    }
}

impl In8<Addr> for CPU {
    fn read<C: CpuContext>(&mut self, src: Addr, ctx: &mut C) -> u8 {
        match src {
            Addr::BC => {
                let addr = self.registers.read_16(Reg16::BC);
                ctx.read_cycle(addr)
            }
            Addr::DE => {
                let addr = self.registers.read_16(Reg16::DE);
                ctx.read_cycle(addr)
            }
            Addr::HL => {
                let addr = self.registers.read_16(Reg16::HL);
                ctx.read_cycle(addr)
            }
            Addr::HLI => {
                let addr = self.registers.read_16(Reg16::HL);
                self.registers.write_16(Reg16::HL, addr.wrapping_add(1));
                ctx.read_cycle(addr)
            }
            Addr::HLD => {
                let addr = self.registers.read_16(Reg16::HL);
                self.registers.write_16(Reg16::HL, addr.wrapping_sub(1));
                ctx.read_cycle(addr)
            }
            Addr::Direct => {
                let low = self.next_byte(ctx);
                let high = self.next_byte(ctx);
                let addr = (high as u16) << 8 | low as u16;

                ctx.read_cycle(addr)
            }
            Addr::MaskedMemory => {
                let addr = self.next_byte(ctx);
                ctx.read_cycle_high(addr)
            }
            Addr::MaskedC => {
                let addr = self.registers.read_8(Reg8::C);
                ctx.read_cycle_high(addr)
            }
        }
    }
}

impl Out8<Reg8> for CPU {
    fn write<C: CpuContext>(&mut self, dst: Reg8, value: u8, _: &mut C) {
        self.registers.write_8(dst, value);
    }
}

impl Out8<Addr> for CPU {
    fn write<C: CpuContext>(&mut self, src: Addr, data: u8, ctx: &mut C) {
        match src {
            Addr::BC => {
                let addr = self.registers.read_16(Reg16::BC);
                ctx.write_cycle(addr, data);
            }
            Addr::DE => {
                let addr = self.registers.read_16(Reg16::DE);
                ctx.write_cycle(addr, data);
            }
            Addr::HL => {
                let addr = self.registers.read_16(Reg16::HL);
                ctx.write_cycle(addr, data);
            }
            Addr::HLI => {
                let addr = self.registers.read_16(Reg16::HL);
                self.registers.write_16(Reg16::HL, addr.wrapping_add(1));
                ctx.write_cycle(addr, data);
            }
            Addr::HLD => {
                let addr = self.registers.read_16(Reg16::HL);
                self.registers.write_16(Reg16::HL, addr.wrapping_sub(1));
                ctx.write_cycle(addr, data);
            }
            Addr::Direct => {
                let low = self.next_byte(ctx);
                let high = self.next_byte(ctx);
                let addr = (high as u16) << 8 | low as u16;

                ctx.write_cycle(addr, data);
            }
            Addr::MaskedMemory => {
                let addr = self.next_byte(ctx);
                ctx.write_cycle_high(addr, data);
            }
            Addr::MaskedC => {
                let addr = self.registers.read_8(Reg8::C);
                ctx.write_cycle_high(addr, data);
            }
        }
    }
}
