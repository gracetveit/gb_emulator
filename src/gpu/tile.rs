#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Tile {
    lines: [u16; 8]
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            lines: [0; 8]
        }
    }

    pub fn reset(&mut self) {
        self.lines = [0; 8];
    }

    pub fn update(&mut self, value: u16, i: u16) {
        self.lines[i as usize] = value;
    }

    pub fn parse(&self, x: u8, y:u8) -> Color {
        Tile::parse_byte(self.lines[y as usize], x)
    }

    fn parse_byte(word: u16, n: u8) -> Color {
        let ms_byte = (word >> 8) as u8;
        let ls_byte = (word & 0x00FF) as u8;

        let ms_byte_bit = ((ms_byte >> 7 -n) & 0x01) == 1;
        let ls_byte_bit = ((ls_byte >> 7 -n) & 0x01) == 1;

        match (ms_byte_bit, ls_byte_bit) {
            (false, false) => Color::B,
            (true, false) => Color::DG,
            (false, true) => Color::LG,
            (true, true) => Color::W
        }
    }
}

pub enum Color {
    W,
    LG,
    DG,
    B
}

#[cfg(test)]
#[test]
fn test_parse() {
    let sprite = Tile {
        lines: [
            0x7C7C,
            0x00C6,
            0xC600,
            0x00FE,
            0xC6C6,
            0x00C6,
            0xC600,
            0x0000
        ]
    };

    assert!(matches!(sprite.parse(0, 0), Color::B));
    assert!(matches!(sprite.parse(1, 0), Color::W));
    assert!(matches!(sprite.parse(0, 1), Color::LG));
    assert!(matches!(sprite.parse(0, 2), Color::DG));
}
