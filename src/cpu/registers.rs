type Byte = u8;
type Word = u16;

struct Registers {
    a: Byte,
    b: Byte,
    c: Byte,
    d: Byte,
    e: Byte,
    f: Byte,
    h: Byte,
    l: Byte,
}

impl Registers {
    fn get_bc(&self) -> Word {
        (self.b as Word) << 8 | (self.c as Word)
    }

    fn set_bc(&mut self, value: Word) {
        self.b = ((value & 0xFF00) >> 8) as Byte;
        self.c = (value & 0x00FF) as Byte;
    }
}

struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: Byte = 7;
const SUBTRACT_FLAG_BYTE_POSITION: Byte = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: Byte = 5;
const CARRY_FLAG_BYTE_POSITION: Byte = 4;

impl std::convert::From<FlagRegister> for Byte {
    fn from(flag: FlagRegister) -> Byte {
        let mut result: Byte = 0;

        if flag.zero {
            // Set the bit at position 7 to 1
            result |= 1 << ZERO_FLAG_BYTE_POSITION;
        }

        if flag.subtract {
            // Set the bit at position 6 to 1
            result |= 1 << SUBTRACT_FLAG_BYTE_POSITION;
        }

        if flag.half_carry {
            // Set the bit at position 5 to 1
            result |= 1 << HALF_CARRY_FLAG_BYTE_POSITION;
        }

        if flag.carry {
            // Set the bit at position 4 to 1
            result |= 1 << CARRY_FLAG_BYTE_POSITION;
        }

        result
    }
}
