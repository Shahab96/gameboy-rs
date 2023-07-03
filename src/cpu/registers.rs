// The operations represented by the following functions are described here:
// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html#the-flags-register-lower-8-bits-of-af-register
const ZERO_FLAG: u8 = 0b1000_0000;
const SUBTRACT_FLAG: u8 = 0b0100_0000;
const HALF_CARRY_FLAG: u8 = 0b0010_0000;
const CARRY_FLAG: u8 = 0b0001_0000;

#[derive(Copy, Clone)]
pub struct Registers {
    // Program counter
    pub pc: u16,

    // Stack pointer
    pub sp: u16,

    // The 8-bit registers
    data: [u8; 8],
}

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

#[derive(Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

/*
 * We will implement getters and setters for the AF, BC, DE and HL registers, since they are used
 * for 16-bit arithmetic operations.
 *
 * A, B, D and H are the high bytes of the AF, BC, DE and HL registers respectively.
 * F, C, E and L are the low bytes of the AF, BC, DE and HL registers respectively.
 *
 * To get the value of a 16-bit register, we need to shift the high byte 8 bits to the left and
 * OR it with the low byte.
 *
 * To set the value of a 16-bit register, we need to shift the provided value 8 bits to the right
 * and AND it with 0xFF to get the high byte. Then we need to AND the lower 8 bits with the
 * provided value to get the low byte.
 */
#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            data: [0; 8],
        }
    }

    pub fn read(&self, register: Reg8) -> u8 {
        match register {
            Reg8::A => self.data[0],
            Reg8::B => self.data[1],
            Reg8::C => self.data[2],
            Reg8::D => self.data[3],
            Reg8::E => self.data[4],
            Reg8::F => self.data[5],
            Reg8::H => self.data[6],
            Reg8::L => self.data[7],
        }
    }

    pub fn read16(&self, register: Reg16) -> u16 {
        match register {
            Reg16::AF => ((self.data[0] as u16) << 8) | (self.data[5] as u16),
            Reg16::BC => ((self.data[1] as u16) << 8) | (self.data[2] as u16),
            Reg16::DE => ((self.data[3] as u16) << 8) | (self.data[4] as u16),
            Reg16::HL => ((self.data[6] as u16) << 8) | (self.data[7] as u16),
        }
    }

    pub fn write(&mut self, register: Reg8, value: u8) {
        match register {
            Reg8::A => self.data[0] = value,
            Reg8::B => self.data[1] = value,
            Reg8::C => self.data[2] = value,
            Reg8::D => self.data[3] = value,
            Reg8::E => self.data[4] = value,
            Reg8::F => self.data[5] = value,
            Reg8::H => self.data[6] = value,
            Reg8::L => self.data[7] = value,
        }
    }

    pub fn write16(&mut self, register: Reg16, value: u16) {
        match register {
            Reg16::AF => {
                self.data[0] = ((value & 0xFF00) >> 8) as u8;
                self.data[5] = (value & 0x00FF) as u8;
            }
            Reg16::BC => {
                self.data[1] = ((value & 0xFF00) >> 8) as u8;
                self.data[2] = (value & 0x00FF) as u8;
            }
            Reg16::DE => {
                self.data[3] = ((value & 0xFF00) >> 8) as u8;
                self.data[4] = (value & 0x00FF) as u8;
            }
            Reg16::HL => {
                self.data[6] = ((value & 0xFF00) >> 8) as u8;
                self.data[7] = (value & 0x00FF) as u8;
            }
        }
    }

    pub fn set_flags(&mut self, flags: Flags) {
        if flags.zero {
            self.data[5] |= ZERO_FLAG;
        }

        if flags.subtract {
            self.data[5] |= SUBTRACT_FLAG;
        }

        if flags.half_carry {
            self.data[5] |= HALF_CARRY_FLAG;
        }

        if flags.carry {
            self.data[5] |= CARRY_FLAG;
        }
    }

    pub fn get_flags(&self) -> Flags {
        Flags {
            zero: self.data[5] & ZERO_FLAG != 0,
            subtract: self.data[5] & SUBTRACT_FLAG != 0,
            half_carry: self.data[5] & HALF_CARRY_FLAG != 0,
            carry: self.data[5] & CARRY_FLAG != 0,
        }
    }
}
