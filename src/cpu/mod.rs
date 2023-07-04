pub mod alu;
pub mod instruction;
pub mod registers;

use crate::memory::bus::MemoryBus;

use self::alu::ALU;
use self::instruction::{Condition, Instruction};
use self::registers::{Flags, Reg16, Reg8, Registers};

struct CPU {
    // Program counter
    pc: u16,

    ime: bool,

    alu: ALU,
    registers: Registers,
    bus: MemoryBus,
}

impl CPU {
    pub fn new(&self) -> CPU {
        CPU {
            pc: 0,
            ime: false,
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
            // Load instructions
            Instruction::LDRR(target, source) => {
                let value = self.registers.read(source);
                self.registers.write(target, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDRN(target) => {
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read_byte(self.pc);
                self.registers.write(target, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDRHL(target) => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(addr);
                self.registers.write(target, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHLR(source) => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.registers.read(source);
                self.bus.write_byte(addr, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHLN => {
                self.pc = self.pc.wrapping_add(1);
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(self.pc);
                self.bus.write_byte(addr, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDA16(source) => {
                let addr = self.registers.read16(source);
                let value = self.bus.read_byte(addr);
                self.registers.write(Reg8::A, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LD16A(target) => {
                let addr = self.registers.read16(target);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDANN => {
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read_byte(self.pc);
                self.registers.write(Reg8::A, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDNNA => {
                self.pc = self.pc.wrapping_add(1);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(self.pc, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHAC => {
                let addr = 0xFF00 | self.registers.read(Reg8::C) as u16;
                let value = self.bus.read_byte(addr);
                self.registers.write(Reg8::A, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHCA => {
                let addr = 0xFF00 | self.registers.read(Reg8::C) as u16;
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHAN => {
                self.pc = self.pc.wrapping_add(1);
                let addr = 0xFF00 | self.bus.read_byte(self.pc) as u16;
                let value = self.bus.read_byte(addr);
                self.registers.write(Reg8::A, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHNA => {
                self.pc = self.pc.wrapping_add(1);
                let addr = 0xFF00 | self.bus.read_byte(self.pc) as u16;
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDHLDECA => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.registers.write16(Reg16::HL, addr.wrapping_sub(1));
                self.pc.wrapping_add(1)
            }
            Instruction::LDHLINCA => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.registers.write16(Reg16::HL, addr.wrapping_add(1));
                self.pc.wrapping_add(1)
            }
            Instruction::LDAHLDEC => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(addr);
                self.registers.write16(Reg16::HL, addr.wrapping_sub(1));
                self.registers.write(Reg8::A, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LDAHLINC => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(addr);
                self.registers.write16(Reg16::HL, addr.wrapping_add(1));
                self.registers.write(Reg8::A, value);
                self.pc.wrapping_add(1)
            }
            Instruction::LD16NN(target) => {
                let lower_byte = self.pc.wrapping_add(1);
                self.pc = lower_byte;
                let upper_byte = self.pc.wrapping_add(1);
                self.pc = upper_byte;

                let word = self.bus.read_word(lower_byte, upper_byte);
                self.registers.write16(target, word);
                self.pc.wrapping_add(1)
            }
            Instruction::LDNNSP => {
                let lower_byte = self.pc.wrapping_add(1);
                self.pc = lower_byte;
                let upper_byte = self.pc.wrapping_add(1);
                self.pc = upper_byte;

                let word = self.bus.read_word(lower_byte, upper_byte);
                self.registers.write16(Reg16::SP, word);
                self.pc.wrapping_add(1)
            }
            Instruction::LDSPHL => {
                let data = self.registers.read16(Reg16::HL);
                self.registers.write16(Reg16::SP, data);
                self.pc.wrapping_add(1)
            }
            Instruction::PUSH(target) => {
                let data = self.registers.read16(target);
                let addr = self.registers.read16(Reg16::SP);

                self.push_16(data, addr);

                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let mut addr = self.registers.read16(Reg16::SP);
                let lower_byte = self.bus.read_byte(addr);

                addr = addr.wrapping_add(1);
                self.registers.write16(Reg16::SP, addr);
                let upper_byte = self.bus.read_byte(addr);
                addr = addr.wrapping_add(1);
                self.registers.write16(Reg16::SP, addr);

                let data = ((upper_byte as u16) << 8) | lower_byte as u16;
                self.registers.write16(target, data);

                self.pc.wrapping_add(1)
            }

            // 8-bit Arithmetic and Logical Operations
            Instruction::ADD(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
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
            Instruction::ADCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADCNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
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
            Instruction::SUBHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::SUBNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
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
            Instruction::SBCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::SBCNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc.wrapping_add(1)
            }
            Instruction::AND(target) => self.and(target),
            Instruction::ANDHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
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
            Instruction::ANDNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
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
            Instruction::OR(target) => self.or(target),
            Instruction::ORHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
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
            Instruction::ORNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
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
            Instruction::XOR(target) => self.xor(target),
            Instruction::XORHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
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
            Instruction::XORNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
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
            Instruction::CP(target) => self.cp(target, Reg8::A),
            Instruction::CPHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let (_, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.pc.wrapping_add(1)
            }
            Instruction::CPNN => {
                self.pc = self.pc.wrapping_add(1);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc);
                let (_, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.pc.wrapping_add(1)
            }
            Instruction::INC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc.wrapping_add(1)
            }
            Instruction::INCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.bus.read_byte(addr);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.bus.write_byte(addr, result);
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc.wrapping_add(1)
            }
            Instruction::DECHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.bus.read_byte(addr);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.bus.write_byte(addr, result);
                self.pc.wrapping_add(1)
            }

            // 16-bit Arithmetic and Logical Operations
            Instruction::ADDHLR16(target) => {
                let a = self.registers.read16(Reg16::HL);
                let b = self.registers.read16(target);
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write16(Reg16::HL, result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDSPE => {
                let a = self.registers.read16(Reg16::SP);
                let b: u16 = 0xE8;
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write16(Reg16::SP, result);
                self.pc.wrapping_add(1)
            }
            Instruction::INC16(target) => {
                let a = self.registers.read16(target);
                let result = a.wrapping_add(1);

                self.registers.write16(target, result);
                self.pc.wrapping_add(1)
            }
            Instruction::DEC16(target) => {
                let a = self.registers.read16(target);
                let result = a.wrapping_sub(1);

                self.registers.write16(target, result);
                self.pc.wrapping_add(1)
            }

            // Bit Operations
            Instruction::BIT(bit, target) => self.bit(bit, target),
            Instruction::SET(bit, target) => self.set(bit, target),
            Instruction::RESET(bit, target) => self.reset(bit, target),
            Instruction::SWAP(target) => self.swap(target),

            // Bit Shifts
            Instruction::SRL(target) => self.srl(target),
            Instruction::SRA(target) => self.sra(target),
            Instruction::SLA(target) => self.sla(target),
            Instruction::RRA => self.rra(),
            Instruction::RLA => self.rla(),
            Instruction::RRCA => self.rrca(),
            Instruction::RLCA => self.rlca(),
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

            // Misc Operations
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::CPL => self.cpl(),
            Instruction::DAA => self.daa(),
            Instruction::NOP => self.nop(),
            // Instruction::HALT => self.halt(),
            Instruction::STOP => self.stop(),
            Instruction::DI => self.di(),
            Instruction::EI => self.ei(),

            // Control Operations
            Instruction::JP => self.jp(),
            Instruction::JPHL => self.registers.read16(Reg16::HL),
            Instruction::JPCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.jp()
                } else {
                    self.pc.wrapping_add(2)
                }
            }
            Instruction::JR => self.jr(),
            Instruction::JRCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.jr()
                } else {
                    self.pc.wrapping_add(2)
                }
            }
            Instruction::CALL => self.call(),
            Instruction::CALLCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.call()
                } else {
                    self.pc.wrapping_add(2)
                }
            }
            Instruction::RET => self.ret(),
            Instruction::RETCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.ret()
                } else {
                    self.pc
                }
            }
            Instruction::RETI => {
                self.ime = true;
                self.ret()
            }
            Instruction::RST(target) => {
                let [upper, lower] = self.pc.to_le_bytes();
                let mut sp = self.registers.read16(Reg16::SP);

                sp = sp.wrapping_sub(1);
                self.registers.write16(Reg16::SP, sp);
                self.bus.write_byte(sp, upper);

                sp = sp.wrapping_sub(1);
                self.registers.write16(Reg16::SP, sp);
                self.bus.write_byte(sp, lower);

                target as u16
            }
            Instruction::HALT => {
                unimplemented!()
            }
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

    pub fn daa(&mut self) -> u16 {
        // DAA table in page 110 of the official "Game Boy Programming Manual"
        let mut flags = self.registers.get_flags();
        let mut carry = false;
        let mut a = self.registers.read(Reg8::A);

        if !flags.subtract {
            if flags.carry || a > 0x99 {
                a = a.wrapping_add(0x60);
                carry = true;
            }
            if flags.half_carry || a & 0x0f > 0x09 {
                a = a.wrapping_add(0x06);
            }
        } else if flags.carry {
            carry = true;
            a = a.wrapping_add(if flags.half_carry { 0x9a } else { 0xa0 });
        } else if flags.half_carry {
            a = a.wrapping_add(0xfa);
        }

        flags.zero = a == 0;
        flags.half_carry = false;
        flags.carry = carry;

        self.registers.set_flags(flags);
        self.pc.wrapping_add(1)
    }

    pub fn di(&mut self) -> u16 {
        self.ime = false;
        self.pc.wrapping_add(1)
    }

    pub fn ei(&mut self) -> u16 {
        self.ime = true;
        self.pc.wrapping_add(1)
    }

    pub fn nop(&mut self) -> u16 {
        self.pc.wrapping_add(1)
    }

    pub fn stop(&mut self) -> u16 {
        panic!("STOP instruction received");
    }

    pub fn push_16(&mut self, data: u16, addr: u16) {
        let [upper_byte, lower_byte] = u16::to_le_bytes(data);

        self.registers.write16(Reg16::SP, addr.wrapping_sub(1));
        self.bus.write_byte(addr, upper_byte);
        self.registers.write16(Reg16::SP, addr.wrapping_sub(1));
        self.bus.write_byte(addr, lower_byte);
    }

    fn evaluate_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::NZ => !self.registers.get_flags().zero,
            Condition::Z => self.registers.get_flags().zero,
            Condition::NC => !self.registers.get_flags().carry,
            Condition::C => self.registers.get_flags().carry,
        }
    }

    fn jp(&self) -> u16 {
        let lower = self.bus.read_byte(self.pc.wrapping_add(1));
        let upper = self.bus.read_byte(self.pc.wrapping_add(2));

        u16::from_le_bytes([lower, upper])
    }

    fn jr(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(1);
        let offset = self.bus.read_byte(self.pc) as i8;

        self.pc.wrapping_add(offset as u16)
    }

    fn call(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(1);
        let lower = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let upper = self.bus.read_byte(self.pc);

        let mut sp = self.registers.read16(Reg16::SP);
        sp = sp.wrapping_sub(1);
        self.registers.write16(Reg16::SP, sp);

        let [upper_byte, lower_byte] = u16::to_le_bytes(self.pc);
        self.bus.write_byte(sp, upper_byte);
        sp = sp.wrapping_sub(1);
        self.registers.write16(Reg16::SP, sp);

        self.bus.write_byte(sp, lower_byte);
        sp = sp.wrapping_sub(1);
        self.registers.write16(Reg16::SP, sp);

        u16::from_le_bytes([lower, upper])
    }

    fn ret(&mut self) -> u16 {
        let mut sp = self.registers.read16(Reg16::SP);
        let lower = self.bus.read_byte(sp);

        sp = sp.wrapping_add(1);
        self.registers.write16(Reg16::SP, sp);
        let upper = self.bus.read_byte(sp);

        sp = sp.wrapping_add(1);
        self.registers.write16(Reg16::SP, sp);

        u16::from_le_bytes([lower, upper])
    }
}
