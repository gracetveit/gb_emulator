struct GPU {
    mode: GPUMode,
    mode_clock: u16,
    line: u8,
}

impl GPU {
    fn step(&mut self, t: u16) {
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
}

enum GPUMode {
    OAMRead,
    VRAMRead,
    HBlank,
    VBlank,
}
