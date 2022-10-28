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

            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => todo!(), // TODO: Implement `OR (HL)`
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => todo!(), // TODO: Impelemnt `CP (HL)`
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),

            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xC3 => Some(Instruction::JP(JumpTest::Always)),
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => todo!(), // TODO: Implement `ADD A, d8`
            0xC7 => todo!(), // TODO: Implement `RST 00H`
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),
            0xCB => None, // Prefix Byte, does not need to return an instruction
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xCD => Some(Instruction::CALL(JumpTest::Always)),
            0xCE => todo!(), // TODO: Implement `ADC A, d8`
            0xCF => todo!(), // TODO: Implement `RST 08H`

            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD1 => todo!(), // TODO: Implement `POP DE`
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry)),
            0xD3 => None, // Empty Byte
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xD5 => todo!(), // TODO: Implement `PUSH DE`
            0xD6 => todo!(), // TODO: Implement `SUB d8`
            0xD7 => todo!(), // TODO: Implement `RST 10H`
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xD9 => todo!(), // TODO: Implement `RETI`
            0xDA => Some(Instruction::JP(JumpTest::Carry)),
            0xDB => None, // Empty Byte
            0xDC => Some(Instruction::CALL(JumpTest::Carry)),
            0xDD => None, // Empty Byte
            0xDE => todo!(), // TODO: Implement `SBC A, d8`
            0xDF => todo!(), // TODO: Implement `RST 18H`

            0xE0 => todo!(), // TODO: Implement `LDH (a8), A`
            0xE1 => todo!(), // TODO: Implement `POP HL`
            0xE2 => todo!(), // TODO: Implement `LD (C), A`
            0xE3 => None, // Empty Byte
            0xE4 => None, // Empty Byte
            0xE5 => todo!(), // TODO: Implement `PUSH HL`
            0xE6 => todo!(), // TODO: Implement `AND d8`
            0xE7 => todo!(), // TODO: Implement `RST 20H`
            0xE8 => todo!(), // TODO: Implement `ADD SP, r8`,
            0xE9 => todo!(), // TODO: Implement `JP HL`
            0xEA => todo!(), // TODO: Implement `LD (a16), A`
            0xEB => None, // Empty Byte
            0xEC => None, // Empty Byte
            0xED => None, // Empty Byte
            0xEE => todo!(), // TODO: Implement `XOR d8`
            0xEF => todo!(), // TODO: Implement `RST 28H`

            0xF0 => todo!(), // TODO: Implement `LDH A, (a8)`
            0xF1 => todo!(), // TODO: Implement `POP AF`
            0xF2 => todo!(), // TODO: Implement `LD A, (C)`
            0xF3 => todo!(), // TODO: Implement `DI`
            0xF4 => None, // Empty Byte
            0xF5 => todo!(), // TODO: Implement `PUSH AF`
            0xF6 => todo!(), // TODO: Implement `OR d8`,
            0xF7 => todo!(), // TODO: Implement `RST 30H`
            0xF8 => todo!(), // TODO: Implement `LD HL, SP + r8`
            0xF9 => todo!(), // TODO: Implement `LD SP, HL`
            0xFA => todo!(), // TODO: Implement `LD A, (a16)`
            0xFB => todo!(), // TODO: Implement `EI`
            0xFC => None, // Empty Byte
            0xFD => None, // Empty Byte
            0xFE => todo!(), // TODO: Implement `CP d8`
            0xFF => todo!(), // TODO: Implement `RST 38H`
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // TODO: Implement prefixed Opcodes
            0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
            0x01 => Some(Instruction::RLC(ArithmeticTarget::C)),
            0x02 => Some(Instruction::RLC(ArithmeticTarget::D)),
            0x03 => Some(Instruction::RLC(ArithmeticTarget::E)),
            0x04 => Some(Instruction::RLC(ArithmeticTarget::H)),
            0x05 => Some(Instruction::RLC(ArithmeticTarget::L)),
            0x06 => todo!(), // TODO: Implement `RLC (HL)`
            0x07 => Some(Instruction::RLC(ArithmeticTarget::A)),
            0x08 => Some(Instruction::RRC(ArithmeticTarget::B)),
            0x09 => Some(Instruction::RRC(ArithmeticTarget::C)),
            0x0A => Some(Instruction::RRC(ArithmeticTarget::D)),
            0x0B => Some(Instruction::RRC(ArithmeticTarget::E)),
            0x0C => Some(Instruction::RRC(ArithmeticTarget::H)),
            0x0D => Some(Instruction::RRC(ArithmeticTarget::L)),
            0x0E => todo!(), // TODO: Implement `RRC (HL)`
            0x0F => Some(Instruction::RRC(ArithmeticTarget::A)),

            0x10 => Some(Instruction::RL(ArithmeticTarget::B)),
            0x11 => Some(Instruction::RL(ArithmeticTarget::C)),
            0x12 => Some(Instruction::RL(ArithmeticTarget::D)),
            0x13 => Some(Instruction::RL(ArithmeticTarget::E)),
            0x14 => Some(Instruction::RL(ArithmeticTarget::H)),
            0x15 => Some(Instruction::RL(ArithmeticTarget::L)),
            0x16 => todo!(), // TODO: Implement `RL (HL)`
            0x17 => Some(Instruction::RL(ArithmeticTarget::A)),
            0x18 => Some(Instruction::RR(ArithmeticTarget::B)),
            0x19 => Some(Instruction::RR(ArithmeticTarget::C)),
            0x1A => Some(Instruction::RR(ArithmeticTarget::D)),
            0x1B => Some(Instruction::RR(ArithmeticTarget::E)),
            0x1C => Some(Instruction::RR(ArithmeticTarget::H)),
            0x1D => Some(Instruction::RR(ArithmeticTarget::L)),
            0x1E => todo!(), // TODO: Implement `RR (HL)`
            0x1F => Some(Instruction::RR(ArithmeticTarget::A)),

            0x20 => Some(Instruction::SLA(ArithmeticTarget::B)),
            0x21 => Some(Instruction::SLA(ArithmeticTarget::C)),
            0x22 => Some(Instruction::SLA(ArithmeticTarget::D)),
            0x23 => Some(Instruction::SLA(ArithmeticTarget::E)),
            0x24 => Some(Instruction::SLA(ArithmeticTarget::H)),
            0x25 => Some(Instruction::SLA(ArithmeticTarget::L)),
            0x26 => todo!(), // TODO: Implement `SLA (HL)`
            0x27 => Some(Instruction::SLA(ArithmeticTarget::A)),
            0x28 => Some(Instruction::SRA(ArithmeticTarget::B)),
            0x29 => Some(Instruction::SRA(ArithmeticTarget::C)),
            0x2A => Some(Instruction::SRA(ArithmeticTarget::D)),
            0x2B => Some(Instruction::SRA(ArithmeticTarget::E)),
            0x2C => Some(Instruction::SRA(ArithmeticTarget::H)),
            0x2D => Some(Instruction::SRA(ArithmeticTarget::L)),
            0x2E => todo!(), // TODO: Implement `SRA (HL)`
            0x2F => Some(Instruction::SRA(ArithmeticTarget::A)),

            0x30 => Some(Instruction::SWAP(ArithmeticTarget::B)),
            0x31 => Some(Instruction::SWAP(ArithmeticTarget::C)),
            0x32 => Some(Instruction::SWAP(ArithmeticTarget::D)),
            0x33 => Some(Instruction::SWAP(ArithmeticTarget::E)),
            0x34 => Some(Instruction::SWAP(ArithmeticTarget::H)),
            0x35 => Some(Instruction::SWAP(ArithmeticTarget::L)),
            0x36 => todo!(), // TODO: Implement `SWAP (HL)`
            0x37 => Some(Instruction::SWAP(ArithmeticTarget::A)),
            0x38 => Some(Instruction::SRL(ArithmeticTarget::B)),
            0x39 => Some(Instruction::SRL(ArithmeticTarget::C)),
            0x3A => Some(Instruction::SRL(ArithmeticTarget::D)),
            0x3B => Some(Instruction::SRL(ArithmeticTarget::E)),
            0x3C => Some(Instruction::SRL(ArithmeticTarget::H)),
            0x3D => Some(Instruction::SRL(ArithmeticTarget::L)),
            0x3E => todo!(), // TODO: Implement `SRL (HL)`
            0x3F => Some(Instruction::SRL(ArithmeticTarget::A)),

            0x40 => Some(Instruction::BIT(ArithmeticTarget::B, 0)),
            0x41 => Some(Instruction::BIT(ArithmeticTarget::C, 0)),
            0x42 => Some(Instruction::BIT(ArithmeticTarget::D, 0)),
            0x43 => Some(Instruction::BIT(ArithmeticTarget::E, 0)),
            0x44 => Some(Instruction::BIT(ArithmeticTarget::H, 0)),
            0x45 => Some(Instruction::BIT(ArithmeticTarget::L, 0)),
            0x46 => todo!(), // TODO: Implement `BIT 0, (HL)`
            0x47 => Some(Instruction::BIT(ArithmeticTarget::A, 0)),
            0x48 => Some(Instruction::BIT(ArithmeticTarget::B, 1)),
            0x49 => Some(Instruction::BIT(ArithmeticTarget::C, 1)),
            0x4A => Some(Instruction::BIT(ArithmeticTarget::D, 1)),
            0x4B => Some(Instruction::BIT(ArithmeticTarget::E, 1)),
            0x4C => Some(Instruction::BIT(ArithmeticTarget::H, 1)),
            0x4D => Some(Instruction::BIT(ArithmeticTarget::L, 1)),
            0x4E => todo!(), // TODO: Implement `BIT 1, (HL)`
            0x4F => Some(Instruction::BIT(ArithmeticTarget::A, 1)),

            0x50 => Some(Instruction::BIT(ArithmeticTarget::B, 2)),
            0x51 => Some(Instruction::BIT(ArithmeticTarget::C, 2)),
            0x52 => Some(Instruction::BIT(ArithmeticTarget::D, 2)),
            0x53 => Some(Instruction::BIT(ArithmeticTarget::E, 2)),
            0x54 => Some(Instruction::BIT(ArithmeticTarget::H, 2)),
            0x55 => Some(Instruction::BIT(ArithmeticTarget::L, 2)),
            0x56 => todo!(), // TODO: Implement `BIT 2, (HL)`
            0x57 => Some(Instruction::BIT(ArithmeticTarget::A, 2)),
            0x58 => Some(Instruction::BIT(ArithmeticTarget::B, 3)),
            0x59 => Some(Instruction::BIT(ArithmeticTarget::C, 3)),
            0x5A => Some(Instruction::BIT(ArithmeticTarget::D, 3)),
            0x5B => Some(Instruction::BIT(ArithmeticTarget::E, 3)),
            0x5C => Some(Instruction::BIT(ArithmeticTarget::H, 3)),
            0x5D => Some(Instruction::BIT(ArithmeticTarget::L, 3)),
            0x5E => todo!(), // TODO: Implement `BIT 3, (HL)`
            0x5F => Some(Instruction::BIT(ArithmeticTarget::A, 3)),

            0x60 => Some(Instruction::BIT(ArithmeticTarget::B, 4)),
            0x61 => Some(Instruction::BIT(ArithmeticTarget::C, 4)),
            0x62 => Some(Instruction::BIT(ArithmeticTarget::D, 4)),
            0x63 => Some(Instruction::BIT(ArithmeticTarget::E, 4)),
            0x64 => Some(Instruction::BIT(ArithmeticTarget::H, 4)),
            0x65 => Some(Instruction::BIT(ArithmeticTarget::L, 4)),
            0x66 => todo!(), // TODO: Implement `BIT 4, (HL)`
            0x67 => Some(Instruction::BIT(ArithmeticTarget::A, 4)),
            0x68 => Some(Instruction::BIT(ArithmeticTarget::B, 5)),
            0x69 => Some(Instruction::BIT(ArithmeticTarget::C, 5)),
            0x6A => Some(Instruction::BIT(ArithmeticTarget::D, 5)),
            0x6B => Some(Instruction::BIT(ArithmeticTarget::E, 5)),
            0x6C => Some(Instruction::BIT(ArithmeticTarget::H, 5)),
            0x6D => Some(Instruction::BIT(ArithmeticTarget::L, 5)),
            0x6E => todo!(), // TODO: Implement `BIT 5, (HL)`
            0x6F => Some(Instruction::BIT(ArithmeticTarget::A, 5)),

            0x70 => Some(Instruction::BIT(ArithmeticTarget::B, 6)),
            0x71 => Some(Instruction::BIT(ArithmeticTarget::C, 6)),
            0x72 => Some(Instruction::BIT(ArithmeticTarget::D, 6)),
            0x73 => Some(Instruction::BIT(ArithmeticTarget::E, 6)),
            0x74 => Some(Instruction::BIT(ArithmeticTarget::H, 6)),
            0x75 => Some(Instruction::BIT(ArithmeticTarget::L, 6)),
            0x76 => todo!(), // TODO: Implement `BIT 6, (HL)`
            0x77 => Some(Instruction::BIT(ArithmeticTarget::A, 6)),
            0x78 => Some(Instruction::BIT(ArithmeticTarget::B, 7)),
            0x79 => Some(Instruction::BIT(ArithmeticTarget::C, 7)),
            0x7A => Some(Instruction::BIT(ArithmeticTarget::D, 7)),
            0x7B => Some(Instruction::BIT(ArithmeticTarget::E, 7)),
            0x7C => Some(Instruction::BIT(ArithmeticTarget::H, 7)),
            0x7D => Some(Instruction::BIT(ArithmeticTarget::L, 7)),
            0x7E => todo!(), // TODO: Implement `BIT 7, (HL)`
            0x7F => Some(Instruction::BIT(ArithmeticTarget::A, 7)),

            0x80 => Some(Instruction::RES(ArithmeticTarget::B, 0)),
            0x81 => Some(Instruction::RES(ArithmeticTarget::C, 0)),
            0x82 => Some(Instruction::RES(ArithmeticTarget::D, 0)),
            0x83 => Some(Instruction::RES(ArithmeticTarget::E, 0)),
            0x84 => Some(Instruction::RES(ArithmeticTarget::H, 0)),
            0x85 => Some(Instruction::RES(ArithmeticTarget::L, 0)),
            0x86 => todo!(), // TODO: Implement `RES 0, (HL)`
            0x87 => Some(Instruction::RES(ArithmeticTarget::A, 0)),
            0x88 => Some(Instruction::RES(ArithmeticTarget::B, 1)),
            0x89 => Some(Instruction::RES(ArithmeticTarget::C, 1)),
            0x8A => Some(Instruction::RES(ArithmeticTarget::D, 1)),
            0x8B => Some(Instruction::RES(ArithmeticTarget::E, 1)),
            0x8C => Some(Instruction::RES(ArithmeticTarget::H, 1)),
            0x8D => Some(Instruction::RES(ArithmeticTarget::L, 1)),
            0x8E => todo!(), // TODO: Implement `RES 1, (HL)`
            0x8F => Some(Instruction::RES(ArithmeticTarget::A, 1)),

            0x90 => Some(Instruction::RES(ArithmeticTarget::B, 2)),
            0x91 => Some(Instruction::RES(ArithmeticTarget::C, 2)),
            0x92 => Some(Instruction::RES(ArithmeticTarget::D, 2)),
            0x93 => Some(Instruction::RES(ArithmeticTarget::E, 2)),
            0x94 => Some(Instruction::RES(ArithmeticTarget::H, 2)),
            0x95 => Some(Instruction::RES(ArithmeticTarget::L, 2)),
            0x96 => todo!(), // TODO: Implement `RES 2, (HL)`
            0x97 => Some(Instruction::RES(ArithmeticTarget::A, 2)),
            0x98 => Some(Instruction::RES(ArithmeticTarget::B, 3)),
            0x99 => Some(Instruction::RES(ArithmeticTarget::C, 3)),
            0x9A => Some(Instruction::RES(ArithmeticTarget::D, 3)),
            0x9B => Some(Instruction::RES(ArithmeticTarget::E, 3)),
            0x9C => Some(Instruction::RES(ArithmeticTarget::H, 3)),
            0x9D => Some(Instruction::RES(ArithmeticTarget::L, 3)),
            0x9E => todo!(), // TODO: Implement `RES 3, (HL)`
            0x9F => Some(Instruction::RES(ArithmeticTarget::A, 3)),

            0xA0 => Some(Instruction::RES(ArithmeticTarget::B, 4)),
            0xA1 => Some(Instruction::RES(ArithmeticTarget::C, 4)),
            0xA2 => Some(Instruction::RES(ArithmeticTarget::D, 4)),
            0xA3 => Some(Instruction::RES(ArithmeticTarget::E, 4)),
            0xA4 => Some(Instruction::RES(ArithmeticTarget::H, 4)),
            0xA5 => Some(Instruction::RES(ArithmeticTarget::L, 4)),
            0xA6 => todo!(), // TODO: Implement `RES 4, (HL)`
            0xA7 => Some(Instruction::RES(ArithmeticTarget::A, 4)),
            0xA8 => Some(Instruction::RES(ArithmeticTarget::B, 5)),
            0xA9 => Some(Instruction::RES(ArithmeticTarget::C, 5)),
            0xAA => Some(Instruction::RES(ArithmeticTarget::D, 5)),
            0xAB => Some(Instruction::RES(ArithmeticTarget::E, 5)),
            0xAC => Some(Instruction::RES(ArithmeticTarget::H, 5)),
            0xAD => Some(Instruction::RES(ArithmeticTarget::L, 5)),
            0xAE => todo!(), // TODO: Implement `RES 5, (HL)`
            0xAF => Some(Instruction::RES(ArithmeticTarget::A, 5)),

            0xB0 => Some(Instruction::RES(ArithmeticTarget::B, 6)),
            0xB1 => Some(Instruction::RES(ArithmeticTarget::C, 6)),
            0xB2 => Some(Instruction::RES(ArithmeticTarget::D, 6)),
            0xB3 => Some(Instruction::RES(ArithmeticTarget::E, 6)),
            0xB4 => Some(Instruction::RES(ArithmeticTarget::H, 6)),
            0xB5 => Some(Instruction::RES(ArithmeticTarget::L, 6)),
            0xB6 => todo!(), // TODO: Implement `RES 6, (HL)`
            0xB7 => Some(Instruction::RES(ArithmeticTarget::A, 6)),
            0xB8 => Some(Instruction::RES(ArithmeticTarget::B, 7)),
            0xB9 => Some(Instruction::RES(ArithmeticTarget::C, 7)),
            0xBA => Some(Instruction::RES(ArithmeticTarget::D, 7)),
            0xBB => Some(Instruction::RES(ArithmeticTarget::E, 7)),
            0xBC => Some(Instruction::RES(ArithmeticTarget::H, 7)),
            0xBD => Some(Instruction::RES(ArithmeticTarget::L, 7)),
            0xBE => todo!(), // TODO: Implement `RES 7, (HL)`
            0xBF => Some(Instruction::RES(ArithmeticTarget::A, 7)),

            0xC0 => Some(Instruction::SET(ArithmeticTarget::B, 0)),
            0xC1 => Some(Instruction::SET(ArithmeticTarget::C, 0)),
            0xC2 => Some(Instruction::SET(ArithmeticTarget::D, 0)),
            0xC3 => Some(Instruction::SET(ArithmeticTarget::E, 0)),
            0xC4 => Some(Instruction::SET(ArithmeticTarget::H, 0)),
            0xC5 => Some(Instruction::SET(ArithmeticTarget::L, 0)),
            0xC6 => todo!(), // TODO: Implement `SET 0, (HL)`
            0xC7 => Some(Instruction::SET(ArithmeticTarget::A, 0)),
            0xC8 => Some(Instruction::SET(ArithmeticTarget::B, 1)),
            0xC9 => Some(Instruction::SET(ArithmeticTarget::C, 1)),
            0xCA => Some(Instruction::SET(ArithmeticTarget::D, 1)),
            0xCB => Some(Instruction::SET(ArithmeticTarget::E, 1)),
            0xCC => Some(Instruction::SET(ArithmeticTarget::H, 1)),
            0xCD => Some(Instruction::SET(ArithmeticTarget::L, 1)),
            0xCE => todo!(), // TODO: Implement `SET 1, (HL)`
            0xCF => Some(Instruction::SET(ArithmeticTarget::A, 1)),

            0xD0 => Some(Instruction::SET(ArithmeticTarget::B, 2)),
            0xD1 => Some(Instruction::SET(ArithmeticTarget::C, 2)),
            0xD2 => Some(Instruction::SET(ArithmeticTarget::D, 2)),
            0xD3 => Some(Instruction::SET(ArithmeticTarget::E, 2)),
            0xD4 => Some(Instruction::SET(ArithmeticTarget::H, 2)),
            0xD5 => Some(Instruction::SET(ArithmeticTarget::L, 2)),
            0xD6 => todo!(), // TODO: Implement `SET 2, (HL)`
            0xD7 => Some(Instruction::SET(ArithmeticTarget::A, 2)),
            0xD8 => Some(Instruction::SET(ArithmeticTarget::B, 3)),
            0xD9 => Some(Instruction::SET(ArithmeticTarget::C, 3)),
            0xDA => Some(Instruction::SET(ArithmeticTarget::D, 3)),
            0xDB => Some(Instruction::SET(ArithmeticTarget::E, 3)),
            0xDC => Some(Instruction::SET(ArithmeticTarget::H, 3)),
            0xDD => Some(Instruction::SET(ArithmeticTarget::L, 3)),
            0xDE => todo!(), // TODO: Implement `SET 3, (HL)`
            0xDF => Some(Instruction::SET(ArithmeticTarget::A, 3)),

            0xE0 => Some(Instruction::SET(ArithmeticTarget::B, 4)),
            0xE1 => Some(Instruction::SET(ArithmeticTarget::C, 4)),
            0xE2 => Some(Instruction::SET(ArithmeticTarget::D, 4)),
            0xE3 => Some(Instruction::SET(ArithmeticTarget::E, 4)),
            0xE4 => Some(Instruction::SET(ArithmeticTarget::H, 4)),
            0xE5 => Some(Instruction::SET(ArithmeticTarget::L, 4)),
            0xE6 => todo!(), // TODO: Implement `SET 4, (HL)`
            0xE7 => Some(Instruction::SET(ArithmeticTarget::A, 4)),
            0xE8 => Some(Instruction::SET(ArithmeticTarget::B, 5)),
            0xE9 => Some(Instruction::SET(ArithmeticTarget::C, 5)),
            0xEA => Some(Instruction::SET(ArithmeticTarget::D, 5)),
            0xEB => Some(Instruction::SET(ArithmeticTarget::E, 5)),
            0xEC => Some(Instruction::SET(ArithmeticTarget::H, 5)),
            0xED => Some(Instruction::SET(ArithmeticTarget::L, 5)),
            0xEE => todo!(), // TODO: Implement `SET 5, (HL)`
            0xEF => Some(Instruction::SET(ArithmeticTarget::A, 5)),

            _ => todo!()
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
