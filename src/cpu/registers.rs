use std::num::Wrapping;

use crate::{memory::bus::MemoryBus, utils::traits::Storage};

// The operations represented by the following functions are described here:
// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html#the-flags-register-lower-8-bits-of-af-register
const ZERO_FLAG: u8 = 0b1000_0000;
const SUBTRACT_FLAG: u8 = 0b0100_0000;
const HALF_CARRY_FLAG: u8 = 0b0010_0000;
const CARRY_FLAG: u8 = 0b0001_0000;

#[derive(Debug)]
pub struct ProgramCounter {
    pub pointer: Wrapping<u16>,
}

impl Storage<&mut MemoryBus, u16> for ProgramCounter {
    fn read(&mut self, src: &mut MemoryBus) -> u16 {
        let addr = self.pointer.0 as usize;
        let data: u8 = src.read(addr);

        self.pointer += 1;

        data as u16
    }

    fn write(&mut self, _: &mut MemoryBus, value: u16) {
        self.pointer.0 = value;
    }
}

#[derive(Debug)]
pub struct StackPointer {
    pub pointer: Wrapping<u16>,
}

impl Storage<&mut MemoryBus, u16> for StackPointer {
    fn read(&mut self, src: &mut MemoryBus) -> u16 {
        let data: u8 = src.read(self.pointer.0 as usize);

        self.pointer += 2;

        data as u16
    }

    fn write(&mut self, dest: &mut MemoryBus, value: u16) {
        self.pointer -= 2;
        dest.write(self.pointer.0 as usize, value);
    }
}

#[derive(Debug)]
pub struct Registers {
    // Program Counter
    pub pc: ProgramCounter,

    // Stack pointer
    pub sp: StackPointer,

    // The 8-bit registers
    data: [u8; 8],
}

pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
}

impl Storage<Reg8, u8> for Registers {
    fn read(&mut self, src: Reg8) -> u8 {
        match src {
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

    fn write(&mut self, dest: Reg8, value: u8) {
        match dest {
            Reg8::A => self.data[0] = value,
            Reg8::B => self.data[1] = value,
            Reg8::C => self.data[2] = value,
            Reg8::D => self.data[3] = value,
            Reg8::E => self.data[4] = value,
            Reg8::F => panic!("Attempted a direct write to the F register!"),
            Reg8::H => self.data[6] = value,
            Reg8::L => self.data[7] = value,
        }
    }
}

impl Storage<Reg16, u16> for Registers {
    fn read(&mut self, src: Reg16) -> u16 {
        match src {
            Reg16::AF => u16::from_le_bytes([self.data[0], self.data[5]]),
            Reg16::BC => u16::from_le_bytes([self.data[1], self.data[2]]),
            Reg16::DE => u16::from_le_bytes([self.data[3], self.data[4]]),
            Reg16::HL => u16::from_le_bytes([self.data[6], self.data[7]]),
        }
    }

    fn write(&mut self, dest: Reg16, value: u16) {
        let [high, low] = value.to_le_bytes();

        match dest {
            Reg16::AF => {
                self.data[0] = high;
                self.data[5] = low;
            }
            Reg16::BC => {
                self.data[1] = high;
                self.data[2] = low;
            }
            Reg16::DE => {
                self.data[3] = high;
                self.data[4] = low;
            }
            Reg16::HL => {
                self.data[6] = high;
                self.data[7] = low;
            }
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            sp: StackPointer {
                pointer: Wrapping(0xFFFF),
            },
            pc: ProgramCounter {
                pointer: Wrapping(0x100),
            },
            data: [0; 8],
        }
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.data[5] = 0;

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
