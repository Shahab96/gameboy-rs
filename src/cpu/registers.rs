use super::{Byte, Word};

pub(super) struct Registers {
    pub(super) a: Byte,
    pub(super) b: Byte,
    pub(super) c: Byte,
    pub(super) d: Byte,
    pub(super) e: Byte,
    // The F register is a special one, see the FlagRegister struct below
    pub(super) f: FlagRegister,
    pub(super) h: Byte,
    pub(super) l: Byte,
}

/*
 * We will implement getters and setters for the BC, DE and HL registers, since they are used
 * for 16-bit arithmetic operations.
 *
 * B, D and H are the high bytes of the BC, DE and HL registers respectively.
 * C, E and L are the low bytes of the BC, DE and HL registers respectively.
 *
 * To get the value of a 16-bit register, we need to shift the high byte 8 bits to the left and
 * OR it with the low byte.
 *
 * To set the value of a 16-bit register, we need to shift the provided value 8 bits to the right
 * and AND it with 0xFF to get the high byte. Then we need to AND the lower 8 bits with the
 * provided value to get the low byte.
 */
impl Registers {
    fn get_word(&self, high_byte: Byte, low_byte: Byte) -> Word {
        (high_byte as Word) << 8 | (low_byte as Word)
    }

    fn set_word(&self, value: Word) -> (Byte, Byte) {
        let high = ((value & 0xFF00) >> 8) as Byte;
        let low = (value & 0x00FF) as Byte;

        (high, low)
    }

    fn get_bc(&self) -> Word {
        self.get_word(self.b, self.c)
    }

    fn set_bc(&mut self, value: Word) {
        let (high, low) = self.set_word(value);

        self.b = high;
        self.c = low;
    }

    fn get_de(&self) -> Word {
        self.get_word(self.d, self.e)
    }

    fn set_de(&mut self, value: Word) {
        let (high, low) = self.set_word(value);

        self.d = high;
        self.e = low;
    }

    pub(crate) fn get_hl(&self) -> Word {
        self.get_word(self.h, self.l)
    }

    pub(crate) fn set_hl(&mut self, value: Word) {
        let (high, low) = self.set_word(value);

        self.h = high;
        self.l = low;
    }
}

// This struct abstracts the operations on the flag register
pub(super) struct FlagRegister {
    pub(super) zero: bool,
    pub(super) subtract: bool,

    /*
     * Half Carry is set if adding the lower nibbles of the value and register A
     * together result in a value bigger than 0xF. If the result is larger than 0xF
     * than the addition caused a carry from the lower nibble to the upper nibble.
     */
    pub(super) half_carry: bool,
    pub(super) carry: bool,
}

// The operations represented by the following functions are described here:
// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html#the-flags-register-lower-8-bits-of-af-register
const ZERO_FLAG: Byte = 0b1000_0000;
const SUBTRACT_FLAG: Byte = 0b0100_0000;
const HALF_CARRY_FLAG: Byte = 0b0010_0000;
const CARRY_FLAG: Byte = 0b0001_0000;

impl std::convert::From<FlagRegister> for Byte {
    fn from(flag: FlagRegister) -> Byte {
        // We will construct the byte by performing bitwise operations.
        let mut result: Byte = 0;

        if flag.zero {
            // Set the bit at position 7 to 1
            result |= ZERO_FLAG;
        }

        if flag.subtract {
            // Set the bit at position 6 to 1
            result |= SUBTRACT_FLAG;
        }

        if flag.half_carry {
            // Set the bit at position 5 to 1
            result |= HALF_CARRY_FLAG;
        }

        if flag.carry {
            // Set the bit at position 4 to 1
            result |= CARRY_FLAG;
        }

        result
    }
}
