use std::fs;

#[derive(Debug)]
pub struct CartReader {
    data: Vec<u8>
}

impl CartReader {
    pub fn new() -> Self {
        CartReader { data: vec![] }
    }

    pub fn read_bios(&mut self) {
        self.read(String::from("gb_bios.bin"));
    }

    // pub fn read_rom(&mut self) {}

    pub fn read(&mut self, filename: String) {
        let file = fs::read(format!("./roms/{filename:}"));
        match file {
            Ok(data) => {
                self.data = data;
            }
            Err(err) => panic!("{err:}")
        }
    }
}

#[test]
fn test_cart_reader() {
    let mut test_reader = CartReader::new();
    test_reader.read(String::from("gb_bios.bin"));
    assert_eq!(test_reader.data[0..3], vec![0x31, 0xFE, 0xFF])
}
