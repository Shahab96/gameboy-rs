use super::registers::{Reg16, Reg8};

pub enum Instruction {
    ADD(Reg8),
    ADDHL(Reg16),
    ADC(Reg8),
    SUB(Reg8),
    SBC(Reg8),
    AND(Reg8),
    OR(Reg8),
    XOR(Reg8),
    CP(Reg8),
    INC(Reg8),
    DEC(Reg8),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    BIT(u8, Reg8),
    SET(u8, Reg8),
    SRL(Reg8),
    RR(Reg8),
    RL(Reg8),
    RLC(Reg8),
    RRC(Reg8),
    SRA(Reg8),
    SLA(Reg8),
    SWAP(Reg8),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            // TODO: Add map of instructions
            // See Chapter 2 of https://gekkio.fi/files/gb-docs/gbctr.pdf
            _ => None,
        }
    }
}
