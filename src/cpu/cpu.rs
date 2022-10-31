use super::{
    instruction::{
        ArithmeticTarget, D8Operation, Instruction, JumpTest, LoadByteSource, LoadByteTarget,
        LoadType, SixteenBitArithmeticTarget, StackTarget,
    },
    memory_bus::MemoryBus,
    registers::{FlagsRegister, Registers},
};

pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
}

impl CPU {
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for: 0x{}", description);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::ADD16(target) => match target {
                SixteenBitArithmeticTarget::BC => {
                    let value = self.registers.get_bc();
                    let new_value = self.add_hl(value);
                    self.registers.set_hl(new_value);
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::DE => {
                    let value = self.registers.get_de();
                    let new_value = self.add_hl(value);
                    self.registers.set_hl(new_value);
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::HL => {
                    let value = self.registers.get_hl();
                    let new_value = self.add_hl(value);
                    self.registers.set_hl(new_value);
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::SP => {
                    let value = self.sp;
                    let new_value = self.add_hl(value);
                    self.registers.set_hl(new_value);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::ADC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.h;
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::SUB(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::SBC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::AND(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::OR(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::XOR(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.xor(value, true);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.h;
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::CP(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    self.sub(value);
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::INC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.inc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.inc(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.inc(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.inc(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.inc(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.inc(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.inc(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.inc(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::INC16(target) => match target {
                SixteenBitArithmeticTarget::BC => {
                    self.registers
                        .set_bc(self.registers.get_bc().wrapping_add(1));
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::DE => {
                    self.registers
                        .set_de(self.registers.get_de().wrapping_add(1));
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::HL => {
                    self.registers
                        .set_hl(self.registers.get_hl().wrapping_add(1));
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::SP => {
                    self.sp = self.sp.wrapping_add(1);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::DEC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.dec(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.dec(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.dec(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.dec(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.dec(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.dec(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.dec(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(1)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.dec(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::DEC16(target) => match target {
                SixteenBitArithmeticTarget::BC => {
                    self.registers
                        .set_bc(self.registers.get_bc().wrapping_sub(1));
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::DE => {
                    self.registers
                        .set_de(self.registers.get_de().wrapping_sub(1));
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::HL => {
                    self.registers
                        .set_hl(self.registers.get_hl().wrapping_sub(1));
                    self.pc.wrapping_add(1)
                }
                SixteenBitArithmeticTarget::SP => {
                    self.sp = self.sp.wrapping_sub(1);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::CCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                let value = self.registers.a;
                let new_value = self.rr(value, true, true);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                let value = self.registers.a;
                let new_value = self.rl(value, true, true);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                let value = self.registers.a;
                let new_value = self.rrca(value);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RRLA => {
                let value = self.registers.a;
                let new_value = self.rl(value, true, false);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                let value = self.registers.a;

                let mut new_value: u8 = 255;

                let mut i = 0b00000001;

                while i != 0 {
                    new_value = new_value ^ (value & i);
                    i = i << 1;
                }

                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;

                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(target, n) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    self.bit(value, n);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::RES(target, n) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.res(value, n);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.res(value, n);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.res(value, n);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.res(value, n);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.res(value, n);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.res(value, n);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.res(value, n);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.res(value, n);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::SET(target, n) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.set(value, n);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.set(value, n);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.set(value, n);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.set(value, n);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.set(value, n);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.set(value, n);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.set(value, n);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.set(value, n);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::SRL(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.srl(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.srl(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.srl(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.srl(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.srl(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.srl(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.srl(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.inc(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::RR(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    // RR is a seperate instruction from RRA, *will* check the
                    // value to set Zero flag.
                    let new_value = self.rr(value, false, true);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rr(value, false, true);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rr(value, false, true);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rr(value, false, true);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rr(value, false, true);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rr(value, false, true);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rr(value, false, true);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.rr(value, false, true);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::RL(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    // RR is a seperate instruction from RRA, *will* check the
                    // value to set Zero flag.
                    let new_value = self.rl(value, false, true);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rl(value, false, true);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rl(value, false, true);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rl(value, false, true);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rl(value, false, true);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rl(value, false, true);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rl(value, false, true);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.rl(value, false, true);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::RRC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    // RR is a seperate instruction from RRA, *will* check the
                    // value to set Zero flag.
                    let new_value = self.rrc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rrc(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rrc(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rrc(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rrc(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rrc(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rrc(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.rrc(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::RLC(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    // RR is a seperate instruction from RRA, *will* check the
                    // value to set Zero flag.
                    let new_value = self.rlc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.rlc(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.rlc(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.rlc(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.rlc(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.rlc(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.rlc(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.rlc(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::SRA(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sra(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sra(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sra(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sra(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sra(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sra(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sra(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.sra(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::SLA(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.sla(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.sla(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.sla(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.sla(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.sla(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.sla(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.sla(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.sla(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::SWAP(target) => match target {
                ArithmeticTarget::A => {
                    let value = self.registers.a;
                    let new_value = self.swap(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::B => {
                    let value = self.registers.b;
                    let new_value = self.swap(value);
                    self.registers.b = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.swap(value);
                    self.registers.c = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::D => {
                    let value = self.registers.d;
                    let new_value = self.swap(value);
                    self.registers.d = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::E => {
                    let value = self.registers.e;
                    let new_value = self.swap(value);
                    self.registers.e = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::H => {
                    let value = self.registers.h;
                    let new_value = self.swap(value);
                    self.registers.h = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::L => {
                    let value = self.registers.l;
                    let new_value = self.swap(value);
                    self.registers.l = new_value;
                    self.pc.wrapping_add(2)
                }
                ArithmeticTarget::HL => {
                    let value = self.bus.read_byte(self.registers.get_hl());
                    let new_value = self.swap(value);
                    self.bus.write_byte(self.registers.get_hl(), new_value);
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::ImmedieteArithmetic(operation) => match operation {
                D8Operation::ADD => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                D8Operation::ADC => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.adc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                D8Operation::AND => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.and(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                D8Operation::CP => {
                    let value = self.bus.read_byte(self.pc + 1);
                    self.sub(value);
                    self.pc.wrapping_add(2)
                }
                D8Operation::OR => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.or(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                D8Operation::SBC => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.sbc(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                D8Operation::SUB => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.sub(value);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
                D8Operation::XOR => {
                    let value = self.bus.read_byte(self.pc + 1);
                    let new_value = self.xor(value, false);
                    self.registers.a = new_value;
                    self.pc.wrapping_add(2)
                }
            },
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::HL => self.bus.read_byte(self.registers.get_hl()),
                        LoadByteSource::D8 => self.bus.read_byte(self.pc.wrapping_add(1)),
                        LoadByteSource::HLI => {
                            let value = self.bus.read_byte(self.registers.get_hl());

                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_add(1));

                            value
                        }
                        LoadByteSource::HLD => {
                            let value = self.bus.read_byte(self.registers.get_hl());

                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_sub(1));

                            value
                        }
                        LoadByteSource::BC => self.bus.read_byte(self.registers.get_bc()),
                        LoadByteSource::DE => self.bus.read_byte(self.registers.get_de()),
                        LoadByteSource::RefC => {
                            let value = self.registers.c as u16;
                            self.bus.read_byte(value.wrapping_add(0xFF00))
                        }
                        LoadByteSource::A16 => {
                            let ls_byte = self.bus.read_byte(self.pc + 1) as u16;
                            let ms_byte = self.bus.read_byte(self.pc + 2) as u16;
                            let addr = ms_byte << 8 | ls_byte;
                            self.bus.read_byte(addr)
                        }
                        LoadByteSource::A8 => {
                            let value = self.bus.read_byte(self.pc + 1) as u16;
                            self.bus.read_byte(value.wrapping_add(0xFF00))
                        }
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::B => self.registers.b = source_value,
                        LoadByteTarget::C => self.registers.c = source_value,
                        LoadByteTarget::D => self.registers.d = source_value,
                        LoadByteTarget::E => self.registers.e = source_value,
                        LoadByteTarget::H => self.registers.h = source_value,
                        LoadByteTarget::L => self.registers.l = source_value,
                        LoadByteTarget::HL => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value);
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_add(1));
                        }
                        LoadByteTarget::HLD => {
                            self.bus.write_byte(self.registers.get_hl(), source_value);
                            self.registers
                                .set_hl(self.registers.get_hl().wrapping_sub(1));
                        }
                        LoadByteTarget::BC => {
                            self.bus.write_byte(self.registers.get_bc(), source_value);
                        }
                        LoadByteTarget::DE => {
                            self.bus.write_byte(self.registers.get_de(), source_value);
                        }
                        LoadByteTarget::RefC => {
                            let c_addr = self.registers.c as u16;
                            self.bus
                                .write_byte(c_addr.wrapping_add(0xFF00), source_value);
                        }
                        LoadByteTarget::A16 => {
                            let ls_byte = self.bus.read_byte(self.pc + 1) as u16;
                            let ms_byte = self.bus.read_byte(self.pc + 2) as u16;
                            let addr = ms_byte << 8 | ls_byte;
                            self.bus.write_byte(addr, source_value);
                        }
                        LoadByteTarget::A8 => {
                            let c_addr = self.bus.read_byte(self.pc + 1) as u16;
                            self.bus
                                .write_byte(c_addr.wrapping_add(0xFF00), source_value);
                        }
                    };
                    match (target, source) {
                        (_, LoadByteSource::D8) => self.pc.wrapping_add(2),
                        (LoadByteTarget::A16, _) => self.pc.wrapping_add(3),
                        (_, LoadByteSource::A16) => self.pc.wrapping_add(3),
                        (LoadByteTarget::A8, _) => self.pc.wrapping_add(2),
                        (_, LoadByteSource::A8) => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
            },
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                };
                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
                let jump_condition: bool = match test {
                    _ => {
                        // TODO: support Jump conditions
                        todo!()
                    }
                };
                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition: bool = match test {
                    _ => {
                        // TODO: support Jump conditions
                        todo!()
                    }
                };
                self.return_(jump_condition)
            }
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            Instruction::ADDSP => {
                // Identify if n is negative or positive
                let (n, is_positive) = CPU::sign(self.bus.read_byte(self.pc.wrapping_add(1)));
                // grab the unsigned value from the 'signed' n
                // depending on the operation, add or subtract n from sp
                match is_positive {
                    true => {
                        let (new_value, did_overflow) = self.sp.overflowing_add(n as u16);

                        self.registers.f.half_carry = (((new_value as u16 >> 11) & 1) & ((self.sp >> 11) & 1)) == 1;
                        self.registers.f.carry = did_overflow;

                        self.sp = new_value;
                    }
                    false => {
                        let (new_value, did_overflow) = self.sp.overflowing_sub(n as u16);
                        self.sp = new_value
                    }
                }
                // set flags

                self.pc.wrapping_add(2)
            }
        }
    }

    fn add_half_carry (x: u16, y:u16, is_eight: bool) -> bool {
        match is_eight {
            true => (x & 0xF) + (y & 0xF) >= 0xF,
            false => (x & 0xFFF) + (y & 0xFFF) >= 0xFF
        }
    }

    fn sub_half_carry (x: u16, y: u16, is_eight: bool) -> bool {
        match is_eight {
            true => (x & 0xF) < (y & 0xF),
            false => (x & 0xFFF) < (y & 0xFFF)
        }
    }

    fn sign(value: u8) -> (u8, bool) {
        let is_positive = (value >> 7) & 1 == 0;
        let new_value = value & 0x7F;
        (new_value, is_positive)
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn read_next_word(&mut self) -> u16 {
        // TODO: Implement read_next_word
        todo!()
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half Cary is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF (16). If the result is larger than 0xF
        // then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.f.half_carry = CPU::add_half_carry(value as u16, self.registers.a as u16, true);
        self.registers.f.carry = did_overflow;
        new_value
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let target = self.registers.get_hl();
        let (new_value, did_overflow) = target.overflowing_add(value);

        // Does not affect Zero flag
        self.registers.f.subtract = false;
        self.registers.f.half_carry = CPU::add_half_carry(target, value, false);
        self.registers.f.carry = did_overflow;

        new_value
    }

    fn adc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = CPU::add_half_carry(self.registers.a as u16, value as u16, true);
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
        self.registers.f.half_carry = CPU::sub_half_carry(self.registers.a as u16, value as u16, true);
        self.registers.f.carry = did_overflow;

        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        // TODO: Fix half carry
        let new_value = self.registers.a.wrapping_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = CPU::sub_half_carry(self.registers.a as u16, value as u16, true);
        self.registers.f.carry = value > self.registers.a;

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
        self.registers.f.carry = false;

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
        let new_value = value.wrapping_add(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = CPU::add_half_carry(value as u16, 1, true);

        new_value
    }

    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = CPU::sub_half_carry(value as u16, 1, true);

        new_value
    }

    fn rr(&mut self, value: u8, is_a_register: bool, through_carry: bool) -> u8 {
        let carry_value: u8 = match self.registers.f.carry {
            true => 0x80,
            false => 0x0,
        };

        let new_value = match through_carry {
            true => (value >> 1) | carry_value,
            false => value >> 1,
        };

        self.registers.f.zero = match is_a_register {
            true => false,
            false => new_value == 0,
        };
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 1 == 1;

        new_value
    }

    fn rl(&mut self, value: u8, is_a_register: bool, through_carry: bool) -> u8 {
        let carry_value: u8 = match self.registers.f.carry {
            true => 1,
            false => 0,
        };
        let new_value = match through_carry {
            true => (value << 1) | carry_value,
            false => value << 1,
        };

        self.registers.f.zero = match is_a_register {
            true => false,
            false => new_value == 0,
        };
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value >> 7) & 1 == 1;

        new_value
    }

    fn bit(&mut self, value: u8, n: u8) {
        let compare_value: u8 = 1 << n;

        self.registers.f.zero = value & compare_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    fn res(&mut self, value: u8, n: u8) -> u8 {
        let compare_value: u8 = 1 << n;
        let new_value = value ^ compare_value;
        new_value
    }

    fn set(&mut self, value: u8, n: u8) -> u8 {
        let compare_value: u8 = 1 << n;
        let new_value = value | compare_value;
        new_value
    }

    fn srl(&mut self, value: u8) -> u8 {
        let carry_value = 1 & value;
        let new_value = value >> 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_value == 1;

        new_value
    }
    fn sra(&mut self, value: u8) -> u8 {
        let msb = 0x80 & value;
        let new_value = (value >> 1) | msb;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = 1 & value == 1;

        new_value
    }
    fn rlc(&mut self, value: u8) -> u8 {
        let carry_value = (0x80 & value) >> 7;
        let new_value = (value << 1) | carry_value;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_value == 1;

        new_value
    }

    fn rrc(&mut self, value: u8) -> u8 {
        let carry_value = 1 & value;
        let new_value = (value >> 1) | (carry_value << 7);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_value == 1;

        new_value
    }

    fn rrca(&mut self, value: u8) -> u8 {
        let carry_value = 1 & value;
        let new_value = (value >> 1) | (carry_value << 7);

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_value == 1;

        new_value
    }

    fn sla(&mut self, value: u8) -> u8 {
        let carry_value = (0x80 & value) >> 7;
        let new_value = value << 1;

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_value == 1;

        new_value
    }

    fn swap(&mut self, value: u8) -> u8 {
        let upper_nibble = value & 0xF0;
        let lower_nibble = value & 0x0F;

        let new_value = (upper_nibble >> 4) | (lower_nibble << 4);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        new_value
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
        }
    }
}

#[cfg(test)]
fn create_cpu(a: u8, b: u8, f: FlagsRegister) -> CPU {
    // let test_bus = [8u; 0xFFF].default()
    CPU {
        registers: Registers {
            a,
            b,
            c: 0,
            d: 0,
            e: 0,
            f,
            h: 0,
            l: 0,
        },
        pc: 0,
        sp: 0,
        bus: MemoryBus::new(),
        is_halted: false,
    }
}

#[test]
fn test_add() {
    let mut test_cpu = create_cpu(15, 1, FlagsRegister::from(0));

    test_cpu.execute(Instruction::ADD(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 16);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, true);
    assert_eq!(test_cpu.registers.f.carry, false);
}

#[test]
fn test_adc() {
    let mut test_cpu = create_cpu(255, 2, FlagsRegister::from(0));

    test_cpu.execute(Instruction::ADC(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 2);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, true);
    assert_eq!(test_cpu.registers.f.carry, true);
}

#[test]
fn test_sub() {
    let mut test_cpu = create_cpu(10, 5, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SUB(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 5);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, true);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_sbc() {
    let mut test_cpu = create_cpu(0x0, 0x10, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SBC(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 239);
    assert_eq!(u8::from(test_cpu.registers.f), 0x50)
}

#[test]
fn test_and() {
    let mut test_cpu = create_cpu(0b11110000, 0b00010000, FlagsRegister::from(0));

    test_cpu.execute(Instruction::AND(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 0b00010000);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, true);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_or() {
    let mut test_cpu = create_cpu(0b11110000, 0b00000001, FlagsRegister::from(0));

    test_cpu.execute(Instruction::OR(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 0b11110001);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_xor() {
    let mut test_cpu = create_cpu(0b11110000, 0b11110001, FlagsRegister::from(0));

    test_cpu.execute(Instruction::XOR(ArithmeticTarget::B));

    assert_eq!(test_cpu.registers.a, 1);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_inc() {
    let mut test_cpu = create_cpu(0, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::INC(ArithmeticTarget::A));

    assert_eq!(test_cpu.registers.a, 1);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_dec() {
    let mut test_cpu = create_cpu(1, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::DEC(ArithmeticTarget::A));

    assert_eq!(test_cpu.registers.a, 0);

    assert_eq!(test_cpu.registers.f.zero, true);
    assert_eq!(test_cpu.registers.f.subtract, true);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, false);
}

#[test]
fn test_ccf() {
    let mut test_cpu = create_cpu(0, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::CCF);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, true);

    test_cpu.execute(Instruction::CCF);

    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_scf() {
    let mut test_cpu = create_cpu(0, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SCF);

    assert_eq!(test_cpu.registers.f.zero, false);
    assert_eq!(test_cpu.registers.f.subtract, false);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, true)
}

#[test]
fn test_rra() {
    let mut test_cpu = create_cpu(
        0b00010110,
        0,
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
    );

    test_cpu.execute(Instruction::RRA);

    assert_eq!(test_cpu.registers.a, 0b10001011);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_rla() {
    let mut test_cpu = create_cpu(
        0b000010111,
        0,
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
    );

    test_cpu.execute(Instruction::RLA);

    assert_eq!(test_cpu.registers.a, 0b000101111);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_rrca() {
    let mut test_cpu = create_cpu(
        0b00010111,
        0,
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        },
    );

    test_cpu.execute(Instruction::RRCA);

    assert_eq!(
        test_cpu.registers.a, 0b10001011,
        "Testing Rotate of Register A"
    );
    assert_eq!(
        test_cpu.registers.f.carry, true,
        "Testing Bit 0 to Carry Flag"
    )
}

#[test]
fn test_rrla() {
    let mut test_cpu = create_cpu(
        0b000010111,
        0,
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
    );

    test_cpu.execute(Instruction::RRLA);

    assert_eq!(test_cpu.registers.a, 0b000101110);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_cpl() {
    let mut test_cpu = create_cpu(0b11110000, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::CPL);

    assert_eq!(test_cpu.registers.a, 0b00001111, "Testing register A");

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b01100000,
        "Testing Registers"
    );
}

#[test]
fn test_bit() {
    let mut test_cpu = create_cpu(0b00001111, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::BIT(ArithmeticTarget::A, 0));

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00100000,
        "Testing reading Bit 0 of A register"
    );

    test_cpu.execute(Instruction::BIT(ArithmeticTarget::A, 7));

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b10100000,
        "Testing reading Bit 7 of A register"
    )
}

#[test]
fn test_res() {
    let mut test_cpu = create_cpu(0b00001111, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::RES(ArithmeticTarget::A, 0));

    assert_eq!(
        test_cpu.registers.a, 0b00001110,
        "Testing resetting bit 0 of register A"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0,
        "Flags are not touched during operation"
    )
}

#[test]
fn test_set() {
    let mut test_cpu = create_cpu(0, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SET(ArithmeticTarget::A, 3));

    assert_eq!(
        test_cpu.registers.a, 8,
        "Testing setting bit 3 of register A"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0,
        "Flags are not touched during operation"
    )
}

#[test]
fn test_srl() {
    let mut test_cpu = create_cpu(0b10001111, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SRL(ArithmeticTarget::A));

    assert_eq!(
        test_cpu.registers.a, 0b01000111,
        "Testing logical shift right of Register A"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag contains bit from shift"
    )
}

#[test]
fn test_rr() {
    let mut test_cpu = create_cpu(0, 0b10000001, FlagsRegister::from(0));

    test_cpu.execute(Instruction::RR(ArithmeticTarget::B));

    assert_eq!(
        test_cpu.registers.b, 0b01000000,
        "Testing Rotate Right Through Carry of Register B"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag contains rotated bit"
    )
}

#[test]
fn test_rl() {
    let mut test_cpu = create_cpu(0, 0b10000001, FlagsRegister::from(0));

    test_cpu.execute(Instruction::RL(ArithmeticTarget::B));

    assert_eq!(
        test_cpu.registers.b, 0b00000010,
        "Testing Rotate Left Through Cary of Register B"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag contains rotated bit"
    )
}

#[test]
fn test_rrc() {
    let mut test_cpu = create_cpu(0, 0b10000001, FlagsRegister::from(0b00010000));

    test_cpu.execute(Instruction::RRC(ArithmeticTarget::B));

    assert_eq!(
        test_cpu.registers.b, 0b11000000,
        "Testing Rotate Right Through Carry of Register B"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag contains Old bit 0"
    )
}

#[test]
fn test_rlc() {
    let mut test_cpu = create_cpu(0, 0b10000001, FlagsRegister::from(0b00010000));

    test_cpu.execute(Instruction::RLC(ArithmeticTarget::B));

    assert_eq!(
        test_cpu.registers.b, 0b00000011,
        "Testing Rotate Right of Register B"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag contains old bit 7"
    )
}

#[test]
fn test_sra() {
    let mut test_cpu = create_cpu(0b10100001, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SRA(ArithmeticTarget::A));

    assert_eq!(
        test_cpu.registers.a, 0b11010000,
        "Testing Arithmetic Shift Right on Register A"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag set to bit 0"
    )
}

#[test]
fn test_sla() {
    let mut test_cpu = create_cpu(0b10100001, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SLA(ArithmeticTarget::A));

    assert_eq!(
        test_cpu.registers.a, 0b01000010,
        "Arithmetic Shift Left of Register A"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0b00010000,
        "Carry flag is ste to old bit 7"
    )
}

#[test]
fn test_swap() {
    let mut test_cpu = create_cpu(0b11110000, 0, FlagsRegister::from(0));

    test_cpu.execute(Instruction::SWAP(ArithmeticTarget::A));

    assert_eq!(
        test_cpu.registers.a, 0b00001111,
        "Swap the upper and lower nibbles of register A"
    );

    assert_eq!(
        u8::from(test_cpu.registers.f),
        0,
        "Zero flag is set if zero, all other flags reset"
    )
}

#[test]
fn test_sign() {
    let test_value = 0b11110000;
    assert_eq!(CPU::sign(test_value), (0b01110000, false))
}

#[test]
fn half_carry_add_test() {
    let x = 0b00001000;
    let y = 0b00001000;
    assert_eq!(CPU::add_half_carry(x as u16, y as u16, true), true)
}

#[test]
fn half_carry_sub_test() {
    let x:u8 = 0;
    let y:u8 = 0xF;
    assert_eq!(CPU::sub_half_carry(x as u16, y as u16, true), true)
    // let x_16 = 0
}
