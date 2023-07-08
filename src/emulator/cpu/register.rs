pub struct Registers {
    pub pc: u16,
    pub sp: u16,

    pub data: [u8; 8],
}

pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

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

pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0x1000,
            sp: 0xFFFF,
            data: [0; 8],
        }
    }

    pub fn read_8(&self, src: Reg8) -> u8 {
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

    pub fn read_16(&self, src: Reg16) -> u16 {
        match src {
            Reg16::AF => ((self.data[0] as u16) << 8) | (self.data[5] as u16),
            Reg16::BC => ((self.data[1] as u16) << 8) | (self.data[2] as u16),
            Reg16::DE => ((self.data[3] as u16) << 8) | (self.data[4] as u16),
            Reg16::HL => ((self.data[6] as u16) << 8) | (self.data[7] as u16),
            Reg16::SP => self.sp,
        }
    }

    pub fn write_8(&mut self, dest: Reg8, value: u8) {
        match dest {
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

    pub fn write_16(&mut self, dest: Reg16, value: u16) {
        match dest {
            Reg16::AF => {
                self.data[0] = (value >> 8) as u8;
                self.data[5] = value as u8;
            }
            Reg16::BC => {
                self.data[1] = (value >> 8) as u8;
                self.data[2] = value as u8;
            }
            Reg16::DE => {
                self.data[3] = (value >> 8) as u8;
                self.data[4] = value as u8;
            }
            Reg16::HL => {
                self.data[6] = (value >> 8) as u8;
                self.data[7] = value as u8;
            }
            Reg16::SP => self.sp = value,
        }
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.data[5] = flags.into();
    }

    pub fn get_flags(&self) -> Flags {
        Flags::from(self.data[5])
    }
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        let mut result = 0;

        if self.z {
            result |= 0b1000_0000;
        }

        if self.n {
            result |= 0b0100_0000;
        }

        if self.h {
            result |= 0b0010_0000;
        }

        if self.c {
            result |= 0b0001_0000;
        }

        result
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            z: value & 0b1000_0000 != 0,
            n: value & 0b0100_0000 != 0,
            h: value & 0b0010_0000 != 0,
            c: value & 0b0001_0000 != 0,
        }
    }
}
