pub mod registers;
#[cfg(test)]
mod registers_test;

pub enum Instruction {
    ADD(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

pub struct CPU {
    registers: registers::Registers
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* TODO: support more targets */}
                }
            }
            _ => { /* TODO: support more targets */}
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // TODO: set flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and
        // register A together result in a value bigger than 0xF. If the result
        // is larger than 0xF than the addition caused  a carry from the lower
        // nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xf;
        new_value
    }
}
