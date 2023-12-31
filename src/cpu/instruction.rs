use super::registers::{Reg16, Reg8};

#[derive(Debug)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug)]
pub enum Instruction {
    // 8 bit loads
    LDRR(Reg8, Reg8),
    LDRN(Reg8),
    LDRHL(Reg8),
    LDHLR(Reg8),
    LDHLN,
    LDA16(Reg16),
    LD16A(Reg16),
    LDANN,
    LDNNA,
    LDHAC,
    LDHCA,
    LDHAN,
    LDHNA,
    LDAHLDEC,
    LDHLDECA,
    LDAHLINC,
    LDHLINCA,

    // 16 bit loads
    LD16NN(Reg16),
    LD16SP,
    LDNNSP,
    LDSPHL,
    PUSH(Reg16),
    POP(Reg16),

    // 8-bit Arithmetic
    ADD(Reg8),
    ADDHL,
    ADDNN,
    ADC(Reg8),
    ADCHL,
    ADCNN,
    SUB(Reg8),
    SUBHL,
    SUBNN,
    SBC(Reg8),
    SBCHL,
    SBCNN,
    AND(Reg8),
    ANDHL,
    ANDNN,
    OR(Reg8),
    ORHL,
    ORNN,
    XOR(Reg8),
    XORHL,
    XORNN,
    CP(Reg8),
    CPHL,
    CPNN,
    INC(Reg8),
    INCHL,
    DEC(Reg8),
    DECHL,
    RRA,
    RLA,
    RRCA,
    RLCA,
    RR(Reg8),
    RL(Reg8),
    RLC(Reg8),
    RRC(Reg8),
    RLHL,
    RRHL,
    RLCHL,
    RRCHL,

    // Control Flow
    BIT(u8, Reg8),
    BITHL(u8),
    SET(u8, Reg8),
    SETHL(u8),
    RESET(u8, Reg8),
    RESETHL(u8),
    SWAP(Reg8),
    SWAPHL,
    SRL(Reg8),
    SRLHL,
    SRA(Reg8),
    SRAHL,
    SLA(Reg8),
    SLAHL,
    CALL,
    CALLCC(Condition),
    JP,
    JPCC(Condition),
    JPHL,
    JR,
    JRCC(Condition),
    RET,
    RETCC(Condition),
    RETI,
    RST(u8),

    // 16-bit Arithmetic
    ADDHLR16(Reg16),
    ADDHLRSP,
    ADDSPE,
    DEC16(Reg16),
    INC16(Reg16),
    DEC16SP,
    INC16SP,

    // Misc Instructions
    CPL,
    CCF,
    SCF,
    DAA,
    DI,
    EI,
    HALT,
    STOP,
    NOP,

    // CB Prefix
    PREFIXCB,
}

