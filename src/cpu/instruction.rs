pub enum Instruction {
    ADD(ArithmeticTarget),
    // ADDHL(ArithmeticTarget)
    // ADC(ArithmeticTarget)
}
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L
}
