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
            Instruction::ADC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.h;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::SUB(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                } // _ => {}
            },
            Instruction::SBC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                } // _ => {}
            },
            Instruction::AND(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::OR(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                }
            },
            Instruction::XOR(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.xor(value, true);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.h;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                }
            },
            Instruction::CP(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    self.sub(value);
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    self.sub(value);
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    self.sub(value);
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    self.sub(value);
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    self.sub(value);
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    self.sub(value);
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    self.sub(value);
                }
            },
            Instruction::INC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.inc(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.inc(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.inc(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.inc(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.inc(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.inc(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.inc(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::DEC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.dec(value);
                    self.registers.a = new_value;
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.dec(value);
                    self.registers.b = new_value;
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.dec(value);
                    self.registers.c = new_value;
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.dec(value);
                    self.registers.d = new_value;
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.dec(value);
                    self.registers.e = new_value;
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.dec(value);
                    self.registers.h = new_value;
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.dec(value);
                    self.registers.l = new_value;
                }
            },
            Instruction::CCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
            }
            Instruction::SCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
            }
            Instruction::RRA => {
                let carry_value:u8 = match self.registers.f.carry {
                    true => 0x80,
                    false => 0x0
                };
                self.registers.f.carry = (self.registers.a >> 7) & 1 == 1;
                self.registers.a = (self.registers.a >> 1) | carry_value;
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half Cary is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF (16). If the result is larger than 0xF
        // then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;
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

    fn adc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;
        match self.registers.f.carry {
            true => new_value + 1,
            false => new_value,
        }
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = ((self.registers.a & 0xF) - (value & 0xF)) & 0x10 == 0x10;
        self.registers.f.carry = did_overflow;

        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = ((self.registers.a & 0xF) - (value & 0xF)) & 0x10 == 0x10;
        self.registers.f.carry = did_overflow;

        match self.registers.f.carry {
            true => new_value - 1,
            false => new_value,
        }
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = value & self.registers.a;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = true;

        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = value | self.registers.a;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        new_value
    }

    fn xor(&mut self, value: u8, is_a_register: bool) -> u8 {
        let new_value = value ^ self.registers.a;

        self.registers.f.zero = is_a_register || new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        new_value
    }

    fn inc(&mut self, value: u8) -> u8 {
        // Does not set the carry flag, so overflowing add does not need to
        // record info
        let (new_value, _) = value.overflowing_add(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn dec(&mut self, value: u8) -> u8 {
        let (new_value, _) = value.overflowing_sub(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = ((self.registers.a & 0xF) - (value & 0xF)) & 0x10 == 0x10;

        new_value
    }
}
