pub mod alu;
pub mod instruction;
pub mod registers;

use self::alu::ALU;
use self::instruction::{Condition, Instruction};
use self::registers::{Flags, Reg16, Reg8, Registers};
use crate::memory::bus::MemoryBus;
use crate::utils::traits::Storage;

#[derive(Debug)]
pub enum Mode {
    Halted,
    Running,
    InterruptDispatch,
}

#[derive(Debug)]
pub struct CPU<'a> {
    ime: bool,
    alu: ALU,
    registers: Registers,
    bus: &'a mut MemoryBus,
    mode: Mode,
}

impl CPU<'_> {
    pub fn new<'a>(bus: &'a mut MemoryBus) -> CPU<'a> {
        CPU {
            bus,
            ime: false,
            alu: ALU {},
            registers: Registers::new(),
            mode: Mode::Running,
        }
    }

    fn check_interrupt_requests(&mut self) -> u8 {
        let interrupt_requests: u8 = self.bus.read(0xFF0F as usize);
        let interrupt_enable: u8 = self.bus.read(0xFFFF as usize);

        interrupt_requests & interrupt_enable & 0x1F
    }

    pub fn step(&mut self) {
        let opcode = self.registers.pc.read(&mut self.bus) as u8;

        match self.mode {
            Mode::Halted => {
                let interrupt_requests = self.check_interrupt_requests();
                if interrupt_requests != 0 {
                    self.mode = Mode::InterruptDispatch;
                }
                return;
            }
            Mode::Running => (),
            Mode::InterruptDispatch => unimplemented!(),
        }

        if let Some(instruction) = Instruction::from_byte(opcode) {
            dbg!(
                "{:#X}: {:#X} {:?}",
                self.registers.pc.pointer.0,
                opcode,
                &instruction
            );
            self.execute(instruction);
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
            }
            Instruction::LDRN(target) => {
                let value = self.registers.pc.read(&mut self.bus) as u8;
                self.registers.write(target, value);
            }
            Instruction::LDRHL(target) => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.bus.read(addr as usize);
                self.registers.write(target, value);
            }
            Instruction::LDHLR(source) => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.registers.read(source);
                self.bus.write(addr as usize, value);
            }
            Instruction::LDHLN => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.registers.pc.read(&mut self.bus) as u8;
                self.bus.write(addr as usize, value);
            }
            Instruction::LDA16(source) => {
                let addr = self.registers.read(source) as usize;
                let value = self.bus.read(addr);
                self.registers.write(Reg8::A, value);
            }
            Instruction::LD16A(target) => {
                let addr = self.registers.read(target) as usize;
                let value = self.registers.read(Reg8::A);
                self.bus.write(addr, value);
            }
            Instruction::LDANN => {
                let value = self.registers.pc.read(&mut self.bus) as u8;
                self.registers.write(Reg8::A, value);
            }
            Instruction::LDNNA => {
                let lower_byte = self.registers.pc.read(&mut self.bus) as u8;
                let upper_byte = self.registers.pc.read(&mut self.bus) as u8;

                let addr = u16::from_le_bytes([lower_byte, upper_byte]) as usize;

                let value = self.registers.read(Reg8::A);
                self.bus.write(addr, value);
            }
            Instruction::LDHAC => {
                let addr = 0xFF00 | self.registers.read(Reg8::C) as u16;
                let value = self.bus.read(addr as usize);
                self.registers.write(Reg8::A, value);
            }
            Instruction::LDHCA => {
                let addr = 0xFF00 | self.registers.read(Reg8::C) as u16;
                let value = self.registers.read(Reg8::A);
                self.bus.write(addr as usize, value);
            }
            Instruction::LDHAN => {
                let addr = 0xFF00 | self.registers.pc.read(&mut self.bus);
                let value = self.bus.read(addr as usize);
                self.registers.write(Reg8::A, value);
            }
            Instruction::LDHNA => {
                let addr = 0xFF00 | self.registers.pc.read(&mut self.bus);
                let value = self.registers.read(Reg8::A);
                self.bus.write(addr as usize, value);
            }
            Instruction::LDHLDECA => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.registers.read(Reg8::A);
                self.bus.write(addr as usize, value);
                self.registers.write(Reg16::HL, addr.wrapping_sub(1));
            }
            Instruction::LDHLINCA => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.registers.read(Reg8::A);
                self.bus.write(addr as usize, value);
                self.registers.write(Reg16::HL, addr.wrapping_add(1));
            }
            Instruction::LDAHLDEC => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.bus.read(addr as usize);
                self.registers.write(Reg16::HL, addr.wrapping_sub(1));
                self.registers.write(Reg8::A, value);
            }
            Instruction::LDAHLINC => {
                let addr = self.registers.read(Reg16::HL);
                let value = self.bus.read(addr as usize);
                self.registers.write(Reg16::HL, addr.wrapping_add(1));
                self.registers.write(Reg8::A, value);
            }
            Instruction::LD16NN(target) => {
                let lower_byte = self.registers.pc.read(&mut self.bus) as u8;
                let upper_byte = self.registers.pc.read(&mut self.bus) as u8;

                let data = u16::from_le_bytes([lower_byte, upper_byte]);

                self.registers.write(target, data);
            }
            Instruction::LD16SP => {
                let lower_byte = self.registers.pc.read(&mut self.bus) as u8;
                let upper_byte = self.registers.pc.read(&mut self.bus) as u8;

                let data = u16::from_le_bytes([lower_byte, upper_byte]);

                self.registers.sp.write(&mut self.bus, data);
            }
            Instruction::LDNNSP => {
                let lower_byte = self.registers.pc.read(&mut self.bus) as u8;
                let upper_byte = self.registers.pc.read(&mut self.bus) as u8;

                let data = u16::from_le_bytes([lower_byte, upper_byte]);
                self.registers.sp.write(&mut self.bus, data);
            }
            Instruction::LDSPHL => {
                let data = self.registers.read(Reg16::HL);
                self.registers.sp.write(&mut self.bus, data);
            }
            Instruction::PUSH(target) => {
                let data = self.registers.read(target);
                self.registers.sp.write(&mut self.bus, data);
            }
            Instruction::POP(target) => {
                let data = self.registers.sp.read(&mut self.bus);
                self.registers.write(target, data);
            }

            // 8-bit Arithmetic and Logical Operations
            Instruction::ADD(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::ADDHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read(addr);
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::ADDNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
                let (result, flags) = self.alu.add(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::ADC(target) => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.read(target);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::ADCHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read(addr);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.adc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::ADCNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
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
            Instruction::SUBHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read(addr);
                let (result, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::SUBNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
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
            Instruction::SBCHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read(addr);
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::SBCNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
                let carry = self.registers.get_flags().carry as u8;
                let (result, flags) = self.alu.sbc(a, b, carry);

                self.registers.set_flags(flags);
                self.registers.write(Reg8::A, result);
            }
            Instruction::AND(target) => self.and(target),
            Instruction::ANDHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b: u8 = self.bus.read(addr);
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
            Instruction::ANDNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
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
            Instruction::OR(target) => self.or(target),
            Instruction::ORHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b: u8 = self.bus.read(addr);
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
            Instruction::ORNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
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
            Instruction::XOR(target) => self.xor(target),
            Instruction::XORHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b: u8 = self.bus.read(addr);
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
            Instruction::XORNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
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
            Instruction::CP(target) => self.cp(target, Reg8::A),
            Instruction::CPHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.registers.read(Reg8::A);
                let b = self.bus.read(addr);
                let (_, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
            }
            Instruction::CPNN => {
                let a = self.registers.read(Reg8::A);
                let b = self.registers.pc.read(&mut self.bus) as u8;
                let (_, flags) = self.alu.sub(a, b);

                self.registers.set_flags(flags);
            }
            Instruction::INC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::INCHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.bus.read(addr);
                let (result, flags) = self.alu.add(a, 1);

                self.registers.set_flags(flags);
                self.bus.write(addr, result);
            }
            Instruction::DEC(target) => {
                let a = self.registers.read(target);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.registers.write(target, result);
            }
            Instruction::DECHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let a = self.bus.read(addr);
                let (result, flags) = self.alu.sub(a, 1);

                self.registers.set_flags(flags);
                self.bus.write(addr, result);
            }

            // 16-bit Arithmetic and Logical Operations
            Instruction::ADDHLR16(target) => {
                let a = self.registers.read(Reg16::HL);
                let b = self.registers.read(target);
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg16::HL, result);
            }
            Instruction::ADDHLRSP => {
                let a = self.registers.read(Reg16::HL);
                let b = self.registers.sp.pointer.0;
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.write(Reg16::HL, result);
            }
            Instruction::ADDSPE => {
                let a = self.registers.sp.pointer.0;
                let b: u16 = 0xE8;
                let (result, flags) = self.alu.add16(a, b);

                self.registers.set_flags(flags);
                self.registers.sp.pointer.0 = result;
            }
            Instruction::INC16(target) => {
                let a = self.registers.read(target);
                let result = a.wrapping_add(1);

                self.registers.write(target, result);
            }
            Instruction::INC16SP => {
                let a = self.registers.sp.pointer.0;
                let result = a.wrapping_add(1);

                self.registers.sp.pointer.0 = result;
            }
            Instruction::DEC16(target) => {
                let a = self.registers.read(target);
                let result = a.wrapping_sub(1);

                self.registers.write(target, result);
            }
            Instruction::DEC16SP => {
                let a = self.registers.sp.pointer.0;
                let result = a.wrapping_sub(1);

                self.registers.sp.pointer.0 = result;
            }

            // Bit Operations
            Instruction::BIT(bit, target) => {
                let data = self.registers.read(target);
                self.bit(bit, data)
            }
            Instruction::BITHL(bit) => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let data = self.bus.read(addr);
                self.bit(bit, data)
            }
            Instruction::SET(bit, target) => {
                let mut data = self.registers.read(target);
                self.set(bit, &mut data);
                self.registers.write(target, data);
            }
            Instruction::SETHL(bit) => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.set(bit, &mut data);
                self.bus.write(addr, data);
            }
            Instruction::RESET(bit, target) => {
                let mut data = self.registers.read(target);
                self.reset(bit, &mut data);
                self.reset(bit, &mut data);
            }
            Instruction::RESETHL(bit) => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.reset(bit, &mut data);
                self.bus.write(addr, data);
            }
            Instruction::SWAP(target) => {
                let mut data = self.registers.read(target);
                self.swap(&mut data);
                self.registers.write(target, data);
            }
            Instruction::SWAPHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.swap(&mut data);
                self.bus.write(addr, data);
            }

            // Bit Shifts
            Instruction::SRL(target) => {
                let mut data = self.registers.read(target);
                self.srl(&mut data);
                self.registers.write(target, data);
            }
            Instruction::SRLHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.srl(&mut data);
                self.bus.write(addr, data);
            }
            Instruction::SRA(target) => {
                let mut data = self.registers.read(target);
                self.sra(&mut data);
                self.registers.write(target, data);
            }
            Instruction::SRAHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.sra(&mut data);
                self.bus.write(addr, data);
            }
            Instruction::SLA(target) => {
                let mut data = self.registers.read(target);
                self.sla(&mut data);
                self.registers.write(target, data);
            }
            Instruction::SLAHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.sla(&mut data);
                self.bus.write(addr, data);
            }
            Instruction::RRA => {
                let mut data = self.registers.read(Reg8::A);
                self.rr(&mut data);
                self.registers.write(Reg8::A, data);
            }
            Instruction::RLA => {
                let mut data = self.registers.read(Reg8::A);
                self.rl(&mut data);
                self.registers.write(Reg8::A, data);
            }
            Instruction::RRCA => {
                let mut data = self.registers.read(Reg8::A);
                self.rrc(&mut data);
                self.registers.write(Reg8::A, data);
            }
            Instruction::RLCA => {
                let mut data = self.registers.read(Reg8::A);
                self.rlc(&mut data);
                self.registers.write(Reg8::A, data);
            }
            Instruction::RR(target) => {
                let mut data = self.registers.read(target);
                self.rr(&mut data);
                self.registers.write(target, data);
            }
            Instruction::RL(target) => {
                let mut data = self.registers.read(target);
                self.rl(&mut data);
                self.registers.write(target, data);
            }
            Instruction::RRC(target) => {
                let mut data = self.registers.read(target);
                self.rrc(&mut data);
                self.registers.write(target, data);
            }
            Instruction::RLC(target) => {
                let mut data = self.registers.read(target);
                self.rlc(&mut data);
                self.registers.write(target, data);
            }
            Instruction::RRHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.rr(&mut data);
                self.bus.write(addr, data);
            }
            Instruction::RLHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.rl(&mut data);
                self.bus.write(addr, data);
            }
            Instruction::RRCHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.rrc(&mut data);
                self.bus.write(addr, data);
            }
            Instruction::RLCHL => {
                let addr = self.registers.read(Reg16::HL) as usize;
                let mut data = self.bus.read(addr);
                self.rlc(&mut data);
                self.bus.write(addr, data);
            }

            // Misc Operations
            Instruction::CCF => self.ccf(),
            Instruction::SCF => self.scf(),
            Instruction::CPL => self.cpl(),
            Instruction::DAA => self.daa(),
            Instruction::NOP => self.nop(),
            Instruction::HALT => {
                dbg!("halted @ 0x{:04X}", self.registers.pc.pointer.0);
                self.mode = Mode::Halted;
            }
            Instruction::STOP => self.stop(),
            Instruction::DI => self.di(),
            Instruction::EI => self.ei(),

            // Control Operations
            Instruction::JP => self.jp(),
            Instruction::JPHL => {
                let addr = self.registers.read(Reg16::HL);
                self.registers.pc.write(&mut self.bus, addr);
            }
            Instruction::JPCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.jp()
                } else {
                    self.registers.pc.pointer += 2;
                }
            }
            Instruction::JR => self.jr(),
            Instruction::JRCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.jr()
                } else {
                    self.registers.pc.pointer += 1;
                }
            }
            Instruction::CALL => self.call(),
            Instruction::CALLCC(condition) => {
                if self.evaluate_condition(condition) {
                    self.call()
                } else {
                    self.registers.pc.pointer += 2;
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
                self.registers
                    .sp
                    .write(&mut self.bus, self.registers.pc.pointer.0);

                self.registers.pc.pointer.0 = target.into();
            }

            // Prefix Operations
            Instruction::PREFIXCB => {
                let opcode = self.registers.pc.read(&mut self.bus) as u8;

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
    }

    fn ccf(&mut self) {
        let carry = self.registers.get_flags().carry;

        self.registers.set_flags(Flags {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: !carry,
        });
    }

    fn scf(&mut self) {
        let zero = self.registers.get_flags().zero;

        self.registers.set_flags(Flags {
            zero,
            subtract: false,
            half_carry: false,
            carry: true,
        });
    }

    fn rr(&mut self, data: &mut u8) {
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rr(*data, carry);

        *data = result;

        self.registers.set_flags(flags);
    }

    fn rl(&mut self, data: &mut u8) {
        let carry = self.registers.get_flags().carry as u8;
        let (result, flags) = self.alu.rl(*data, carry);

        *data = result;

        self.registers.set_flags(flags);
    }

    fn rrc(&mut self, data: &mut u8) {
        let (result, flags) = self.alu.rrc(*data);

        *data = result;

        self.registers.set_flags(flags);
    }

    fn rlc(&mut self, data: &mut u8) {
        let (result, flags) = self.alu.rlc(*data);

        *data = result;

        self.registers.set_flags(flags);
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
    }

    fn reset(&mut self, bit: u8, data: &mut u8) {
        *data = *data & (0 << bit);
    }

    fn set(&mut self, bit: u8, data: &mut u8) {
        *data = *data | (1 << bit);
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
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn ei(&mut self) {
        self.ime = true;
    }

    fn nop(&mut self) {}

    fn stop(&mut self) {
        panic!("STOP instruction received");
    }

    fn push_16(&mut self, data: u16) {
        self.registers.sp.write(&mut self.bus, data);
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
        let lower = self.registers.pc.read(&mut self.bus) as u8;
        let upper = self.registers.pc.read(&mut self.bus) as u8;

        let addr = u16::from_le_bytes([lower, upper]);
        self.registers.pc.write(&mut self.bus, addr);
    }

    fn jr(&mut self) {
        let offset = self.registers.pc.read(&mut self.bus) as i8;

        if offset < 0 {
            self.registers.pc.pointer -= (-offset) as u16;
        } else {
            self.registers.pc.pointer += offset as u16;
        }
    }

    fn call(&mut self) {
        let lower = self.registers.pc.read(&mut self.bus) as u8;
        let upper = self.registers.pc.read(&mut self.bus) as u8;

        let data = u16::from_le_bytes([lower, upper]);

        self.registers.sp.write(&mut self.bus, data);
        self.registers.pc.write(&mut self.bus, data);
    }

    fn ret(&mut self) {
        let addr = self.registers.sp.read(&mut self.bus);

        self.registers.pc.write(&mut self.bus, addr);
    }
}
