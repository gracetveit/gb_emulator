pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget)
}
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L
}
