use std::fs;

pub struct MemoryBus {
    pub in_bios: bool,
    bios: [u8; 0xFF + 1],
    rom: [u8; 0x7FFF + 1],
    // Temp v_ram setup
    v_ram: [u8; 0x2000],
    e_ram: [u8; 0x2000],
    w_ram: [u8; 0x5E00],
    z_ram: [u8; 0x80],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        let mut new_bus = MemoryBus {
            in_bios: true,
            bios: MemoryBus::read_bios(),
            rom: [0; 0x8000],
            v_ram: [0; 0x2000],
            e_ram: [0; 0x2000],
            w_ram: [0; 0x5E00],
            z_ram: [0; 0x80],
        };
        // new_bus.read_rom();
        new_bus
    }

    fn read_bios() -> [u8; 0x100] {
        let mut bios = [0; 0x100];
        let file = fs::read("./roms/gb_bios.bin");
        match file {
            Ok(t) => {
                let mut i: usize = 0;
                for x in t.into_iter() {
                    bios[i] = x;
                    i += 1;
                }
            }
            Err(e) => {
                println!("Error: {e:?}")
            }
        }
        bios
    }

    // TODO: Re-implement `read_rom` when all write_byte functionality is complete
    // fn read_rom(&mut self){
    //     let mut rom = [0; 0x8000];
    //     let file = fs::read("./roms/zelda.gb");
    //     let initial_bios_toggle = self.in_bios;
    //     self.in_bios = false;
    //     match file {
    //         Ok(t) => {
    //             let mut i: u16 = 0;
    //             for x in t.into_iter() {
    //                 self.write_byte(i, x);
    //                 i += 1;
    //             }
    //         }
    //         Err(e) => {
    //             println!("Error: {e:?}")
    //         }
    //     }
    //     self.in_bios = initial_bios_toggle;
    // }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr & 0xF000 {
            // BIOS / ROM0
            0x0000 => {
                if self.in_bios {
                    self.bios[addr as usize]
                } else {
                    self.rom[addr as usize]
                }
            }
            // ROM0
            0x1000..=0x3000 => self.rom[addr as usize],
            // ROM1
            0x4000..=0x7000 => self.rom[addr as usize],
            // Graphics: VRAM
            0x8000..=0x9000 => self.v_ram[(addr & 0x1FFF) as usize],
            // External RAM
            0xA000..=0xB000 => self.e_ram[(addr & 0x1FFF) as usize],
            // Working Ram
            0xC000..=0xD000 => self.w_ram[(addr & 0x1FFF) as usize],
            // Working Ram shadow
            0xE000 => self.w_ram[(addr & 0x1FFF) as usize],
            // Working RAM shadow, I/o, Zero-page RAM
            0xF000 => {
                match addr & 0x0F00 {
                    // Working RAM shadow
                    0x000..=0xD00 => self.w_ram[(addr & 0x1FFF) as usize],
                    // Graphics: objets attribute memory
                    // OAM is 160 bytes, remaining bytes read as 0
                    0xE00 => {
                        if addr < 0xFEA0 {
                            todo!() // TODO: Write GPU fn: oam
                        } else {
                            0
                        }
                    }
                    // Zero-page
                    0xF00 => {
                        if addr >= 0xFF80 {
                            self.z_ram[(addr & 0x7F) as usize]
                        } else {
                            // I/O control handling
                            // Currently unhandled
                            0
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr & 0xF000 {
            // BIOS / ROM0
            0x0000 => {
                if self.in_bios {
                    self.bios[addr as usize] = byte;
                } else {
                    self.rom[addr as usize] = byte;
                }
            }
            // ROM0
            0x1000..=0x3000 => {
                self.rom[addr as usize] = byte;
            }
            // ROM1
            0x4000..=0x7000 => {
                self.rom[addr as usize] = byte;
            }
            // Graphics: VRAM
            0x8000..=0x9000 => {
                self.v_ram[(addr & 0x1FFF) as usize] = byte;
            }
            // External RAM
            0xA000..=0xB000 => {
                self.e_ram[(addr & 0x1FFF) as usize] = byte;
            }
            // Working Ram
            0xC000..=0xD000 => {
                self.w_ram[(addr & 0x1FFF) as usize] = byte;
            }
            // Working Ram shadow
            0xE000 => {
                self.w_ram[(addr & 0x1FFF) as usize] = byte;
            }
            // Working RAM shadow, I/o, Zero-page RAM
            0xF000 => {
                match addr & 0x0F00 {
                    // Working RAM shadow
                    0x000..=0xD00 => {
                        self.w_ram[(addr & 0x1FFF) as usize] = byte;
                    }
                    // Graphics: objets attribute memory
                    // OAM is 160 bytes, remaining bytes read as 0
                    0xE00 => {
                        if addr < 0xFEA0 {
                            todo!() // TODO: Write GPU fn: oam
                        } else {
                        }
                    }
                    // Zero-page
                    0xF00 => {
                        if addr >= 0xFF80 {
                            self.z_ram[(addr & 0x7F) as usize] = byte;
                        } else {
                            // I/O control handling
                            // Currently unhandled
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    pub fn read_word(&self, initial_addr: u16) -> u16 {
        let ls_byte = self.read_byte(initial_addr) as u16;
        let ms_byte = self.read_byte(initial_addr.wrapping_add(1)) as u16;
        (ms_byte << 8) | ls_byte
    }

    pub fn write_word(&mut self, initial_addr: u16, value: u16) {
        let ls_byte = (value & 0x00FF) as u8;
        let ms_byte = (value >> 8) as u8;

        self.write_byte(initial_addr, ls_byte);
        self.write_byte(initial_addr.wrapping_add(1), ms_byte);
    }
}

#[cfg(test)]
#[test]
fn write_read_byte_test() {
    let mut test_bus = MemoryBus::new();

    test_bus.in_bios = false;
    let test_addr = 0xA12C;
    let test_byte = 0x93;

    test_bus.write_byte(test_addr, test_byte);
    assert_eq!(test_bus.read_byte(test_addr), test_byte)
}

#[test]
fn write_read_word_test() {
    let mut test_bus = MemoryBus::new();

    test_bus.in_bios = false;
    let test_addr = 0xA12C;
    let test_word = 0x9383;

    test_bus.write_word(test_addr, test_word);
    assert_eq!(test_bus.read_word(test_addr), test_word)
}
#[test]
fn memory_bus_init_test() {
    MemoryBus::new();
}

#[test]
fn read_bios_test() {
    let test_bus = MemoryBus::new();

    assert_eq!(test_bus.read_byte(0), 0x31);
    assert_eq!(test_bus.read_byte(1), 0xFE);
    assert_eq!(test_bus.read_byte(2), 0xFF);
}
