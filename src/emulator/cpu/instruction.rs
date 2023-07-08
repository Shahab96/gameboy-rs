pub struct Immediate8;

pub enum Addr {
    BC,
    DE,
    HL,
    HLD,
    HLI,
    Direct,
    MaskedMemory,
    MaskedC,
}

pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}
