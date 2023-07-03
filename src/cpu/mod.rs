pub mod alu;
pub mod instruction;
pub mod registers;

use crate::memory::bus::MemoryBus;

use self::alu::ALU;
use self::instruction::Instruction;
use self::registers::{Flags, Reg16, Reg8, Registers};

struct CPU {
    // Program counter
    pc: u16,

    // Stack pointer
    sp: u16,

    alu: ALU,
    registers: Registers,
    bus: MemoryBus,
}

impl CPU {
    pub fn new(&self) -> CPU {
        CPU {
            pc: 0,
            sp: 0,
            alu: ALU {},
            registers: Registers::new(),
            bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let opcode = self.bus.read_byte(self.pc);
        let next_pc = if let Some(instruction) = Instruction::from_byte(opcode) {
            self.execute(instruction)
        } else {
            panic!("Invalid opcode: {:#X}", opcode);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDHL(target) => {
                let a = self.registers.read16(Reg16::HL);
                let b = self.registers.read16(target);
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write16(Reg16::HL, result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::SBC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
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
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc.wrapping_add(1)
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
                self.pc.wrapping_add(1)
            }
            Instruction::RL(target) => {
                let carry = self.registers.get_flags().carry as u8;
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rl(a, carry);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc.wrapping_add(1)
            }
            Instruction::RRC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rrc(a);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc.wrapping_add(1)
            }
            Instruction::RLC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.rlc(a);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc.wrapping_add(1)
            }
            Instruction::SRA(target) => self.sra(target),
            Instruction::SLA(target) => self.sla(target),
            Instruction::SWAP(target) => self.swap(target),
        }
    }

    pub fn cp(&mut self, source: Reg8, target: Reg8) -> u16 {
        let a = self.registers.read(source);
        let b = self.registers.read(target);
        let (_, flags) = self.alu.sub(a, b);

        self.registers.set_flags(flags);
        self.pc.wrapping_add(1)
    }

    pub fn and(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }

    pub fn or(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }

    pub fn xor(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }

    pub fn ccf(&mut self) -> u16 {
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: !carry,
        });
        self.pc.wrapping_add(1)
    }

    pub fn scf(&mut self) -> u16 {
        let zero = self.registers.get_flags().zero;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry: true,
        });
        self.pc.wrapping_add(1)
    }

    pub fn rra(&mut self) -> u16 {
        let a = self.registers.read(Reg8::A);
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rr(a, carry);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
        self.pc.wrapping_add(1)
    }

    pub fn rla(&mut self) -> u16 {
        let a = self.registers.read(Reg8::A);
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rl(a, carry);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
        self.pc.wrapping_add(1)
    }

    pub fn rrca(&mut self) -> u16 {
        let a = self.registers.read(Reg8::A);
        let (result, flags) = self.alu.rrc(a);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
        self.pc.wrapping_add(1)
    }

    pub fn rlca(&mut self) -> u16 {
        let a = self.registers.read(Reg8::A);
        let (result, flags) = self.alu.rlc(a);

        self.registers.set_flags(flags);
        self.registers.write(Reg8::A, result);
        self.pc.wrapping_add(1)
    }

    pub fn cpl(&mut self) -> u16 {
        let a = self.registers.read(Reg8::A);
        let flags = self.registers.get_flags();
        let result = !a;

        self.registers.set_flags(Flags {
            zero: flags.zero,
            subtract: true,
            ..flags
        });

        self.registers.write(Reg8::A, result);
        self.pc.wrapping_add(1)
    }

    pub fn bit(&mut self, bit: u8, data: Reg8) -> u16 {
        let a = self.registers.read(data);
        let zero = (a & (1 << bit)) == 0;
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: true,
            carry,
        });
        self.pc.wrapping_add(1)
    }

    pub fn reset(&mut self, bit: u8, data: Reg8) -> u16 {
        let a = self.registers.read(data);
        let result = a & (0 << bit);

        self.registers.write(data, result);
        self.pc.wrapping_add(1)
    }

    pub fn set(&mut self, bit: u8, data: Reg8) -> u16 {
        let a = self.registers.read(data);
        let result = a | (1 << bit);

        self.registers.write(data, result);
        self.pc.wrapping_add(1)
    }

    pub fn srl(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }

    pub fn sra(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }

    pub fn sla(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }

    pub fn swap(&mut self, data: Reg8) -> u16 {
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
        self.pc.wrapping_add(1)
    }
}
