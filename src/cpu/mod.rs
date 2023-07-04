pub mod alu;
pub mod instruction;
pub mod registers;

use std::num::Wrapping;

use crate::memory::bus::MemoryBus;

use self::alu::ALU;
use self::instruction::{Condition, Instruction};
use self::registers::{Flags, Reg16, Reg8, Registers};

#[derive(Debug)]
pub struct CPU {
    // Program counter
    pc: Wrapping<u16>,

    // Stack pointer
    sp: Wrapping<u16>,

    ime: bool,

    alu: ALU,
    registers: Registers,
    bus: MemoryBus,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: Wrapping(0),
            sp: Wrapping(0xFFFF),
            ime: false,
            alu: ALU {},
            registers: Registers::new(),
            bus: MemoryBus::new(),
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Vec<u8>) {
        cartridge.iter().enumerate().for_each(|(i, byte)| {
            self.bus.write_byte(i as u16, *byte);
        });
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read_byte(self.pc.0);
        if let Some(instruction) = Instruction::from_byte(opcode) {
            println!("{:#X}: {:#X} {:?}", self.pc.0, opcode, instruction);
            self.execute(instruction)
        } else {
            panic!("Invalid opcode: {:#X}", opcode);
        };
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            // Load instructions
            Instruction::LDRR(target, source) => {
                let value = self.registers.read(source);
                self.registers.write(target, value);
                self.pc += 1;
            }
            Instruction::LDRN(target) => {
                self.pc += 1;
                let value = self.bus.read_byte(self.pc.0);
                self.registers.write(target, value);
                self.pc += 1;
            }
            Instruction::LDRHL(target) => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(addr);
                self.registers.write(target, value);
                self.pc += 1;
            }
            Instruction::LDHLR(source) => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.registers.read(source);
                self.bus.write_byte(addr, value);
                self.pc += 1;
            }
            Instruction::LDHLN => {
                self.pc += 1;
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(self.pc.0);
                self.bus.write_byte(addr, value);
                self.pc += 1;
            }
            Instruction::LDA16(source) => {
                let addr = self.registers.read16(source);
                let value = self.bus.read_byte(addr);
                self.registers.write(Reg8::A, value);
                self.pc += 1;
            }
            Instruction::LD16A(target) => {
                let addr = self.registers.read16(target);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.pc += 1;
            }
            Instruction::LDANN => {
                self.pc += 1;
                let value = self.bus.read_byte(self.pc.0);
                self.registers.write(Reg8::A, value);
                self.pc += 1;
            }
            Instruction::LDNNA => {
                self.pc += 1;
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(self.pc.0, value);
                self.pc += 1;
            }
            Instruction::LDHAC => {
                let addr = 0xFF00 | self.registers.read(Reg8::C) as u16;
                let value = self.bus.read_byte(addr);
                self.registers.write(Reg8::A, value);
                self.pc += 1;
            }
            Instruction::LDHCA => {
                let addr = 0xFF00 | self.registers.read(Reg8::C) as u16;
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.pc += 1;
            }
            Instruction::LDHAN => {
                self.pc += 1;
                let addr = 0xFF00 | self.bus.read_byte(self.pc.0) as u16;
                let value = self.bus.read_byte(addr);
                self.registers.write(Reg8::A, value);
                self.pc += 1;
            }
            Instruction::LDHNA => {
                self.pc += 1;
                let addr = 0xFF00 | self.bus.read_byte(self.pc.0) as u16;
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.pc += 1;
            }
            Instruction::LDHLDECA => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.registers.write16(Reg16::HL, addr.wrapping_sub(1));
                self.pc += 1;
            }
            Instruction::LDHLINCA => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.registers.read(Reg8::A);
                self.bus.write_byte(addr, value);
                self.registers.write16(Reg16::HL, addr.wrapping_add(1));
                self.pc += 1;
            }
            Instruction::LDAHLDEC => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(addr);
                self.registers.write16(Reg16::HL, addr.wrapping_sub(1));
                self.registers.write(Reg8::A, value);
                self.pc += 1;
            }
            Instruction::LDAHLINC => {
                let addr = self.registers.read16(Reg16::HL);
                let value = self.bus.read_byte(addr);
                self.registers.write16(Reg16::HL, addr.wrapping_add(1));
                self.registers.write(Reg8::A, value);
                self.pc += 1;
            }
            Instruction::LD16NN(target) => {
                let lower_byte = self.pc.0;
                self.pc += 1;
                let upper_byte = self.pc.0;
                self.pc += 1;

                let word = self.bus.read_word(lower_byte, upper_byte);
                self.registers.write16(target, word);
            }
            Instruction::LDNNSP => {
                let lower_byte = self.pc.0;
                self.pc += 1;
                let upper_byte = self.pc.0;
                self.pc += 1;

                let word = self.bus.read_word(lower_byte, upper_byte);
                self.sp.0 = word;
            }
            Instruction::LDSPHL => {
                let data = self.registers.read16(Reg16::HL);
                self.sp.0 = data;
                self.pc += 1;
            }
            Instruction::PUSH(target) => {
                let data = self.registers.read16(target);
                self.push_16(data);
                self.pc += 1;
            }
            Instruction::POP(target) => {
                let lower_byte = self.bus.read_byte(self.sp.0);
                self.sp += 1;
                let upper_byte = self.bus.read_byte(self.sp.0);
                self.sp += 1;

                let data = ((upper_byte as u16) << 8) | lower_byte as u16;
                self.registers.write16(target, data);

                self.pc += 1;
            }

            // 8-bit Arithmetic and Logical Operations
            Instruction::ADD(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::ADDHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::ADDNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::ADC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::ADCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::ADCNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::SUB(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::SUBHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::SUBNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::SBC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::SBCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::SBCNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
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
                self.pc += 1;
            }
            Instruction::ANDNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let result = a & b;
                let flags = Flags {
                    zero: result == 0,
                    subtract: false,
                    half_carry: true,
                    carry: false,
                };

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
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
                self.pc += 1;
            }
            Instruction::ORNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let result = a | b;
                let flags = Flags {
                    zero: result == 0,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                };

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
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
                self.pc += 1;
            }
            Instruction::XORNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let result = a ^ b;
                let flags = Flags {
                    zero: result == 0,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                };

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
                self.pc += 1;
            }
            Instruction::CP(target) => self.cp(target, Reg8::A),
            Instruction::CPHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(addr);
                let (_, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.pc += 1;
            }
            Instruction::CPNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read_byte(self.pc.0);
                let (_, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.pc += 1;
            }
            Instruction::INC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc += 1;
            }
            Instruction::INCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.bus.read_byte(addr);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.bus.write_byte(addr, result);
                self.pc += 1;
            }
            Instruction::DEC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
                self.pc += 1;
            }
            Instruction::DECHL => {
                let addr = self.registers.read16(Reg16::HL);
                let a = self.bus.read_byte(addr);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.bus.write_byte(addr, result);
                self.pc += 1;
            }

            // 16-bit Arithmetic and Logical Operations
            Instruction::ADDHLR16(target) => {
                let a = self.registers.read16(Reg16::HL);
                let b = self.registers.read16(target);
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write16(Reg16::HL, result);
                self.pc += 1;
            }
            Instruction::ADDSPE => {
                let a = self.sp.0;
                let b: u16 = 0xE8;
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.sp.0 = result;
                self.pc += 1;
            }
            Instruction::INC16(target) => {
                let a = self.registers.read16(target);
                let result = a.wrapping_add(1);

                self.registers.write16(target, result);
                self.pc += 1;
            }
            Instruction::DEC16(target) => {
                let a = self.registers.read16(target);
                let result = a.wrapping_sub(1);

                self.registers.write16(target, result);
                self.pc += 1;
            }

            // Bit Operations
            Instruction::BIT(bit, target) => {
                let data = self.registers.read(target);
                self.bit(bit, data)
            }
            Instruction::BITHL(bit) => {
                let addr = self.registers.read16(Reg16::HL);
                let data = self.bus.read_byte(addr);
                self.bit(bit, data)
            }
            Instruction::SET(bit, target) => {
                let mut data = self.registers.read(target);
                let step = self.set(bit, &mut data);
                self.registers.write(target, data);

                step
            }
            Instruction::SETHL(bit) => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.set(bit, &mut data);
                self.bus.write_byte(addr, data);

                step
            }
            Instruction::RESET(bit, target) => {
                let mut data = self.registers.read(target);
                let step = self.reset(bit, &mut data);
                self.reset(bit, &mut data);

                step
            }
            Instruction::RESETHL(bit) => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.reset(bit, &mut data);
                self.bus.write_byte(addr, data);

                step
            }
            Instruction::SWAP(target) => {
                let mut data = self.registers.read(target);
                let step = self.swap(&mut data);
                self.registers.write(target, data);

                step
            }
            Instruction::SWAPHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.swap(&mut data);
                self.bus.write_byte(addr, data);

                step
            }

            // Bit Shifts
            Instruction::SRL(target) => {
                let mut data = self.registers.read(target);
                let step = self.srl(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::SRLHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.srl(&mut data);

                self.bus.write_byte(addr, data);
                step
            }
            Instruction::SRA(target) => {
                let mut data = self.registers.read(target);
                let step = self.sra(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::SRAHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.sra(&mut data);

                self.bus.write_byte(addr, data);
                step
            }
            Instruction::SLA(target) => {
                let mut data = self.registers.read(target);
                let step = self.sla(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::SLAHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.sla(&mut data);

                self.bus.write_byte(addr, data);
                step
            }
            Instruction::RRA => {
                let mut data = self.registers.read(Reg8::A);
                let step = self.rr(&mut data);

                self.registers.write(Reg8::A, data);
                step
            }
            Instruction::RLA => {
                let mut data = self.registers.read(Reg8::A);
                let step = self.rl(&mut data);

                self.registers.write(Reg8::A, data);
                step
            }
            Instruction::RRCA => {
                let mut data = self.registers.read(Reg8::A);
                let step = self.rrc(&mut data);

                self.registers.write(Reg8::A, data);
                step
            }
            Instruction::RLCA => {
                let mut data = self.registers.read(Reg8::A);
                let step = self.rlc(&mut data);

                self.registers.write(Reg8::A, data);
                step
            }
            Instruction::RR(target) => {
                let mut data = self.registers.read(target);
                let step = self.rr(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::RL(target) => {
                let mut data = self.registers.read(target);
                let step = self.rl(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::RRC(target) => {
                let mut data = self.registers.read(target);
                let step = self.rrc(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::RLC(target) => {
                let mut data = self.registers.read(target);
                let step = self.rlc(&mut data);

                self.registers.write(target, data);
                step
            }
            Instruction::RRHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.rr(&mut data);

                self.bus.write_byte(addr, data);
                step
            }
            Instruction::RLHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.rl(&mut data);

                self.bus.write_byte(addr, data);
                step
            }
            Instruction::RRCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.rrc(&mut data);

                self.bus.write_byte(addr, data);
                step
            }
            Instruction::RLCHL => {
                let addr = self.registers.read16(Reg16::HL);
                let mut data = self.bus.read_byte(addr);
                let step = self.rlc(&mut data);

                self.bus.write_byte(addr, data);
                step
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
            Instruction::JPHL => self.pc.0 = self.registers.read16(Reg16::HL),
            Instruction::JPCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.jp()
                } else {
                    self.pc += 2;
                }
            }
            Instruction::JR => self.jr(),
            Instruction::JRCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.jr()
                } else {
                    self.pc += 2;
                }
            }
            Instruction::CALL => self.call(),
            Instruction::CALLCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.call()
                } else {
                    self.pc += 2;
                }
            }
            Instruction::RET => self.ret(),
            Instruction::RETCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.ret()
                }
            }
            Instruction::RETI => {
                self.ime = true;
                self.ret()
            }
            Instruction::RST(target) => {
                let [upper, lower] = self.pc.0.to_le_bytes();

                self.sp -= 1;
                self.bus.write_byte(self.sp.0, upper);

                self.sp -= 1;
                self.bus.write_byte(self.sp.0, lower);

                self.pc.0 = target as u16;
            }
            Instruction::HALT => {
                unimplemented!()
            }

            // Prefix Operations
            Instruction::PREFIXCB => {
                let opcode = self.bus.read_byte(self.pc.0);

                self.pc += 1;
                if let Some(instruction) = Instruction::from_byte_prefixed(opcode) {
                    self.execute(instruction)
                } else {
                    panic!("Invalid instruction: {:02X}{:02X}", 0xcb, opcode)
                }
            }
        }
    }

    fn cp(&mut self, source: Reg8, target: Reg8) {
        let a = self.registers.read(source);
        let b = self.registers.read(target);
        let (_, flags) = self.alu.sub(a, b);

        self.registers.set_flags(flags);
        self.pc += 1;
    }

    fn and(&mut self, data: Reg8) {
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
        self.pc += 1;
    }

    fn or(&mut self, data: Reg8) {
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
        self.pc += 1;
    }

    fn xor(&mut self, data: Reg8) {
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
        self.pc += 1;
    }

    fn ccf(&mut self) {
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: !carry,
        });
        self.pc += 1;
    }

    fn scf(&mut self) {
        let zero = self.registers.get_flags().zero;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry: true,
        });
        self.pc += 1;
    }

    fn rr(&mut self, data: &mut u8) {
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rr(*data, carry);

        *data = result;

        self.registers.set_flags(flags);
        self.pc += 1;
    }

    fn rl(&mut self, data: &mut u8) {
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rl(*data, carry);

        *data = result;

        self.registers.set_flags(flags);
        self.pc += 1;
    }

    fn rrc(&mut self, data: &mut u8) {
        let (result, flags) = self.alu.rrc(*data);

        *data = result;

        self.registers.set_flags(flags);
        self.pc += 1;
    }

    fn rlc(&mut self, data: &mut u8) {
        let (result, flags) = self.alu.rlc(*data);

        *data = result;

        self.registers.set_flags(flags);
        self.pc += 1;
    }

    fn cpl(&mut self) {
        let a = self.registers.read(Reg8::A);
        let flags = self.registers.get_flags();
        let result = !a;

        self.registers.set_flags(Flags {
            zero: flags.zero,
            subtract: true,
            ..flags
        });

        self.registers.write(Reg8::A, result);
        self.pc += 1;
    }

    fn bit(&mut self, bit: u8, data: u8) {
        let zero = (data & (1 << bit)) == 0;
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: true,
            carry,
        });
        self.pc += 1;
    }

    fn reset(&mut self, bit: u8, data: &mut u8) {
        *data = *data & (0 << bit);

        self.pc += 1;
    }

    fn set(&mut self, bit: u8, data: &mut u8) {
        *data = *data | (1 << bit);

        self.pc += 1;
    }

    fn srl(&mut self, data: &mut u8) {
        let carry = *data & 0x01 != 0;
        *data >>= 1;
        let zero = *data == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry,
        });
        self.pc += 1;
    }

    fn sra(&mut self, data: &mut u8) {
        let sign_bit = *data & 0x80;
        let carry = *data & 0x01 != 0;
        *data = (*data >> 1) | sign_bit;
        let zero = *data == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry,
        });
        self.pc += 1;
    }

    fn sla(&mut self, data: &mut u8) {
        let carry = *data & 0x80 != 0;
        *data <<= 1;
        let zero = *data == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry,
        });
        self.pc += 1;
    }

    fn swap(&mut self, data: &mut u8) {
        *data = (*data << 4) | (*data >> 4);
        let zero = *data == 0;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry: false,
        });
        self.pc += 1;
    }

    fn daa(&mut self) {
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
        self.pc += 1;
    }

    fn di(&mut self) {
        self.ime = false;
        self.pc += 1;
    }

    fn ei(&mut self) {
        self.ime = true;
        self.pc += 1;
    }

    fn nop(&mut self) {
        self.pc += 1;
    }

    fn stop(&mut self) {
        panic!("STOP instruction received");
    }

    fn push_16(&mut self, data: u16) {
        let [high, low] = u16::to_le_bytes(data);

        self.sp -= 1;
        self.bus.write_byte(self.sp.0, high);
        self.sp -= 1;
        self.bus.write_byte(self.sp.0, low);
    }

    fn evaluate_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::NZ => !self.registers.get_flags().zero,
            Condition::Z => self.registers.get_flags().zero,
            Condition::NC => !self.registers.get_flags().carry,
            Condition::C => self.registers.get_flags().carry,
        }
    }

    fn jp(&mut self) {
        let lower = self.bus.read_byte(self.pc.0);
        self.pc += 1;
        let upper = self.bus.read_byte(self.pc.0);

        self.pc.0 = u16::from_le_bytes([lower, upper]);
    }

    fn jr(&mut self) {
        let offset = self.bus.read_byte(self.pc.0) as i8;

        if offset < 0 {
            self.pc -= (-offset) as u16;
        } else {
            self.pc += offset as u16;
        }
    }

    fn call(&mut self) {
        let high = self.bus.read_byte(self.pc.0);
        self.pc += 1;
        let low = self.bus.read_byte(self.pc.0);

        let data = u16::from_le_bytes([high, low]);

        self.push_16(data);
        self.pc += 1;
    }

    fn ret(&mut self) {
        let lower = self.bus.read_byte(self.sp.0);
        self.sp += 1;
        let upper = self.bus.read_byte(self.sp.0);
        self.sp += 1;

        self.pc.0 = u16::from_le_bytes([lower, upper])
    }
}
