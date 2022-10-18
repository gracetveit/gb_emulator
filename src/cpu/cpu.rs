use super::{
    instruction::{ArithmeticTarget, Instruction},
    registers::Registers,
};

pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
            },
            // Instruction::ADDHL(target) => match target {
            //     ArithmeticTarget::A => {
            //         let value = self.registers.a;
            //         let new_value = self.addhl(value);
            //         self.registers.set_hl(new_value);
            //     }
            //     _ => { /* TODO: Add more targets */ }
            // },
            _ => { /* TODO: Support more Instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Cary is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF (16). If the result is larger than 0xF
        // then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // fn addhl(&mut self, value: u8) -> u16 {
    //     /*
    //     TODO

    //     my code says that the half-carry flag is the result of the 8-bit addition
    //     of H + (whatever high register) + carry from L + (whatever low register)

    //     so basically, do the low addition to get the carry, then treat the high
    //     addition as a regular ADC
    //     */
    //     let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16);
    //     self.registers.f.zero = new_value == 0;
    //     self.registers.f.subtract = false;
    //     self.registers.f.carry = did_overflow;
    //     self.registers.f.half_carry = (self.registers.get_hl() & 0xF) + (value as u16 & 0xF) > 0xF;
    //     new_value
    // }
}