impl Instruction {
    // See Chapter 2 of https://gekkio.fi/files/gb-docs/gbctr.pdf
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            // 8-bit loads
            0x7f => Some(Instruction::LDRR(Reg8::A, Reg8::A)),
            0x78 => Some(Instruction::LDRR(Reg8::A, Reg8::B)),
            0x79 => Some(Instruction::LDRR(Reg8::A, Reg8::C)),
            0x7a => Some(Instruction::LDRR(Reg8::A, Reg8::D)),
            0x7b => Some(Instruction::LDRR(Reg8::A, Reg8::E)),
            0x7c => Some(Instruction::LDRR(Reg8::A, Reg8::H)),
            0x7d => Some(Instruction::LDRR(Reg8::A, Reg8::L)),
            0x47 => Some(Instruction::LDRR(Reg8::B, Reg8::A)),
            // 0x40 => self.ld_b_b(ctx),
            0x41 => Some(Instruction::LDRR(Reg8::B, Reg8::C)),
            0x42 => Some(Instruction::LDRR(Reg8::B, Reg8::D)),
            0x43 => Some(Instruction::LDRR(Reg8::B, Reg8::E)),
            0x44 => Some(Instruction::LDRR(Reg8::B, Reg8::H)),
            0x45 => Some(Instruction::LDRR(Reg8::B, Reg8::L)),
            0x4f => Some(Instruction::LDRR(Reg8::C, Reg8::A)),
            0x48 => Some(Instruction::LDRR(Reg8::C, Reg8::B)),
            0x49 => Some(Instruction::LDRR(Reg8::C, Reg8::C)),
            0x4a => Some(Instruction::LDRR(Reg8::C, Reg8::D)),
            0x4b => Some(Instruction::LDRR(Reg8::C, Reg8::E)),
            0x4c => Some(Instruction::LDRR(Reg8::C, Reg8::H)),
            0x4d => Some(Instruction::LDRR(Reg8::C, Reg8::L)),
            0x57 => Some(Instruction::LDRR(Reg8::D, Reg8::A)),
            0x50 => Some(Instruction::LDRR(Reg8::D, Reg8::B)),
            0x51 => Some(Instruction::LDRR(Reg8::D, Reg8::C)),
            0x52 => Some(Instruction::LDRR(Reg8::D, Reg8::D)),
            0x53 => Some(Instruction::LDRR(Reg8::D, Reg8::E)),
            0x54 => Some(Instruction::LDRR(Reg8::D, Reg8::H)),
            0x55 => Some(Instruction::LDRR(Reg8::D, Reg8::L)),
            0x5f => Some(Instruction::LDRR(Reg8::E, Reg8::A)),
            0x58 => Some(Instruction::LDRR(Reg8::E, Reg8::B)),
            0x59 => Some(Instruction::LDRR(Reg8::E, Reg8::C)),
            0x5a => Some(Instruction::LDRR(Reg8::E, Reg8::D)),
            0x5b => Some(Instruction::LDRR(Reg8::E, Reg8::E)),
            0x5c => Some(Instruction::LDRR(Reg8::E, Reg8::H)),
            0x5d => Some(Instruction::LDRR(Reg8::E, Reg8::L)),
            0x67 => Some(Instruction::LDRR(Reg8::H, Reg8::A)),
            0x60 => Some(Instruction::LDRR(Reg8::H, Reg8::B)),
            0x61 => Some(Instruction::LDRR(Reg8::H, Reg8::C)),
            0x62 => Some(Instruction::LDRR(Reg8::H, Reg8::D)),
            0x63 => Some(Instruction::LDRR(Reg8::H, Reg8::E)),
            0x64 => Some(Instruction::LDRR(Reg8::H, Reg8::H)),
            0x65 => Some(Instruction::LDRR(Reg8::H, Reg8::L)),
            0x6f => Some(Instruction::LDRR(Reg8::L, Reg8::A)),
            0x68 => Some(Instruction::LDRR(Reg8::L, Reg8::B)),
            0x69 => Some(Instruction::LDRR(Reg8::L, Reg8::C)),
            0x6a => Some(Instruction::LDRR(Reg8::L, Reg8::D)),
            0x6b => Some(Instruction::LDRR(Reg8::L, Reg8::E)),
            0x6c => Some(Instruction::LDRR(Reg8::L, Reg8::H)),
            0x6d => Some(Instruction::LDRR(Reg8::L, Reg8::L)),
            0x3e => Some(Instruction::LDRN(Reg8::A)),
            0x06 => Some(Instruction::LDRN(Reg8::B)),
            0x0e => Some(Instruction::LDRN(Reg8::C)),
            0x16 => Some(Instruction::LDRN(Reg8::D)),
            0x1e => Some(Instruction::LDRN(Reg8::E)),
            0x26 => Some(Instruction::LDRN(Reg8::H)),
            0x2e => Some(Instruction::LDRN(Reg8::L)),
            0x7e => Some(Instruction::LDA16(Reg16::HL)),
            0x0a => Some(Instruction::LDA16(Reg16::BC)),
            0x02 => Some(Instruction::LD16A(Reg16::BC)),
            0x1a => Some(Instruction::LDA16(Reg16::DE)),
            0x12 => Some(Instruction::LD16A(Reg16::DE)),
            0x46 => Some(Instruction::LDRHL(Reg8::B)),
            0x4e => Some(Instruction::LDRHL(Reg8::C)),
            0x56 => Some(Instruction::LDRHL(Reg8::D)),
            0x5e => Some(Instruction::LDRHL(Reg8::E)),
            0x66 => Some(Instruction::LDRHL(Reg8::H)),
            0x6e => Some(Instruction::LDRHL(Reg8::L)),
            0x36 => Some(Instruction::LDHLN),
            0x77 => Some(Instruction::LDHLR(Reg8::A)),
            0x70 => Some(Instruction::LDHLR(Reg8::B)),
            0x71 => Some(Instruction::LDHLR(Reg8::C)),
            0x72 => Some(Instruction::LDHLR(Reg8::D)),
            0x73 => Some(Instruction::LDHLR(Reg8::E)),
            0x74 => Some(Instruction::LDHLR(Reg8::H)),
            0x75 => Some(Instruction::LDHLR(Reg8::L)),
            0xfa => Some(Instruction::LDANN),
            0xea => Some(Instruction::LDNNA),
            0x3a => Some(Instruction::LDAHLDEC),
            0x32 => Some(Instruction::LDHLDECA),
            0x2a => Some(Instruction::LDAHLINC),
            0x22 => Some(Instruction::LDHLINCA),
            0xf2 => Some(Instruction::LDHAC),
            0xe2 => Some(Instruction::LDHCA),
            0xf0 => Some(Instruction::LDHNA),
            0xe0 => Some(Instruction::LDHAN),

