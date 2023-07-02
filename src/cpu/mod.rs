pub mod alu;
pub mod registers;

use self::alu::ALU;
use self::registers::{Reg16, Reg8, Registers};

pub enum Instruction {
    ADD(Reg8),
    SUB(Reg8),
    AND(Reg8),
    OR(Reg8),
    XOR(Reg8),
    CP(Reg8),
}

struct CPU {
    opcode: u8,
    ime: bool,
    registers: Registers,
}

impl CPU {
    pub fn new(&self) -> CPU {
        CPU {
            opcode: 0x00,
            ime: false,
            registers: Registers::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let alu = ALU {};
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = alu.add(a, b);

                self.registers
                    .set_flags(flags.zero, false, flags.half_carry, flags.carry);
                self.registers.write(Reg8::A, result);
            }
            Instruction::SUB(target) => {
                let alu = ALU {};
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = alu.sub(a, b);

                self.registers
                    .set_flags(flags.zero, true, flags.half_carry, flags.carry);
                self.registers.write(Reg8::A, result);
            }
            Instruction::AND(target) => self.and(target),
            Instruction::OR(target) => self.or(target),
            Instruction::XOR(target) => self.xor(target),
            Instruction::CP(target) => {
                let alu = ALU {};
                self.cp(target, Reg8::A, &alu);
            }
        }
    }

    pub fn cp(&mut self, source: Reg8, target: Reg8, alu: &ALU) {
        let a = self.registers.read(source);
        let b = self.registers.read(target);
        let (_, flags) = alu.sub(a, b);

        self.registers
            .set_flags(flags.zero, true, flags.half_carry, flags.carry);
    }

    pub fn and(&mut self, data: Reg8) {
        let a = self.registers.read(Reg8::A);
        let b = self.registers.read(data);
        let result = a & b;

        let zero = result == 0;

        self.registers.set_flags(zero, true, true, false);
        self.registers.write(Reg8::A, result);
    }

    pub fn or(&mut self, data: Reg8) {
        let a = self.registers.read(Reg8::A);
        let b = self.registers.read(data);

        let result = a | b;

        let zero = result == 0;

        self.registers.set_flags(zero, false, false, false);
        self.registers.write(Reg8::A, result);
    }

    pub fn xor(&mut self, data: Reg8) {
        let a = self.registers.read(Reg8::A);
        let b = self.registers.read(data);

        let result = a ^ b;

        let zero = result == 0;

        self.registers.set_flags(zero, false, false, false);
        self.registers.write(Reg8::A, result);
    }

    pub fn add_hl(&mut self, data: Reg16, alu: &ALU) {
        let hl = self.registers.read16(Reg16::HL);
        let data = self.registers.read16(data);
        let (result, flags) = alu.add16(hl, data);

        self.registers
            .set_flags(false, false, flags.half_carry, flags.carry);

        self.registers.write16(Reg16::HL, result);
    }

    pub fn ccf(&mut self) {
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(false, false, false, !carry);
    }

    pub fn scf(&mut self) {
        let zero = self.registers.get_flags().zero;

        self.registers.set_flags(zero, false, false, true);
    }

    pub fn rra(&mut self, alu: &ALU) {
        let a = self.registers.read(Reg8::A);
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = alu.rr(a, carry);

        self.registers
            .set_flags(flags.zero, false, false, flags.carry);
        self.registers.write(Reg8::A, result);
    }

    pub fn rla(&mut self, alu: &ALU) {
        let a = self.registers.read(Reg8::A);
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = alu.rl(a, carry);

        self.registers
            .set_flags(flags.zero, false, false, flags.carry);
        self.registers.write(Reg8::A, result);
    }

    pub fn rrca(&mut self, alu: &ALU) {
        let a = self.registers.read(Reg8::A);
        let (result, flags) = alu.rrc(a);

        self.registers
            .set_flags(flags.zero, false, false, flags.carry);
        self.registers.write(Reg8::A, result);
    }

    pub fn rlca(&mut self, alu: &ALU) {
        let a = self.registers.read(Reg8::A);
        let (result, flags) = alu.rlc(a);

        self.registers
            .set_flags(flags.zero, false, false, flags.carry);
        self.registers.write(Reg8::A, result);
    }

    pub fn cpl(&mut self) {
        let a = self.registers.read(Reg8::A);
        let flags = self.registers.get_flags();
        let result = !a;

        self.registers
            .set_flags(flags.zero, true, true, flags.carry);

        self.registers.write(Reg8::A, result);
    }

    pub fn bit(&mut self, bit: u8, data: Reg8) {
        let a = self.registers.read(data);
        let zero = (a & (1 << bit)) == 0;
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(zero, false, true, carry)
    }

    pub fn reset(&mut self, bit: u8, data: Reg8) {
        let a = self.registers.read(data);
        let result = a & (0 << bit);

        self.registers.write(data, result);
    }

    pub fn set(&mut self, bit: u8, data: Reg8) {
        let a = self.registers.read(data);
        let result = a | (1 << bit);

        self.registers.write(data, result);
    }

    pub fn srl(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let carry = a & 0x01 != 0;
        let result = a >> 1;
        let zero = result == 0;

        self.registers.set_flags(zero, false, false, carry);
        self.registers.write(data, result);
    }

    pub fn sra(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let sign_bit = a & 0x80;
        let carry = a & 0x01 != 0;
        let result = (a >> 1) | sign_bit;
        let zero = result == 0;

        self.registers.set_flags(zero, false, false, carry);
        self.registers.write(data, result);
    }

    pub fn sla(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let carry = a & 0x80 != 0;
        let result = a << 1;
        let zero = result == 0;

        self.registers.set_flags(zero, false, false, carry);
        self.registers.write(data, result);
    }

    pub fn swap(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let result = (a << 4) | (a >> 4);
        let zero = result == 0;

        self.registers.set_flags(zero, false, false, false);
        self.registers.write(data, result);
    }
}
