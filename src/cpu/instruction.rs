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
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RRLA,
    CPL,
    BIT(ArithmeticTarget, u8),
    RES(ArithmeticTarget, u8),
    SET(ArithmeticTarget, u8),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    HALT,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // TODO: Implement non-prefixed Opcodes
            0x00 => Some(Instruction::NOP),
            0x01 => {
                /* TODO: Implement `LD BC, d16` */
                todo!()
            }
            0x02 => {
                /* TODO: Implement `LD (BC), A` */
                todo!()
            }
            0x03 => {
                /* TODO: Implement `INC BC` */
                todo!()
            }
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x07 => {
                // TODO: Implement `RLCA`
                todo!()
            }
            0x08 => {
                // TODO: Implement `LD (a16), SP`
                todo!()
            }
            0x09 => {
                // TODO: Implement `ADD HL, BC`
                todo!()
            }
            0x0A => {
                // TODO: Implement `LD A, (BC)`
                todo!()
            }
            0x0B => {
                // TODO: Implement `DEC BC`
                todo!()
            }
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0x0F => Some(Instruction::RRCA),

            0x10 => {
                // TODO: Implement `STOP d8`
                todo!()
            }
            0x11 => {
                // TODO Implement `LD DE, d16`
                todo!()
            }
            0x12 => {
                // TODO Implement `LD (DE), A`
                todo!()
            }
            0x13 => {
                // TODO Implement `INC DE`
                todo!()
            }
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x17 => Some(Instruction::RLA),
            0x18 => {
                // TODO Implement `JR r8`
                todo!()
            }
            0x19 => {
                // TODO Implement `ADD HL, DE`
                todo!()
            }
            0x1A => {
                // TODO: Implement `LD A, (DE)`
                todo!()
            }
            0x1B => {
                // TODO: Implement `DEC DE`
                todo!()
            }
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x1F => Some(Instruction::RRA),

            0x20 => {
                // TODO: Implement `JR NZ, r8`
                todo!()
            }
            0x21 => {
                // TODO: Implement `LD HL, d16`
                todo!()
            }
            0x22 => {
                // TODO: Implement `LD (HL+), A`
                todo!()
            }
            0x23 => {
                // TODO: Implement `INC HL`
                todo!()
            }
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x27 => {
                // TODO: Implement `DAA`
                todo!()
            }
            0x28 => {
                // TODO: Implement `JR Z, r8`
                todo!()
            }
            0x29 => {
                // TODO: Implement `ADD HL, HL`
                todo!()
            }
            0x2A => {
                // TODO: Implement `LD A, (HL+)`
                todo!()
            }
            0x2B => {
                // TODO: Implement `DEC HL`
                todo!()
            }
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL),
            _ => None,
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // TODO: Implement prefixed Opcodes
            _ => {
                todo!()
            }
        }
    }
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

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    // TODO: Add remaining types
    // Word
    // AFromIndirect
    // IndirectFromA
    // AFromByteAddress,
    // ByteAddressFromA
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum StackTarget {
    BC,
    // TODO: Add more targets
}
