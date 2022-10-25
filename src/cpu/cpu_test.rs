use super::{
    cpu::CPU,
    instruction::{ArithmeticTarget, Instruction},
    registers::{FlagsRegister, Registers},
};

fn create_cpu(a: u8, b: u8, f: FlagsRegister) -> CPU {
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

// #[test]
// fn test_sbc() {
//     let mut test_cpu = create_cpu(0u8, 5, FlagsRegister::from(0));

//     test_cpu.execute(Instruction::SBC(ArithmeticTarget::B));

//     assert_eq!(test_cpu.registers.a, 251);
// }

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
    let mut test_cpu = create_cpu(
        1,
        0,
        FlagsRegister::from(0)
    );

    test_cpu.execute(Instruction::DEC(ArithmeticTarget::A));

    assert_eq!(test_cpu.registers.a, 0);

    assert_eq!(test_cpu.registers.f.zero, true);
    assert_eq!(test_cpu.registers.f.subtract, true);
    assert_eq!(test_cpu.registers.f.half_carry, false);
    assert_eq!(test_cpu.registers.f.carry, false);
}

#[test]
fn test_rra() {
    let mut test_cpu = create_cpu(
        0b000010110,
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
        0b000010111,
        0,
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
    );

    test_cpu.execute(Instruction::RRCA);

    assert_eq!(test_cpu.registers.a, 0b000001011);
    assert_eq!(test_cpu.registers.f.carry, true)
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