            // 16-bit loads
            0x01 => Some(Instruction::LD16NN(Reg16::BC)),
            0x11 => Some(Instruction::LD16NN(Reg16::DE)),
            0x21 => Some(Instruction::LD16NN(Reg16::HL)),
            0x31 => Some(Instruction::LD16SP),
            0x08 => Some(Instruction::LDNNSP),
            0xf9 => Some(Instruction::LDSPHL),
            // 0xf8 => self.load16_hl_sp_e(ctx),
            0xc5 => Some(Instruction::PUSH(Reg16::BC)),
            0xd5 => Some(Instruction::PUSH(Reg16::DE)),
            0xe5 => Some(Instruction::PUSH(Reg16::HL)),
            0xf5 => Some(Instruction::PUSH(Reg16::AF)),
            0xc1 => Some(Instruction::POP(Reg16::BC)),
            0xd1 => Some(Instruction::POP(Reg16::DE)),
            0xe1 => Some(Instruction::POP(Reg16::HL)),
            0xf1 => Some(Instruction::POP(Reg16::AF)),

            // 8-bit Arithmetic
            0x87 => Some(Instruction::ADD(Reg8::A)),
            0x80 => Some(Instruction::ADD(Reg8::B)),
            0x81 => Some(Instruction::ADD(Reg8::C)),
            0x82 => Some(Instruction::ADD(Reg8::D)),
            0x83 => Some(Instruction::ADD(Reg8::E)),
            0x84 => Some(Instruction::ADD(Reg8::H)),
            0x85 => Some(Instruction::ADD(Reg8::L)),
            0x86 => Some(Instruction::ADDHL),
            0xc6 => Some(Instruction::ADDNN),
            0x8f => Some(Instruction::ADC(Reg8::A)),
            0x88 => Some(Instruction::ADC(Reg8::B)),
            0x89 => Some(Instruction::ADC(Reg8::C)),
            0x8a => Some(Instruction::ADC(Reg8::D)),
            0x8b => Some(Instruction::ADC(Reg8::E)),
            0x8c => Some(Instruction::ADC(Reg8::H)),
            0x8d => Some(Instruction::ADC(Reg8::L)),
            0x8e => Some(Instruction::ADCHL),
            0xce => Some(Instruction::ADCNN),
            0x97 => Some(Instruction::SUB(Reg8::A)),
            0x90 => Some(Instruction::SUB(Reg8::B)),
            0x91 => Some(Instruction::SUB(Reg8::C)),
            0x92 => Some(Instruction::SUB(Reg8::D)),
            0x93 => Some(Instruction::SUB(Reg8::E)),
            0x94 => Some(Instruction::SUB(Reg8::H)),
            0x95 => Some(Instruction::SUB(Reg8::L)),
            0x96 => Some(Instruction::SUBHL),
            0xd6 => Some(Instruction::SUBNN),
            0x9f => Some(Instruction::SBC(Reg8::A)),
            0x98 => Some(Instruction::SBC(Reg8::B)),
            0x99 => Some(Instruction::SBC(Reg8::C)),
            0x9a => Some(Instruction::SBC(Reg8::D)),
            0x9b => Some(Instruction::SBC(Reg8::E)),
            0x9c => Some(Instruction::SBC(Reg8::H)),
            0x9d => Some(Instruction::SBC(Reg8::L)),
            0x9e => Some(Instruction::SBCHL),
            0xde => Some(Instruction::SBCNN),
            0xbf => Some(Instruction::CP(Reg8::A)),
            0xb8 => Some(Instruction::CP(Reg8::B)),
            0xb9 => Some(Instruction::CP(Reg8::C)),
            0xba => Some(Instruction::CP(Reg8::D)),
            0xbb => Some(Instruction::CP(Reg8::E)),
            0xbc => Some(Instruction::CP(Reg8::H)),
            0xbd => Some(Instruction::CP(Reg8::L)),
            0xbe => Some(Instruction::CPHL),
            0xfe => Some(Instruction::CPNN),
            0xa7 => Some(Instruction::AND(Reg8::A)),
            0xa0 => Some(Instruction::AND(Reg8::B)),
            0xa1 => Some(Instruction::AND(Reg8::C)),
            0xa2 => Some(Instruction::AND(Reg8::D)),
            0xa3 => Some(Instruction::AND(Reg8::E)),
            0xa4 => Some(Instruction::AND(Reg8::H)),
            0xa5 => Some(Instruction::AND(Reg8::L)),
            0xa6 => Some(Instruction::ANDHL),
            0xe6 => Some(Instruction::ANDNN),
            0xb7 => Some(Instruction::OR(Reg8::A)),
            0xb0 => Some(Instruction::OR(Reg8::B)),
            0xb1 => Some(Instruction::OR(Reg8::C)),
            0xb2 => Some(Instruction::OR(Reg8::D)),
            0xb3 => Some(Instruction::OR(Reg8::E)),
            0xb4 => Some(Instruction::OR(Reg8::H)),
            0xb5 => Some(Instruction::OR(Reg8::L)),
            0xb6 => Some(Instruction::ORHL),
            0xf6 => Some(Instruction::ORNN),
            0xaf => Some(Instruction::XOR(Reg8::A)),
            0xa8 => Some(Instruction::XOR(Reg8::B)),
            0xa9 => Some(Instruction::XOR(Reg8::C)),
            0xaa => Some(Instruction::XOR(Reg8::D)),
            0xab => Some(Instruction::XOR(Reg8::E)),
            0xac => Some(Instruction::XOR(Reg8::H)),
            0xad => Some(Instruction::XOR(Reg8::L)),
            0xae => Some(Instruction::XORHL),
            0xee => Some(Instruction::XORNN),
            0x3c => Some(Instruction::INC(Reg8::A)),
            0x04 => Some(Instruction::INC(Reg8::B)),
            0x0c => Some(Instruction::INC(Reg8::C)),
            0x14 => Some(Instruction::INC(Reg8::D)),
            0x1c => Some(Instruction::INC(Reg8::E)),
            0x24 => Some(Instruction::INC(Reg8::H)),
            0x2c => Some(Instruction::INC(Reg8::L)),
            0x34 => Some(Instruction::INCHL),
            0x3d => Some(Instruction::DEC(Reg8::A)),
            0x05 => Some(Instruction::DEC(Reg8::B)),
            0x0d => Some(Instruction::DEC(Reg8::C)),
            0x15 => Some(Instruction::DEC(Reg8::D)),
            0x1d => Some(Instruction::DEC(Reg8::E)),
            0x25 => Some(Instruction::DEC(Reg8::H)),
            0x2d => Some(Instruction::DEC(Reg8::L)),
            0x35 => Some(Instruction::DECHL),
            0x07 => Some(Instruction::RLCA),
            0x17 => Some(Instruction::RLA),
            0x0f => Some(Instruction::RRCA),
            0x1f => Some(Instruction::RRA),

