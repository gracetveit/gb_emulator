// TODO Add all instructions

pub enum Instruction {
    ADD(ArithmeticTarget),
    // ADDHL
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    // CCF
    // SCF
    // RRA
    // RLA
    // RRCA
    // RRLA
    // CPL
    // BIT
    // RESET
    // SET
    // SRL
    // RR
    // RL
    // RRC
    // RLC
    // SRA
    // SLA
    // SWAP
}
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
