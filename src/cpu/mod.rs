// Type aliasing for readability
type Byte = u8;

// Type alias 2 bytes to a word
type Word = u16;

pub mod instructions;
pub mod registers;

use self::instructions::{ArithmeticTarget, Instruction};
use self::registers::Registers;

struct CPU {
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => self.registers.a = self.add(self.registers.c),
                // TODO: Support other registers
                _ => unimplemented!(),
            },
            Instruction::SUB(target) => match target {
                ArithmeticTarget::C => self.registers.a = self.sub(self.registers.c),
                // TODO: Support other registers
                _ => unimplemented!(),
            },
            Instruction::AND(target) => match target {
                ArithmeticTarget::C => self.registers.a = self.and(self.registers.c),
                // TODO: Support other registers
                _ => unimplemented!(),
            },
            Instruction::OR(target) => match target {
                ArithmeticTarget::C => self.registers.a = self.or(self.registers.c),
                // TODO: Support other registers
                _ => unimplemented!(),
            },
            Instruction::XOR(target) => match target {
                ArithmeticTarget::C => self.registers.a = self.xor(self.registers.c),
                // TODO: Support other registers
                _ => unimplemented!(),
            },
            Instruction::CP(target) => match target {
                ArithmeticTarget::C => {
                    self.sub(self.registers.c);

                    // We merely needed to set the flags, so we can just return an empty unit here
                    ()
                }
                // TODO: Support other registers
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    // This function will add the provided value to the A register
    fn add(&mut self, value: Byte) -> Byte {
        // We want to know if the addition overflows, so we will use the overflowing_add function
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn sub(&mut self, value: Byte) -> Byte {
        // We want to know if the subtraction underflows, so we will use the overflowing_sub function
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_underflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);

        new_value
    }

    fn and(&mut self, value: Byte) -> Byte {
        let new_value = self.registers.a & value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = true;

        new_value
    }

    fn or(&mut self, value: Byte) -> Byte {
        let new_value = self.registers.a | value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn xor(&mut self, value: Byte) -> Byte {
        let new_value = self.registers.a ^ value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = false;
        self.registers.f.half_carry = false;

        new_value
    }

    fn adc(&mut self, value: Byte) -> Byte {
        let carry_flag = self.registers.f.carry as Byte;

        // We will use wrapping_add here because we don't care about the overflow
        let new_value = self
            .registers
            .a
            .wrapping_add(value)
            .wrapping_add(carry_flag);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry =
            (self.registers.a as u16) + (value as u16) + (carry_flag as u16) > 0xFF;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + carry_flag > 0xF;

        new_value
    }

    fn sbc(&mut self, value: Byte) -> Byte {
        let carry_flag = self.registers.f.carry as Byte;

        // We will use wrapping_sub here because we don't care about the overflow
        let new_value = self
            .registers
            .a
            .wrapping_sub(value)
            .wrapping_sub(carry_flag);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xf)
            .wrapping_sub(value & 0xf)
            .wrapping_sub(carry_flag)
            & (0xf + 1)
            != 0;
        self.registers.f.carry = (self.registers.a as u16) < (value as u16) + (carry_flag as u16);

        new_value
    }

    fn add_hl(&mut self, value: Word) {
        let hl = self.registers.get_hl();

        // We will use wrapping_add here because we don't care about the overflow
        let new_value = hl.wrapping_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = hl > 0xFFFF - value;
        self.registers.f.half_carry = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;

        self.registers.set_hl(new_value);
    }

    fn inc(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                self.registers.a = self.add(1);
            }
            ArithmeticTarget::B => {
                self.registers.b = self.add(1);
            }
            ArithmeticTarget::C => {
                self.registers.c = self.add(1);
            }
            ArithmeticTarget::D => {
                self.registers.d = self.add(1);
            }
            ArithmeticTarget::E => {
                self.registers.e = self.add(1);
            }
            ArithmeticTarget::H => {
                self.registers.h = self.add(1);
            }
            ArithmeticTarget::L => {
                self.registers.l = self.add(1);
            }
        }
    }

    fn dec(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                self.registers.a = self.sub(1);
            }
            ArithmeticTarget::B => {
                self.registers.b = self.sub(1);
            }
            ArithmeticTarget::C => {
                self.registers.c = self.sub(1);
            }
            ArithmeticTarget::D => {
                self.registers.d = self.sub(1);
            }
            ArithmeticTarget::E => {
                self.registers.e = self.sub(1);
            }
            ArithmeticTarget::H => {
                self.registers.h = self.sub(1);
            }
            ArithmeticTarget::L => {
                self.registers.l = self.sub(1);
            }
        }
    }

    fn ccf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = !self.registers.f.carry;
    }

    fn scf(&mut self) {
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    fn rra(&mut self) {
        let carry_flag = self.registers.f.carry as Byte;

        self.registers.a = (self.registers.a >> 1) | (carry_flag << 7);

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = self.registers.a & 0x01 != 0;
    }

    fn rla(&mut self) {
        let carry_flag = self.registers.f.carry as Byte;

        self.registers.a = (self.registers.a << 1) | carry_flag;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = self.registers.a & 0x80 != 0;
    }

    fn rrca(&mut self) {
        self.registers.a = self.registers.a.rotate_right(1);

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = self.registers.a & 0x01 != 0;
    }

    fn rlca(&mut self) {
        self.registers.a = self.registers.a.rotate_left(1);

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = self.registers.a & 0x80 != 0;
    }

    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
    }

    fn bit(&mut self, bit: u8, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        };

        self.registers.f.zero = (value & (1 << bit)) == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    fn reset(&mut self, bit: u8, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => self.registers.a = self.registers.a & (0 << bit),
            ArithmeticTarget::B => self.registers.b = self.registers.b & (0 << bit),
            ArithmeticTarget::C => self.registers.c = self.registers.c & (0 << bit),
            ArithmeticTarget::D => self.registers.d = self.registers.d & (0 << bit),
            ArithmeticTarget::E => self.registers.e = self.registers.e & (0 << bit),
            ArithmeticTarget::H => self.registers.h = self.registers.h & (0 << bit),
            ArithmeticTarget::L => self.registers.l = self.registers.l & (0 << bit),
        };
    }

    fn set(&mut self, bit: u8, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => self.registers.a = self.registers.a | (1 << bit),
            ArithmeticTarget::B => self.registers.b = self.registers.b | (1 << bit),
            ArithmeticTarget::C => self.registers.c = self.registers.c | (1 << bit),
            ArithmeticTarget::D => self.registers.d = self.registers.d | (1 << bit),
            ArithmeticTarget::E => self.registers.e = self.registers.e | (1 << bit),
            ArithmeticTarget::H => self.registers.h = self.registers.h | (1 << bit),
            ArithmeticTarget::L => self.registers.l = self.registers.l | (1 << bit),
        };
    }

    fn srl(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => self.registers.a = self.registers.a >> 1,
            ArithmeticTarget::B => self.registers.b = self.registers.b >> 1,
            ArithmeticTarget::C => self.registers.c = self.registers.c >> 1,
            ArithmeticTarget::D => self.registers.d = self.registers.d >> 1,
            ArithmeticTarget::E => self.registers.e = self.registers.e >> 1,
            ArithmeticTarget::H => self.registers.h = self.registers.h >> 1,
            ArithmeticTarget::L => self.registers.l = self.registers.l >> 1,
        };

        match target {
            ArithmeticTarget::A => {
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.carry = self.registers.a & 0x01 != 0;
            }
            ArithmeticTarget::B => {
                self.registers.f.zero = self.registers.b == 0;
                self.registers.f.carry = self.registers.b & 0x01 != 0;
            }
            ArithmeticTarget::C => {
                self.registers.f.zero = self.registers.c == 0;
                self.registers.f.carry = self.registers.c & 0x01 != 0;
            }
            ArithmeticTarget::D => {
                self.registers.f.zero = self.registers.d == 0;
                self.registers.f.carry = self.registers.d & 0x01 != 0;
            }
            ArithmeticTarget::E => {
                self.registers.f.zero = self.registers.e == 0;
                self.registers.f.carry = self.registers.e & 0x01 != 0;
            }
            ArithmeticTarget::H => {
                self.registers.f.zero = self.registers.h == 0;
                self.registers.f.carry = self.registers.h & 0x01 != 0;
            }
            ArithmeticTarget::L => {
                self.registers.f.zero = self.registers.l == 0;
                self.registers.f.carry = self.registers.l & 0x01 != 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn rr(&mut self, target: ArithmeticTarget) {
        let carry_flag = self.registers.f.carry as Byte;

        match target {
            ArithmeticTarget::A => {
                self.registers.a = (self.registers.a >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.carry = self.registers.a & 0x01 != 0;
            }
            ArithmeticTarget::B => {
                self.registers.b = (self.registers.b >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.b == 0;
                self.registers.f.carry = self.registers.b & 0x01 != 0;
            }
            ArithmeticTarget::C => {
                self.registers.c = (self.registers.c >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.c == 0;
                self.registers.f.carry = self.registers.c & 0x01 != 0;
            }
            ArithmeticTarget::D => {
                self.registers.d = (self.registers.d >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.d == 0;
                self.registers.f.carry = self.registers.d & 0x01 != 0;
            }
            ArithmeticTarget::E => {
                self.registers.e = (self.registers.e >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.e == 0;
                self.registers.f.carry = self.registers.e & 0x01 != 0;
            }
            ArithmeticTarget::H => {
                self.registers.h = (self.registers.h >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.h == 0;
                self.registers.f.carry = self.registers.h & 0x01 != 0;
            }
            ArithmeticTarget::L => {
                self.registers.l = (self.registers.l >> 1) | (carry_flag << 7);
                self.registers.f.zero = self.registers.l == 0;
                self.registers.f.carry = self.registers.l & 0x01 != 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn rl(&mut self, target: ArithmeticTarget) {
        let carry_flag = self.registers.f.carry as Byte;

        match target {
            ArithmeticTarget::A => {
                self.registers.a = (self.registers.a << 1) | carry_flag;
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.carry = self.registers.a & 0x80 != 0;
            }
            ArithmeticTarget::B => {
                self.registers.b = (self.registers.b << 1) | carry_flag;
                self.registers.f.zero = self.registers.b == 0;
                self.registers.f.carry = self.registers.b & 0x80 != 0;
            }
            ArithmeticTarget::C => {
                self.registers.c = (self.registers.c << 1) | carry_flag;
                self.registers.f.zero = self.registers.c == 0;
                self.registers.f.carry = self.registers.c & 0x80 != 0;
            }
            ArithmeticTarget::D => {
                self.registers.d = (self.registers.d << 1) | carry_flag;
                self.registers.f.zero = self.registers.d == 0;
                self.registers.f.carry = self.registers.d & 0x80 != 0;
            }
            ArithmeticTarget::E => {
                self.registers.e = (self.registers.e << 1) | carry_flag;
                self.registers.f.zero = self.registers.e == 0;
                self.registers.f.carry = self.registers.e & 0x80 != 0;
            }
            ArithmeticTarget::H => {
                self.registers.h = (self.registers.h << 1) | carry_flag;
                self.registers.f.zero = self.registers.h == 0;
                self.registers.f.carry = self.registers.h & 0x80 != 0;
            }
            ArithmeticTarget::L => {
                self.registers.l = (self.registers.l << 1) | carry_flag;
                self.registers.f.zero = self.registers.l == 0;
                self.registers.f.carry = self.registers.l & 0x80 != 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn rrc(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                self.registers.a = self.registers.a.rotate_right(1);
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.carry = self.registers.a & 0x01 != 0;
            }
            ArithmeticTarget::B => {
                self.registers.b = self.registers.b.rotate_right(1);
                self.registers.f.zero = self.registers.b == 0;
                self.registers.f.carry = self.registers.b & 0x01 != 0;
            }
            ArithmeticTarget::C => {
                self.registers.c = self.registers.c.rotate_right(1);
                self.registers.f.zero = self.registers.c == 0;
                self.registers.f.carry = self.registers.c & 0x01 != 0;
            }
            ArithmeticTarget::D => {
                self.registers.d = self.registers.d.rotate_right(1);
                self.registers.f.zero = self.registers.d == 0;
                self.registers.f.carry = self.registers.d & 0x01 != 0;
            }
            ArithmeticTarget::E => {
                self.registers.e = self.registers.e.rotate_right(1);
                self.registers.f.zero = self.registers.e == 0;
                self.registers.f.carry = self.registers.e & 0x01 != 0;
            }
            ArithmeticTarget::H => {
                self.registers.h = self.registers.h.rotate_right(1);
                self.registers.f.zero = self.registers.h == 0;
                self.registers.f.carry = self.registers.h & 0x01 != 0;
            }
            ArithmeticTarget::L => {
                self.registers.l = self.registers.l.rotate_right(1);
                self.registers.f.zero = self.registers.l == 0;
                self.registers.f.carry = self.registers.l & 0x01 != 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn rlc(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                self.registers.a = self.registers.a.rotate_left(1);
                self.registers.f.zero = self.registers.a == 0;
                self.registers.f.carry = self.registers.a & 0x80 != 0;
            }
            ArithmeticTarget::B => {
                self.registers.b = self.registers.b.rotate_left(1);
                self.registers.f.zero = self.registers.b == 0;
                self.registers.f.carry = self.registers.b & 0x80 != 0;
            }
            ArithmeticTarget::C => {
                self.registers.c = self.registers.c.rotate_left(1);
                self.registers.f.zero = self.registers.c == 0;
                self.registers.f.carry = self.registers.c & 0x80 != 0;
            }
            ArithmeticTarget::D => {
                self.registers.d = self.registers.d.rotate_left(1);
                self.registers.f.zero = self.registers.d == 0;
                self.registers.f.carry = self.registers.d & 0x80 != 0;
            }
            ArithmeticTarget::E => {
                self.registers.e = self.registers.e.rotate_left(1);
                self.registers.f.zero = self.registers.e == 0;
                self.registers.f.carry = self.registers.e & 0x80 != 0;
            }
            ArithmeticTarget::H => {
                self.registers.h = self.registers.h.rotate_left(1);
                self.registers.f.zero = self.registers.h == 0;
                self.registers.f.carry = self.registers.h & 0x80 != 0;
            }
            ArithmeticTarget::L => {
                self.registers.l = self.registers.l.rotate_left(1);
                self.registers.f.zero = self.registers.l == 0;
                self.registers.f.carry = self.registers.l & 0x80 != 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn sra(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                let sign_bit = self.registers.a & 0x80;
                self.registers.f.carry = self.registers.a & 0x01 != 0;
                self.registers.a = (self.registers.a >> 1) | sign_bit;
                self.registers.f.zero = self.registers.a == 0;
            }
            ArithmeticTarget::B => {
                let sign_bit = self.registers.b & 0x80;
                self.registers.f.carry = self.registers.b & 0x01 != 0;
                self.registers.b = (self.registers.b >> 1) | sign_bit;
                self.registers.f.zero = self.registers.b == 0;
            }
            ArithmeticTarget::C => {
                let sign_bit = self.registers.c & 0x80;
                self.registers.f.carry = self.registers.c & 0x01 != 0;
                self.registers.c = (self.registers.c >> 1) | sign_bit;
                self.registers.f.zero = self.registers.c == 0;
            }
            ArithmeticTarget::D => {
                let sign_bit = self.registers.d & 0x80;
                self.registers.f.carry = self.registers.d & 0x01 != 0;
                self.registers.d = (self.registers.d >> 1) | sign_bit;
                self.registers.f.zero = self.registers.d == 0;
            }
            ArithmeticTarget::E => {
                let sign_bit = self.registers.e & 0x80;
                self.registers.f.carry = self.registers.e & 0x01 != 0;
                self.registers.e = (self.registers.e >> 1) | sign_bit;
                self.registers.f.zero = self.registers.e == 0;
            }
            ArithmeticTarget::H => {
                let sign_bit = self.registers.h & 0x80;
                self.registers.f.carry = self.registers.h & 0x01 != 0;
                self.registers.h = (self.registers.h >> 1) | sign_bit;
                self.registers.f.zero = self.registers.h == 0;
            }
            ArithmeticTarget::L => {
                let sign_bit = self.registers.l & 0x80;
                self.registers.f.carry = self.registers.l & 0x01 != 0;
                self.registers.l = (self.registers.l >> 1) | sign_bit;
                self.registers.f.zero = self.registers.l == 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn sla(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                self.registers.f.carry = self.registers.a & 0x80 != 0;
                self.registers.a <<= 1;
                self.registers.f.zero = self.registers.a == 0;
            }
            ArithmeticTarget::B => {
                self.registers.f.carry = self.registers.b & 0x80 != 0;
                self.registers.b <<= 1;
                self.registers.f.zero = self.registers.b == 0;
            }
            ArithmeticTarget::C => {
                self.registers.f.carry = self.registers.c & 0x80 != 0;
                self.registers.c <<= 1;
                self.registers.f.zero = self.registers.c == 0;
            }
            ArithmeticTarget::D => {
                self.registers.f.carry = self.registers.d & 0x80 != 0;
                self.registers.d <<= 1;
                self.registers.f.zero = self.registers.d == 0;
            }
            ArithmeticTarget::E => {
                self.registers.f.carry = self.registers.e & 0x80 != 0;
                self.registers.e <<= 1;
                self.registers.f.zero = self.registers.e == 0;
            }
            ArithmeticTarget::H => {
                self.registers.f.carry = self.registers.h & 0x80 != 0;
                self.registers.h <<= 1;
                self.registers.f.zero = self.registers.h == 0;
            }
            ArithmeticTarget::L => {
                self.registers.f.carry = self.registers.l & 0x80 != 0;
                self.registers.l <<= 1;
                self.registers.f.zero = self.registers.l == 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn swap(&mut self, target: ArithmeticTarget) {
        match target {
            ArithmeticTarget::A => {
                self.registers.a = (self.registers.a << 4) | (self.registers.a >> 4);
                self.registers.f.zero = self.registers.a == 0;
            }
            ArithmeticTarget::B => {
                self.registers.b = (self.registers.b << 4) | (self.registers.b >> 4);
                self.registers.f.zero = self.registers.b == 0;
            }
            ArithmeticTarget::C => {
                self.registers.c = (self.registers.c << 4) | (self.registers.c >> 4);
                self.registers.f.zero = self.registers.c == 0;
            }
            ArithmeticTarget::D => {
                self.registers.d = (self.registers.d << 4) | (self.registers.d >> 4);
                self.registers.f.zero = self.registers.d == 0;
            }
            ArithmeticTarget::E => {
                self.registers.e = (self.registers.e << 4) | (self.registers.e >> 4);
                self.registers.f.zero = self.registers.e == 0;
            }
            ArithmeticTarget::H => {
                self.registers.h = (self.registers.h << 4) | (self.registers.h >> 4);
                self.registers.f.zero = self.registers.h == 0;
            }
            ArithmeticTarget::L => {
                self.registers.l = (self.registers.l << 4) | (self.registers.l >> 4);
                self.registers.f.zero = self.registers.l == 0;
            }
        };

        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
    }
}
