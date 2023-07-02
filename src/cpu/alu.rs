use super::registers::Flags;

pub struct ALU {}

impl ALU {
    pub fn add(&self, a: u8, b: u8) -> (u8, Flags) {
        // We want to know if the addition overflows, so we will use the overflowing_add function
        let (result, did_overflow) = a.overflowing_add(b);

        let zero = result == 0;
        let half_carry = (a & 0xF) + (b & 0xF) > 0xF;
        let carry = did_overflow;
        let subtract = false;

        (
            result,
            Flags {
                zero,
                subtract,
                half_carry,
                carry,
            },
        )
    }

    pub fn sub(&self, a: u8, b: u8) -> (u8, Flags) {
        // We want to know if the subtraction underflows, so we will use the overflowing_sub function
        let (result, did_underflow) = a.overflowing_sub(b);

        let zero = result == 0;
        let half_carry = (a & 0xF) < (b & 0xF);
        let carry = did_underflow;
        let subtract = true;

        (
            result,
            Flags {
                zero,
                subtract,
                half_carry,
                carry,
            },
        )
    }

    pub fn add16(&self, a: u16, b: u16) -> (u16, Flags) {
        // We will use wrapping_add here because we don't care about the overflow
        let result = a.wrapping_add(b);

        let zero = result == 0;
        let carry = a > 0xFFFF - b;
        let half_carry = (a & 0xFFF) + (b & 0xFFF) > 0xFFF;

        (
            result,
            Flags {
                zero,
                carry,
                half_carry,
                subtract: false,
            },
        )
    }

    pub fn adc(&self, a: u8, b: u8, carry: u8) -> (u8, Flags) {
        // We will use wrapping_add here because we don't care about the overflow
        let result = a.wrapping_add(b).wrapping_add(carry as u8);

        let zero = result == 0;
        let half_carry = (a & 0xF) + (b & 0xF) + (carry as u8) > 0xF;
        let carry = (a as u16) + (b as u16) + (carry as u16) > 0xFF;

        (
            result,
            Flags {
                zero,
                subtract: false,
                half_carry,
                carry,
            },
        )
    }

    pub fn sbc(&self, a: u8, b: u8, carry: u8) -> (u8, Flags) {
        // We will use wrapping_sub here because we don't care about the overflow
        let result = a.wrapping_sub(b).wrapping_sub(carry as u8);

        let zero = result == 0;
        let half_carry = (a & 0xf).wrapping_sub(b & 0xf).wrapping_sub(carry as u8) & (0xf + 1) != 0;
        let carry = (a as u16) < (b as u16) + (carry as u16);

        (
            result,
            Flags {
                zero,
                subtract: true,
                half_carry,
                carry,
            },
        )
    }

    pub fn inc(&self, a: u8, carry: u8) -> (u8, Flags) {
        let result = a.wrapping_add(1);
        let zero = result == 0;
        let half_carry = a & 0xF == 0xF;

        (
            result,
            Flags {
                zero,
                subtract: false,
                half_carry,
                carry: carry == 1,
            },
        )
    }

    pub fn dec(&self, a: u8) -> (u8, Flags) {
        let result = a.wrapping_sub(1);
        let zero = result == 0;
        let half_carry = a & 0xF == 0x0;

        (
            result,
            Flags {
                zero,
                subtract: true,
                half_carry,
                carry: false,
            },
        )
    }

    pub fn rl(&self, a: u8, carry: u8) -> (u8, Flags) {
        let result = (a << 1) | carry;
        let zero = result == 0;
        let half_carry = false;
        let carry = a & 0x80 == 0x80;

        (
            result,
            Flags {
                zero,
                subtract: false,
                half_carry,
                carry,
            },
        )
    }

    pub fn rr(&self, a: u8, carry: u8) -> (u8, Flags) {
        let result = (a >> 1) | ((carry as u8) << 7);
        let zero = result == 0;
        let half_carry = false;
        let carry = a & 0x01 == 0x01;

        (
            result,
            Flags {
                zero,
                subtract: false,
                half_carry,
                carry,
            },
        )
    }

    pub fn rlc(&self, a: u8) -> (u8, Flags) {
        let carry = a & 0x80 != 0;
        let result = a.rotate_left(1);
        let zero = result == 0;

        (
            result,
            Flags {
                zero,
                subtract: false,
                half_carry: false,
                carry,
            },
        )
    }

    pub fn rrc(&self, a: u8) -> (u8, Flags) {
        let carry = a & 0x01 != 0;
        let result = a.rotate_right(1);
        let zero = result == 0;

        (
            result,
            Flags {
                zero,
                subtract: false,
                half_carry: false,
                carry,
            },
        )
    }
}
