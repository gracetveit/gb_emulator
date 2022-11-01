pub struct MemoryBus {
    in_bios: bool,
    memory: [u8; 0xFFFF],
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
        MemoryBus {
            in_bios: true,
            memory: [0;0xFFFF],
            bios: [0;0xFF + 1],
            rom: [0;0x7FFF + 1],
            v_ram: [0; 0x2000],
            e_ram: [0; 0x2000],
            w_ram: [0; 0x5E00],
            z_ram: [0; 0x80],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
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
fn test_read_bios() {
    let test_bus = MemoryBus::new();

    let _ = test_bus.bios[0xFF];
}