            // 16-bit Arithmetic
            0x09 => Some(Instruction::ADDHLR16(Reg16::BC)),
            0x19 => Some(Instruction::ADDHLR16(Reg16::DE)),
            0x29 => Some(Instruction::ADDHLR16(Reg16::HL)),
            0x39 => Some(Instruction::ADDHLRSP),
            0xe8 => Some(Instruction::ADDSPE),
            0x03 => Some(Instruction::INC16(Reg16::BC)),
            0x13 => Some(Instruction::INC16(Reg16::DE)),
            0x23 => Some(Instruction::INC16(Reg16::HL)),
            0x33 => Some(Instruction::INC16SP),
            0x0b => Some(Instruction::DEC16(Reg16::BC)),
            0x1b => Some(Instruction::DEC16(Reg16::DE)),
            0x2b => Some(Instruction::DEC16(Reg16::HL)),
            0x3b => Some(Instruction::DEC16SP),

            // Control
            0xc3 => Some(Instruction::JP),
            0xe9 => Some(Instruction::JPHL),
            0x18 => Some(Instruction::JR),
            0xcd => Some(Instruction::CALL),
            0xc9 => Some(Instruction::RET),
            0xd9 => Some(Instruction::RETI),
            0xc2 => Some(Instruction::JPCC(Condition::NZ)),
            0xca => Some(Instruction::JPCC(Condition::Z)),
            0xd2 => Some(Instruction::JPCC(Condition::NC)),
            0xda => Some(Instruction::JPCC(Condition::C)),
            0x20 => Some(Instruction::JRCC(Condition::NZ)),
            0x28 => Some(Instruction::JRCC(Condition::Z)),
            0x30 => Some(Instruction::JRCC(Condition::NC)),
            0x38 => Some(Instruction::JRCC(Condition::C)),
            0xc4 => Some(Instruction::CALLCC(Condition::NZ)),
            0xcc => Some(Instruction::CALLCC(Condition::Z)),
            0xd4 => Some(Instruction::CALLCC(Condition::NC)),
            0xdc => Some(Instruction::CALLCC(Condition::C)),
            0xc0 => Some(Instruction::RETCC(Condition::NZ)),
            0xc8 => Some(Instruction::RETCC(Condition::Z)),
            0xd0 => Some(Instruction::RETCC(Condition::NC)),
            0xd8 => Some(Instruction::RETCC(Condition::C)),
            0xc7 => Some(Instruction::RST(0x00)),
            0xcf => Some(Instruction::RST(0x08)),
            0xd7 => Some(Instruction::RST(0x10)),
            0xdf => Some(Instruction::RST(0x18)),
            0xe7 => Some(Instruction::RST(0x20)),
            0xef => Some(Instruction::RST(0x28)),
            0xf7 => Some(Instruction::RST(0x30)),
            0xff => Some(Instruction::RST(0x38)),

