#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::from(0),
            h: 0,
            l: 0,
        }
    }

    pub fn get_bc(&self) -> u16 {
        // pushes the 8 digits of b to the last 8 significant digits, and using
        // a bitwise OR operator, adds c as the first 8 digits
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        // Uses a bitwise & operator to grab the last 8 significant digits of
        // value, and shifts them 8 'digits' backwards to make it an 8 bit
        // integer
        self.b = ((value & 0xFF00) >> 8) as u8;
        // Does the same thing, but with the first 8 significant digits
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (u8::from(self.f)) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

const ZERO_FLAG_BYTE_POSTION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSTIONG: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Copy, Clone, Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    /// For each flag in FlagsRegister, converts it to 1 or 0, and bit-shifts
    /// the flag to its proper place, combining all of the bits together using
    /// the bitwise OR operator
    ///
    /// ```
    /// 0001 0000 OR
    /// 0010 0000 OR...
    ///
    /// _________
    /// 1111 0000
    /// ```
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSTION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSTIONG
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    /// Takes a byte, and using the top 4 bits, converts each to their
    /// respective boolean flags, by bitshifting until the targetted bit is
    /// at the first position, and performing a bitwise AND operation with a
    /// single bit. This leaves all bits above the first as 0, and only 1
    /// returning as 1. We can than use this to compare to 0 and return a
    /// boolean true/false value.
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSTION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSTIONG) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
