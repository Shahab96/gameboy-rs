pub(super) enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
}

// We will never perform arithmetic operations on the F register, so we will not include it here
pub(super) enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
