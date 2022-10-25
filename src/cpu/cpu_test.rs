use super::{
    cpu::CPU,
    instruction::Instruction,
    registers::{FlagsRegister, Registers},
};

#[test]
fn test_add() {
    let test_registers = Registers {
        a: 0,
        b: 0,
        c: 1,
        d: 0,
        e: 0,
        f: FlagsRegister::from(0),
        h: 0,
        l: 0,
    };

    let mut test_cpu = CPU {
        registers: test_registers,
    };

    test_cpu.execute(Instruction::ADD(super::instruction::ArithmeticTarget::C));

    assert_eq!(test_cpu.registers.a, 1)
}

#[test]
fn test_rra() {
    let test_registers = Registers {
        a: 0b000010110,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
        h: 0,
        l: 0,
    };

    let mut test_cpu = CPU {
        registers: test_registers,
    };

    test_cpu.execute(Instruction::RRA);

    assert_eq!(test_cpu.registers.a, 0b10001011);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_rla() {
    let test_registers = Registers {
        a: 0b000010111,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
        h: 0,
        l: 0,
    };

    let mut test_cpu = CPU {
        registers: test_registers,
    };

    test_cpu.execute(Instruction::RLA);

    assert_eq!(test_cpu.registers.a, 0b000101111);
    assert_eq!(test_cpu.registers.f.carry, false)
}

#[test]
fn test_rrca() {
    let test_registers = Registers {
        a: 0b000010111,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
        h: 0,
        l: 0,
    };

    let mut test_cpu = CPU {
        registers: test_registers,
    };

    test_cpu.execute(Instruction::RRCA);

    assert_eq!(test_cpu.registers.a, 0b000001011);
    assert_eq!(test_cpu.registers.f.carry, true)
}

#[test]
fn test_rrla() {
    let test_registers = Registers {
        a: 0b000010111,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: true,
        },
        h: 0,
        l: 0,
    };

    let mut test_cpu = CPU {
        registers: test_registers,
    };

    test_cpu.execute(Instruction::RRLA);

    assert_eq!(test_cpu.registers.a, 0b000101110);
    assert_eq!(test_cpu.registers.f.carry, false)
}
