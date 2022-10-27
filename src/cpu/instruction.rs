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
            0x01 => todo!(), // TODO: Implement `LD BC, d16`
            0x02 => todo!(), // TODO: Implement `LD (BC), A`
            0x03 => todo!(), // TODO: Implement `INC BC`
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x07 => todo!(), // TODO: Implement `RLCA`
            0x08 => todo!(), // TODO: Implement `LD (a16), SP`
            0x09 => todo!(), // TODO: Implement `ADD HL, BC`
            0x0A => todo!(), // TODO: Implement `LD A, (BC)`
            0x0B => todo!(), // TODO: Implement `DEC BC`
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0x0F => Some(Instruction::RRCA),

            0x10 => todo!(), // TODO: Implement `STOP d8`
            0x11 => todo!(), // TODO Implement `LD DE, d16`
            0x12 => todo!(), // TODO Implement `LD (DE), A`
            0x13 => todo!(), // TODO Implement `INC DE`
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x17 => Some(Instruction::RLA),
            0x18 => todo!(), // TODO Implement `JR r8`
            0x19 => todo!(), // TODO Implement `ADD HL, DE`
            0x1A => todo!(), // TODO: Implement `LD A, (DE)`
            0x1B => todo!(), // TODO: Implement `DEC DE`
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x1F => Some(Instruction::RRA),

            0x20 => todo!(), // TODO: Implement `JR NZ, r8`
            0x21 => todo!(), // TODO: Implement `LD HL, d16`
            0x22 => todo!(), // TODO: Implement `LD (HL+), A`
            0x23 => todo!(), // TODO: Implement `INC HL`
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x27 => todo!(), // TODO: Implement `DAA`
            0x28 => todo!(), // TODO: Implement `JR Z, r8`
            0x29 => todo!(), // TODO: Implement `ADD HL, HL`
            0x2A => todo!(), // TODO: Implement `LD A, (HL+)`
            0x2B => todo!(), // TODO: Implement `DEC HL`
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D8,
            ))),
            0x2F => Some(Instruction::CPL),

            0x30 => todo!(), // TODO: Implement `JR NC, r8`
            0x31 => todo!(), // TODO: Implement `LD SP, d16`
            0x32 => todo!(), // TODO: Implement `LD (HL-), A`
            0x33 => todo!(), // TODO: Implement `INC SP`
            0x34 => todo!(), // TODO: Implement `INC (HL)`
            0x35 => todo!(), // TODO: Implement `DEC (HL)`
            0x36 => todo!(), // TODO: Implement `LD (HL), d8`
            0x37 => Some(Instruction::SCF),
            0x38 => todo!(), // TODO: Implement `JR C, r8`
            0x39 => todo!(), // TODO: Implement `ADD HL, SP`
            0x3A => todo!(), // TODO: Implement `LD A, (HL-)`
            0x3B => todo!(), // TODO: Implement `DEC SP`
            0x3C => Some(Instruction::INC(ArithmeticTarget::A)),
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),
            0x3F => Some(Instruction::CCF),

            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::B,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => todo!(), // TODO: Implement `LD B, (HL)`
            0x47 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::A,
            ))),
            0x48 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::B,
            ))),
            0x49 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::C,
            ))),
            0x4A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D,
            ))),
            0x4B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::E,
            ))),
            0x4C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::H,
            ))),
            0x4D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::L,
            ))),
            0x4E => todo!(), //TODO: Implement `LD C, (HL)`
            0x4F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::A,
            ))),

            0x50 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::B,
            ))),
            0x51 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::C,
            ))),
            0x52 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D,
            ))),
            0x53 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::E,
            ))),
            0x54 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::H,
            ))),
            0x55 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::L,
            ))),
            0x56 => todo!(), // TODO: Implement `LD D, (HL)`
            0x57 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::A,
            ))),
            0x58 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::B,
            ))),
            0x59 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::C,
            ))),
            0x5A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D,
            ))),
            0x5B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::E,
            ))),
            0x5C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::H,
            ))),
            0x5D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::L,
            ))),
            0x5E => todo!(), // TODO: Implement `LD E, (HL)`
            0x5F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::A,
            ))),

            0x60 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::B,
            ))),
            0x61 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::C,
            ))),
            0x62 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D,
            ))),
            0x63 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::E,
            ))),
            0x64 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::H,
            ))),
            0x65 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::L,
            ))),
            0x66 => todo!(), // TODO: Implement `LD H, (HL)`
            0x67 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::A,
            ))),
            0x68 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::B,
            ))),
            0x69 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::C,
            ))),
            0x6A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D,
            ))),
            0x6B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::E,
            ))),
            0x6C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::H,
            ))),
            0x6D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::L,
            ))),
            0x6E => todo!(), // TODO: Implement `LD L, (HL)`
            0x6F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::A,
            ))),

            0x70 => todo!(), // TODO: Implement `LD (HL), B`
            0x71 => todo!(), // TODO: Implement `LD (HL), C`
            0x72 => todo!(), // TODO: Implement `LD (HL), D`
            0x73 => todo!(), // TODO: Implement `LD (HL), E`
            0x74 => todo!(), // TODO: Implement `LD (HL), H`
            0x75 => todo!(), // TODO: Implement `LD (HL), L`
            0x76 => Some(Instruction::HALT),
            0x77 => todo!(), // TODO: Implement `LD (HL), A`
            0x78 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::B
            ))),
            0x79 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::C
            ))),
            0x7A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D
            ))),
            0x7B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::E
            ))),
            0x7C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::H
            ))),
            0x7D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::L
            ))),
            0x7E => todo!(), // TODO: Implement `LD A, (HL)`
            0x7F => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::A
            ))),

            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => todo!(), // TODO: Implement `ADD A, (HL)`
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => todo!(), // TODO: Implement `ADC A, (HL)`
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),

            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => todo!(), // TODO: Implement `SUB (HL)`
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => todo!(), // TODO: Implement `SBC (HL)`
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),

            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => todo!(), // TODO: Implement `AND (HL)`
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => todo!(), // TODO: Implement `XOR (HL)`
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),

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
