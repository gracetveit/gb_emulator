pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0;0xFFFF]
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
