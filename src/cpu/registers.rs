pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8
}

impl Registers {
    fn get_bc(&self) -> u16 {
        // pushes the 8 digits of b to the last 8 significant digits, and using
        // a bitwise OR operator, adds c as the first 8 digits
        (self.b as u16) << 8
        | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        // Uses a bitwise & operator to grab the last 8 significant digits of
        // value, and shifts them 8 'digits' backwards to make it an 8 bit
        // integer
        self.b = ((value & 0xFF00) >> 8) as u8;
        // Does the same thing, but with the first 8 significant digits
        self.c = (value & 0xFF) as u8;
    }

    // TODO: af getter/setter

    // TODO: de getter/setter

    // TODO: hl getter/setter
}
