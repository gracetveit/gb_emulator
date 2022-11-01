use super::registers::{Registers, FlagsRegister};

const TEST_BITS: u16 = 0b1000110100101101;
const TEST_UPPER_NIBBLE: u8 = 0b10001101;
const TEST_LOWER_NIBBLE: u8 = 0b00101101;

#[test]
fn set_get_bc() {
    let mut test_registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister::from(0),
        h: 0,
        l: 0
    };

    test_registers.set_bc(TEST_BITS);
    assert_eq!(test_registers.b, TEST_UPPER_NIBBLE);
    assert_eq!(test_registers.c, TEST_LOWER_NIBBLE);

    assert_eq!(test_registers.get_bc(), TEST_BITS);
}

#[test]
fn set_get_af() {
    let mut test_registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister::from(0),
        h: 0,
        l: 0
    };

    test_registers.set_af(TEST_BITS);
    assert_eq!(test_registers.a, TEST_UPPER_NIBBLE);
    assert_eq!(u8::from(test_registers.f), 0b00100000);

    assert_eq!(test_registers.get_af(), 0b1000110100100000);
}

#[test]
fn set_get_de(){
    let mut test_registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister::from(0),
        h: 0,
        l: 0
    };

    test_registers.set_de(TEST_BITS);
    assert_eq!(test_registers.d, TEST_UPPER_NIBBLE);
    assert_eq!(test_registers.e, TEST_LOWER_NIBBLE);

    assert_eq!(test_registers.get_de(), TEST_BITS);
}

#[test]
fn set_get_hl(){
    let mut test_registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: FlagsRegister::from(0),
        h: 0,
        l: 0
    };

    test_registers.set_hl(TEST_BITS);
    assert_eq!(test_registers.h, TEST_UPPER_NIBBLE);
    assert_eq!(test_registers.l, TEST_LOWER_NIBBLE);

    assert_eq!(test_registers.get_hl(), TEST_BITS);
}
