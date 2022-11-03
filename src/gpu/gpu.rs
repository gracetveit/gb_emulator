use super::tile::Tile;

pub struct GPU {
    mode: GPUMode,
    mode_clock: u16,
    line: u8,
    tileset: [Tile; 384],
    vram: [u8; 0x2000],
}

impl GPU {
    pub fn step(&mut self, t: u16) {
        self.mode_clock += t;
        match self.mode {
            GPUMode::OAMRead => {
                // OAM read mode, scanline active
                if self.mode_clock >= 80 {
                    // Enter scanline mode 3: (reading VRAM)
                    self.mode_clock = 0;
                    self.mode = GPUMode::VRAMRead;
                }
            }
            GPUMode::VRAMRead => {
                // VRAM read mode, scanline active
                // Treat end of mode 3 as end of scanline
                if self.mode_clock >= 172 {
                    // enter hblank
                    self.mode_clock = 0;
                    self.mode = GPUMode::HBlank;
                }
            }
            GPUMode::HBlank => {
                // Hblank
                // After the last hblank, push the screen data to canvas
                if self.mode_clock >= 204 {
                    self.mode_clock = 0;
                    self.line += 1;

                    if self.line == 143 {
                        // Enter vblank
                        self.mode = GPUMode::VBlank;
                        // TODO: Print image to screen
                    } else {
                        self.mode = GPUMode::OAMRead;
                    }
                }
            }
            GPUMode::VBlank => {
                // Vblank (10 lines)
                if self.mode_clock >= 456 {
                    self.mode_clock = 0;
                    self.line += 1;

                    if self.line > 153 {
                        // Restart scanning modes
                        self.mode = GPUMode::OAMRead;
                        self.line = 0;
                    }
                }
            }
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_vram(&mut self, index: u16, value: u8) {
        self.vram[index as usize] = value;

        // If our index is greater than 0x1800, we're not writing to the tile
        // set storage, so we can just return
        if index >= 0x1800 {
            return;
        }

        // Tile rows are encoded in two bytes with the first byte always on an
        // even address. Bitwise ANDing the address with 0x0FFFE gives us the
        // address of the first byte.
        // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
        let normalized_index = index & 0xFFFE;

        // First we need to get the two bytes that encode the tile row.
        let byte1 = self.vram[normalized_index as usize];
        let byte2 = self.vram[normalized_index as usize + 1];

        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        self.tileset[tile_index as usize].update({ (byte1 as u16) << 8 | byte2 as u16 }, row_index)
    }

    fn reset_tileset(&mut self) {
        for x in &mut self.tileset {
            x.reset()
        }
    }
}

enum GPUMode {
    OAMRead,
    VRAMRead,
    HBlank,
    VBlank,
}

#[cfg(test)]
use super::tile::Color;
#[test]
fn test_write_vram() {
    let mut test_gpu = GPU {
        mode: GPUMode::OAMRead,
        mode_clock: 0,
        line: 0,
        tileset: [Tile::new(); 384],
        vram: [0; 0x2000],
    };

    test_gpu.write_vram(0x0010, 0xC6);

    assert!(matches!(test_gpu.tileset[1].parse(0, 0), Color::DG))
}
