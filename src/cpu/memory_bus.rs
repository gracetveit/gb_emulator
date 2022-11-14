use crate::gpu::gpu::GPU;
use std::fs;

/*

Memory map

0x0000 - 0x3FFF: Rom Bank #0
0x4000 - 0x7FFF: swithcable ROM bank
0x8000 - 0x9FFF: Video RAM
0xA000 - 0xBFFF: Switchable RAM bank
0xC000 - 0xDFFF: Internal RAM
0xE000 - 0xFDFF: Echo of Internal RAM
0xFE00 - 0xFE9F: Sprite Attrib Mem (OAM)
0xFEA0 - 0xFEFF: Empty but unusable for I/O (Ignore)
0xFF00 - 0xFF7F: I/O Ports
0xFF80 - 0xFFFE: Internal / High RAM
0xFFFF: Interrupt Enable Register

*/

#[derive(Debug)]
pub struct MemoryBus {
    pub in_bios: bool,
    bios: [u8; 0x0100],
    rom: [u8; 0x8000],
    // Temp v_ram setup
    e_ram: [u8; 0x2000],
    i_ram: [u8; 0x2000],
    s_ram: [u8; 0x1E00],
    oam: [u8; 0x00A0],
    io: [u8; 0x0080],
    high_ram: [u8; 0x007F],
    interrupt_register: u8,
    pub gpu: GPU,
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        let new_bus = MemoryBus {
            in_bios: true,
            bios: MemoryBus::read_bios(),
            rom: [0; 0x8000],
            e_ram: [0; 0x2000],
            i_ram: [0; 0x2000],
            s_ram: [0; 0x1E00],
            oam: [0; 0x00A0],
            io: [0; 0x0080],
            high_ram: [0; 0x007F],
            interrupt_register: 0,
            gpu: GPU::new(),
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
                println!("WARNING, no bios loaded {e:}");
                bios[0] = 0x31;
                bios[1] = 0xFE;
                bios[2] = 0xFF;
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
        match addr {
            // BIOS / ROM0
            0x0000..=0xFF => {
                if self.in_bios {
                    self.bios[addr as usize]
                } else {
                    self.rom[addr as usize]
                }
            }
            // ROM0 / switchable ROM bank
            0x0100..=0x7FFF => self.rom[addr as usize],
            // Graphics: VRAM
            0x8000..=0x9FFF => self.gpu.read_vram(addr - 0x8000),
            // External / Switchable RAM
            0xA000..=0xBFFF => self.e_ram[(addr - 0xA000) as usize],
            // Internal / Working Ram
            0xC000..=0xDFFF => self.i_ram[(addr - 0xC000) as usize],
            // Working Ram shadow
            0xE000..=0xFDFF => self.s_ram[(addr - 0xE000) as usize],
            // OAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            // Unused
            0xFEA0..=0xFEFF => 0,
            // I/O Registers
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            // High RAM
            0xFF80..=0xFFFE => self.high_ram[(addr - 0xFF80) as usize],
            // Interrupt Enabled Register
            0xFFFF => self.interrupt_register

            // _ => panic!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            // BIOS / ROM0
            0x0000..=0xFF => {
                if self.in_bios {
                    self.bios[addr as usize] = byte;
                } else {
                    self.rom[addr as usize] = byte;
                }
            }
            // ROM0 / switchable ROM bank
            0x0100..=0x7FFF => self.rom[addr as usize] = byte,
            // Graphics: VRAM
            0x8000..=0x9FFF => self.gpu.write_vram(addr - 0x8000, byte),
            // External / Switchable RAM
            0xA000..=0xBFFF => self.e_ram[(addr - 0xA000) as usize] = byte,
            // Internal / Working Ram
            0xC000..=0xDFFF => {
                let rel_addr = addr - 0xC000;
                self.i_ram[rel_addr as usize] = byte;
                self.s_ram[rel_addr as usize] = byte;
            },
            // Working Ram shadow
            0xE000..=0xFDFF => {
                let rel_addr = addr - 0xE000;
                self.i_ram[rel_addr as usize] = byte;
                self.s_ram[rel_addr as usize] = byte;
            },
            // OAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = byte,
            // Unused
            0xFEA0..=0xFEFF => {},
            // I/O Registers
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = byte,
            // High RAM
            0xFF80..=0xFFFE => self.high_ram[(addr - 0xFF80) as usize] = byte,
            // Interrupt Enabled Register
            0xFFFF => self.interrupt_register = byte

            // _ => panic!(),
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
