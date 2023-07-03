pub mod alu;
pub mod registers;

use self::alu::ALU;
use self::registers::{Flags, Reg16, Reg8, Registers};

pub enum Instruction {
    ADD(Reg8),
    ADDHL(Reg16),
    ADC(Reg8),
    SUB(Reg8),
    SBC(Reg8),
    AND(Reg8),
    OR(Reg8),
    XOR(Reg8),
    CP(Reg8),
    INC(Reg8),
    DEC(Reg8),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    BIT(u8, Reg8),
    SET(u8, Reg8),
    SRL(Reg8),
    RR(Reg8),
    RL(Reg8),
    RLC(Reg8),
    RRC(Reg8),
    SRA(Reg8),
    SLA(Reg8),
    SWAP(Reg8),
}

struct CPU {
    alu: ALU,
    opcode: u8,
    ime: bool,
    registers: Registers,
}

impl CPU {
    pub fn new(&self) -> CPU {
        CPU {
            alu: ALU {},
            opcode: 0x00,
            ime: false,
            registers: Registers::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::ADDHL(target) => {
                let a = self.registers.read16(Reg16::HL);
                let b = self.registers.read16(target);
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write16(Reg16::HL, result);
            }
            Instruction::ADC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::SUB(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::SBC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::AND(target) => self.and(target),
            Instruction::OR(target) => self.or(target),
            Instruction::XOR(target) => self.xor(target),
            Instruction::CP(target) => self.cp(target, Reg8::A),
            Instruction::INC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::DEC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::RRA => self.rra(),
            Instruction::RLA => self.rla(),
            Instruction::RRCA => self.rrca(),
            Instruction::RLCA => self.rlca(),
            Instruction::CPL => self.cpl(),
            Instruction::BIT(bit, target) => self.bit(bit, target),
            Instruction::SET(bit, target) => self.set(bit, target),
            Instruction::SRL(target) => self.srl(target),
            Instruction::RR(target) => {
                let carry = self.registers.get_flags().carry as u8;
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rr(a, carry);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::RL(target) => {
                let carry = self.registers.get_flags().carry as u8;
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rl(a, carry);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::RRC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rrc(a);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::RLC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rlc(a);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::SRA(target) => self.sra(target),
            Instruction::SLA(target) => self.sla(target),
            Instruction::SWAP(target) => self.swap(target),
        }
    }

    pub fn cp(&mut self, source: Reg8, target: Reg8) {
        let a = self.registers.read(source);
        let b = self.registers.read(target);
        let (_, flags) = self.alu.sub(a, b);

        self.registers.set_flags(flags);
    }

    pub fn and(&mut self, data: Reg8) {
        let a = self.registers.read(Reg8::A);
        let b = self.registers.read(data);
        let result = a & b;
        let flags = Flags {
            zero: result == 0,
            subtract: false,
            half_carry: true,
            carry: false,
        };

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn or(&mut self, data: Reg8) {
        let a = self.registers.read(Reg8::A);
        let b = self.registers.read(data);
        let result = a | b;
        let flags = Flags {
            zero: result == 0,
            subtract: false,
            half_carry: false,
            carry: false,
        };

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn xor(&mut self, data: Reg8) {
        let a = self.registers.read(Reg8::A);
        let b = self.registers.read(data);
        let result = a ^ b;
        let flags = Flags {
            zero: result == 0,
            subtract: false,
            half_carry: false,
            carry: false,
        };

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn ccf(&mut self) {
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: !carry,
        });
    }

    pub fn scf(&mut self) {
        let zero = self.registers.get_flags().zero;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry: true,
        });
    }

    pub fn rra(&mut self) {
        let a = self.registers.read(Reg8::A);
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rr(a, carry);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn rla(&mut self) {
        let a = self.registers.read(Reg8::A);
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rl(a, carry);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn rrca(&mut self) {
        let a = self.registers.read(Reg8::A);
        let (result, flags) = self.alu.rrc(a);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn rlca(&mut self) {
        let a = self.registers.read(Reg8::A);
        let (result, flags) = self.alu.rlc(a);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
    }

    pub fn cpl(&mut self) {
        let a = self.registers.read(Reg8::A);
        let flags = self.registers.get_flags();
        let result = !a;

        self.registers.set_flags(Flags {
            zero: flags.zero,
            subtract: true,
            ..flags
        });

        self.registers.write(Reg8::A, result);
    }

    pub fn bit(&mut self, bit: u8, data: Reg8) {
        let a = self.registers.read(data);
        let zero = (a & (1 << bit)) == 0;
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: true,
            carry,
        })
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

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry,
        });
        self.registers.write(data, result);
    }

    pub fn sra(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let sign_bit = a & 0x80;
        let carry = a & 0x01 != 0;
        let result = (a >> 1) | sign_bit;
        let zero = result == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry,
        });
        self.registers.write(data, result);
    }

    pub fn sla(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let carry = a & 0x80 != 0;
        let result = a << 1;
        let zero = result == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry,
        });
        self.registers.write(data, result);
    }

    pub fn swap(&mut self, data: Reg8) {
        let a = self.registers.read(data);
        let result = (a << 4) | (a >> 4);
        let zero = result == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry: false,
        });
        self.registers.write(data, result);
    }
}