            // Misc
            0x76 => Some(Instruction::HALT),
            0x10 => Some(Instruction::STOP),
            0xf3 => Some(Instruction::DI),
            0xfb => Some(Instruction::EI),
            0x3f => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),
            0x00 => Some(Instruction::NOP),
            0x27 => Some(Instruction::DAA),
            0x2f => Some(Instruction::CPL),

            // Prefix CB
            0xcb => Some(Instruction::PREFIXCB),

            // Undefined opcodes
            0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd | _ => {
                panic!("Undefined opcode: {:#04x}", byte)
            }
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x07 => Some(Instruction::RLC(Reg8::A)),
            0x00 => Some(Instruction::RLC(Reg8::B)),
            0x01 => Some(Instruction::RLC(Reg8::C)),
            0x02 => Some(Instruction::RLC(Reg8::D)),
            0x03 => Some(Instruction::RLC(Reg8::E)),
            0x04 => Some(Instruction::RLC(Reg8::H)),
            0x05 => Some(Instruction::RLC(Reg8::L)),
            0x06 => Some(Instruction::RLCHL),
            0x17 => Some(Instruction::RL(Reg8::A)),
            0x10 => Some(Instruction::RL(Reg8::B)),
            0x11 => Some(Instruction::RL(Reg8::C)),
            0x12 => Some(Instruction::RL(Reg8::D)),
            0x13 => Some(Instruction::RL(Reg8::E)),
            0x14 => Some(Instruction::RL(Reg8::H)),
            0x15 => Some(Instruction::RL(Reg8::L)),
            0x16 => Some(Instruction::RLHL),
            0x0f => Some(Instruction::RRC(Reg8::A)),
            0x08 => Some(Instruction::RRC(Reg8::B)),
            0x09 => Some(Instruction::RRC(Reg8::C)),
            0x0a => Some(Instruction::RRC(Reg8::D)),
            0x0b => Some(Instruction::RRC(Reg8::E)),
            0x0c => Some(Instruction::RRC(Reg8::H)),
            0x0d => Some(Instruction::RRC(Reg8::L)),
            0x0e => Some(Instruction::RRCHL),
            0x1f => Some(Instruction::RR(Reg8::A)),
            0x18 => Some(Instruction::RR(Reg8::B)),
            0x19 => Some(Instruction::RR(Reg8::C)),
            0x1a => Some(Instruction::RR(Reg8::D)),
            0x1b => Some(Instruction::RR(Reg8::E)),
            0x1c => Some(Instruction::RR(Reg8::H)),
            0x1d => Some(Instruction::RR(Reg8::L)),
            0x1e => Some(Instruction::RRHL),
            0x27 => Some(Instruction::SLA(Reg8::A)),
            0x20 => Some(Instruction::SLA(Reg8::B)),
            0x21 => Some(Instruction::SLA(Reg8::C)),
            0x22 => Some(Instruction::SLA(Reg8::D)),
            0x23 => Some(Instruction::SLA(Reg8::E)),
            0x24 => Some(Instruction::SLA(Reg8::H)),
            0x25 => Some(Instruction::SLA(Reg8::L)),
            0x26 => Some(Instruction::SLAHL),
            0x2f => Some(Instruction::SRA(Reg8::A)),
            0x28 => Some(Instruction::SRA(Reg8::B)),
            0x29 => Some(Instruction::SRA(Reg8::C)),
            0x2a => Some(Instruction::SRA(Reg8::D)),
            0x2b => Some(Instruction::SRA(Reg8::E)),
            0x2c => Some(Instruction::SRA(Reg8::H)),
            0x2d => Some(Instruction::SRA(Reg8::L)),
            0x2e => Some(Instruction::SRAHL),
            0x3f => Some(Instruction::SRL(Reg8::A)),
            0x38 => Some(Instruction::SRL(Reg8::B)),
            0x39 => Some(Instruction::SRL(Reg8::C)),
            0x3a => Some(Instruction::SRL(Reg8::D)),
            0x3b => Some(Instruction::SRL(Reg8::E)),
            0x3c => Some(Instruction::SRL(Reg8::H)),
            0x3d => Some(Instruction::SRL(Reg8::L)),
            0x3e => Some(Instruction::SRLHL),
            0x37 => Some(Instruction::SWAP(Reg8::A)),
            0x30 => Some(Instruction::SWAP(Reg8::B)),
            0x31 => Some(Instruction::SWAP(Reg8::C)),
            0x32 => Some(Instruction::SWAP(Reg8::D)),
            0x33 => Some(Instruction::SWAP(Reg8::E)),
            0x34 => Some(Instruction::SWAP(Reg8::H)),
            0x35 => Some(Instruction::SWAP(Reg8::L)),
            0x36 => Some(Instruction::SWAPHL),
            0x47 => Some(Instruction::BIT(0, Reg8::A)),
            0x4f => Some(Instruction::BIT(1, Reg8::A)),
            0x57 => Some(Instruction::BIT(2, Reg8::A)),
            0x5f => Some(Instruction::BIT(3, Reg8::A)),
            0x67 => Some(Instruction::BIT(4, Reg8::A)),
            0x6f => Some(Instruction::BIT(5, Reg8::A)),
            0x77 => Some(Instruction::BIT(6, Reg8::A)),
            0x7f => Some(Instruction::BIT(7, Reg8::A)),
            0x40 => Some(Instruction::BIT(0, Reg8::B)),
            0x48 => Some(Instruction::BIT(1, Reg8::B)),
            0x50 => Some(Instruction::BIT(2, Reg8::B)),
            0x58 => Some(Instruction::BIT(3, Reg8::B)),
            0x60 => Some(Instruction::BIT(4, Reg8::B)),
            0x68 => Some(Instruction::BIT(5, Reg8::B)),
            0x70 => Some(Instruction::BIT(6, Reg8::B)),
            0x78 => Some(Instruction::BIT(7, Reg8::B)),
            0x41 => Some(Instruction::BIT(0, Reg8::C)),
            0x49 => Some(Instruction::BIT(1, Reg8::C)),
            0x51 => Some(Instruction::BIT(2, Reg8::C)),
            0x59 => Some(Instruction::BIT(3, Reg8::C)),
            0x61 => Some(Instruction::BIT(4, Reg8::C)),
            0x69 => Some(Instruction::BIT(5, Reg8::C)),
            0x71 => Some(Instruction::BIT(6, Reg8::C)),
            0x79 => Some(Instruction::BIT(7, Reg8::C)),
            0x42 => Some(Instruction::BIT(0, Reg8::D)),
            0x4a => Some(Instruction::BIT(1, Reg8::D)),
            0x52 => Some(Instruction::BIT(2, Reg8::D)),
            0x5a => Some(Instruction::BIT(3, Reg8::D)),
            0x62 => Some(Instruction::BIT(4, Reg8::D)),
            0x6a => Some(Instruction::BIT(5, Reg8::D)),
            0x72 => Some(Instruction::BIT(6, Reg8::D)),
            0x7a => Some(Instruction::BIT(7, Reg8::D)),
            0x43 => Some(Instruction::BIT(0, Reg8::E)),
            0x4b => Some(Instruction::BIT(1, Reg8::E)),
            0x53 => Some(Instruction::BIT(2, Reg8::E)),
            0x5b => Some(Instruction::BIT(3, Reg8::E)),
            0x63 => Some(Instruction::BIT(4, Reg8::E)),
            0x6b => Some(Instruction::BIT(5, Reg8::E)),
            0x73 => Some(Instruction::BIT(6, Reg8::E)),
            0x7b => Some(Instruction::BIT(7, Reg8::E)),
            0x44 => Some(Instruction::BIT(0, Reg8::H)),
            0x4c => Some(Instruction::BIT(1, Reg8::H)),
            0x54 => Some(Instruction::BIT(2, Reg8::H)),
            0x5c => Some(Instruction::BIT(3, Reg8::H)),
            0x64 => Some(Instruction::BIT(4, Reg8::H)),
            0x6c => Some(Instruction::BIT(5, Reg8::H)),
            0x74 => Some(Instruction::BIT(6, Reg8::H)),
            0x7c => Some(Instruction::BIT(7, Reg8::H)),
            0x45 => Some(Instruction::BIT(0, Reg8::L)),
            0x4d => Some(Instruction::BIT(1, Reg8::L)),
            0x55 => Some(Instruction::BIT(2, Reg8::L)),
            0x5d => Some(Instruction::BIT(3, Reg8::L)),
            0x65 => Some(Instruction::BIT(4, Reg8::L)),
            0x6d => Some(Instruction::BIT(5, Reg8::L)),
            0x75 => Some(Instruction::BIT(6, Reg8::L)),
            0x7d => Some(Instruction::BIT(7, Reg8::L)),
            0x46 => Some(Instruction::BITHL(0)),
            0x4e => Some(Instruction::BITHL(1)),
            0x56 => Some(Instruction::BITHL(2)),
            0x5e => Some(Instruction::BITHL(3)),
            0x66 => Some(Instruction::BITHL(4)),
            0x6e => Some(Instruction::BITHL(5)),
            0x76 => Some(Instruction::BITHL(6)),
            0x7e => Some(Instruction::BITHL(7)),
            0xc7 => Some(Instruction::SET(0, Reg8::A)),
            0xcf => Some(Instruction::SET(1, Reg8::A)),
            0xd7 => Some(Instruction::SET(2, Reg8::A)),
            0xdf => Some(Instruction::SET(3, Reg8::A)),
            0xe7 => Some(Instruction::SET(4, Reg8::A)),
            0xef => Some(Instruction::SET(5, Reg8::A)),
            0xf7 => Some(Instruction::SET(6, Reg8::A)),
            0xff => Some(Instruction::SET(7, Reg8::A)),
            0xc0 => Some(Instruction::SET(0, Reg8::B)),
            0xc8 => Some(Instruction::SET(1, Reg8::B)),
            0xd0 => Some(Instruction::SET(2, Reg8::B)),
            0xd8 => Some(Instruction::SET(3, Reg8::B)),
            0xe0 => Some(Instruction::SET(4, Reg8::B)),
            0xe8 => Some(Instruction::SET(5, Reg8::B)),
            0xf0 => Some(Instruction::SET(6, Reg8::B)),
            0xf8 => Some(Instruction::SET(7, Reg8::B)),
            0xc1 => Some(Instruction::SET(0, Reg8::C)),
            0xc9 => Some(Instruction::SET(1, Reg8::C)),
            0xd1 => Some(Instruction::SET(2, Reg8::C)),
            0xd9 => Some(Instruction::SET(3, Reg8::C)),
            0xe1 => Some(Instruction::SET(4, Reg8::C)),
            0xe9 => Some(Instruction::SET(5, Reg8::C)),
            0xf1 => Some(Instruction::SET(6, Reg8::C)),
            0xf9 => Some(Instruction::SET(7, Reg8::C)),
            0xc2 => Some(Instruction::SET(0, Reg8::D)),
            0xca => Some(Instruction::SET(1, Reg8::D)),
            0xd2 => Some(Instruction::SET(2, Reg8::D)),
            0xda => Some(Instruction::SET(3, Reg8::D)),
            0xe2 => Some(Instruction::SET(4, Reg8::D)),
            0xea => Some(Instruction::SET(5, Reg8::D)),
            0xf2 => Some(Instruction::SET(6, Reg8::D)),
            0xfa => Some(Instruction::SET(7, Reg8::D)),
            0xc3 => Some(Instruction::SET(0, Reg8::E)),
            0xcb => Some(Instruction::SET(1, Reg8::E)),
            0xd3 => Some(Instruction::SET(2, Reg8::E)),
            0xdb => Some(Instruction::SET(3, Reg8::E)),
            0xe3 => Some(Instruction::SET(4, Reg8::E)),
            0xeb => Some(Instruction::SET(5, Reg8::E)),
            0xf3 => Some(Instruction::SET(6, Reg8::E)),
            0xfb => Some(Instruction::SET(7, Reg8::E)),
            0xc4 => Some(Instruction::SET(0, Reg8::H)),
            0xcc => Some(Instruction::SET(1, Reg8::H)),
            0xd4 => Some(Instruction::SET(2, Reg8::H)),
            0xdc => Some(Instruction::SET(3, Reg8::H)),
            0xe4 => Some(Instruction::SET(4, Reg8::H)),
            0xec => Some(Instruction::SET(5, Reg8::H)),
            0xf4 => Some(Instruction::SET(6, Reg8::H)),
            0xfc => Some(Instruction::SET(7, Reg8::H)),
            0xc5 => Some(Instruction::SET(0, Reg8::L)),
            0xcd => Some(Instruction::SET(1, Reg8::L)),
            0xd5 => Some(Instruction::SET(2, Reg8::L)),
            0xdd => Some(Instruction::SET(3, Reg8::L)),
            0xe5 => Some(Instruction::SET(4, Reg8::L)),
            0xed => Some(Instruction::SET(5, Reg8::L)),
            0xf5 => Some(Instruction::SET(6, Reg8::L)),
            0xfd => Some(Instruction::SET(7, Reg8::L)),
            0xc6 => Some(Instruction::SETHL(0)),
            0xce => Some(Instruction::SETHL(1)),
            0xd6 => Some(Instruction::SETHL(2)),
            0xde => Some(Instruction::SETHL(3)),
            0xe6 => Some(Instruction::SETHL(4)),
            0xee => Some(Instruction::SETHL(5)),
            0xf6 => Some(Instruction::SETHL(6)),
            0xfe => Some(Instruction::SETHL(7)),
            0x87 => Some(Instruction::RESET(0, Reg8::A)),
            0x8f => Some(Instruction::RESET(1, Reg8::A)),
            0x97 => Some(Instruction::RESET(2, Reg8::A)),
            0x9f => Some(Instruction::RESET(3, Reg8::A)),
            0xa7 => Some(Instruction::RESET(4, Reg8::A)),
            0xaf => Some(Instruction::RESET(5, Reg8::A)),
            0xb7 => Some(Instruction::RESET(6, Reg8::A)),
            0xbf => Some(Instruction::RESET(7, Reg8::A)),
            0x80 => Some(Instruction::RESET(0, Reg8::B)),
            0x88 => Some(Instruction::RESET(1, Reg8::B)),
            0x90 => Some(Instruction::RESET(2, Reg8::B)),
            0x98 => Some(Instruction::RESET(3, Reg8::B)),
            0xa0 => Some(Instruction::RESET(4, Reg8::B)),
            0xa8 => Some(Instruction::RESET(5, Reg8::B)),
            0xb0 => Some(Instruction::RESET(6, Reg8::B)),
            0xb8 => Some(Instruction::RESET(7, Reg8::B)),
            0x81 => Some(Instruction::RESET(0, Reg8::C)),
            0x89 => Some(Instruction::RESET(1, Reg8::C)),
            0x91 => Some(Instruction::RESET(2, Reg8::C)),
            0x99 => Some(Instruction::RESET(3, Reg8::C)),
            0xa1 => Some(Instruction::RESET(4, Reg8::C)),
            0xa9 => Some(Instruction::RESET(5, Reg8::C)),
            0xb1 => Some(Instruction::RESET(6, Reg8::C)),
            0xb9 => Some(Instruction::RESET(7, Reg8::C)),
            0x82 => Some(Instruction::RESET(0, Reg8::D)),
            0x8a => Some(Instruction::RESET(1, Reg8::D)),
            0x92 => Some(Instruction::RESET(2, Reg8::D)),
            0x9a => Some(Instruction::RESET(3, Reg8::D)),
            0xa2 => Some(Instruction::RESET(4, Reg8::D)),
            0xaa => Some(Instruction::RESET(5, Reg8::D)),
            0xb2 => Some(Instruction::RESET(6, Reg8::D)),
            0xba => Some(Instruction::RESET(7, Reg8::D)),
            0x83 => Some(Instruction::RESET(0, Reg8::E)),
            0x8b => Some(Instruction::RESET(1, Reg8::E)),
            0x93 => Some(Instruction::RESET(2, Reg8::E)),
            0x9b => Some(Instruction::RESET(3, Reg8::E)),
            0xa3 => Some(Instruction::RESET(4, Reg8::E)),
            0xab => Some(Instruction::RESET(5, Reg8::E)),
            0xb3 => Some(Instruction::RESET(6, Reg8::E)),
            0xbb => Some(Instruction::RESET(7, Reg8::E)),
            0x84 => Some(Instruction::RESET(0, Reg8::H)),
            0x8c => Some(Instruction::RESET(1, Reg8::H)),
            0x94 => Some(Instruction::RESET(2, Reg8::H)),
            0x9c => Some(Instruction::RESET(3, Reg8::H)),
            0xa4 => Some(Instruction::RESET(4, Reg8::H)),
            0xac => Some(Instruction::RESET(5, Reg8::H)),
            0xb4 => Some(Instruction::RESET(6, Reg8::H)),
            0xbc => Some(Instruction::RESET(7, Reg8::H)),
            0x85 => Some(Instruction::RESET(0, Reg8::L)),
            0x8d => Some(Instruction::RESET(1, Reg8::L)),
            0x95 => Some(Instruction::RESET(2, Reg8::L)),
            0x9d => Some(Instruction::RESET(3, Reg8::L)),
            0xa5 => Some(Instruction::RESET(4, Reg8::L)),
            0xad => Some(Instruction::RESET(5, Reg8::L)),
            0xb5 => Some(Instruction::RESET(6, Reg8::L)),
            0xbd => Some(Instruction::RESET(7, Reg8::L)),
            0x86 => Some(Instruction::RESETHL(0)),
            0x8e => Some(Instruction::RESETHL(1)),
            0x96 => Some(Instruction::RESETHL(2)),
            0x9e => Some(Instruction::RESETHL(3)),
            0xa6 => Some(Instruction::RESETHL(4)),
            0xae => Some(Instruction::RESETHL(5)),
            0xb6 => Some(Instruction::RESETHL(6)),
            0xbe => Some(Instruction::RESETHL(7)),
        }
    }
}
