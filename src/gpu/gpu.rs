use std::sync::mpsc::{channel, Receiver, Sender, SyncSender};

use crate::{
    cpu::memory_bus::MemoryBus,
    request_response::{Bus, Request},
};

use super::{pixel_fifo::PixelFIFO, sprite::Sprite, tile::Tile};

// #[derive(Debug)]
pub struct GPU {
    mode: GPUMode,
    mode_clock: u16,
    pub line: u8,
    tileset: [Tile; 384],
    vram: [u8; 0x2000],
    visible_sprites: [Option<Sprite>; 10],
    bus: Bus,
    fifo: PixelFIFO,
    pallettes: PalletteCollection,
    temp_lcd: [[[u8; 4]; 160]; 144],
    lcd_sender: Sender<[[[u8; 4]; 160]; 144]>,
    startup: bool
}

impl GPU {
    pub fn new(request_sender: Sender<Request>, lcd_sender: Sender<[[[u8; 4]; 160]; 144]>) -> GPU {
        let cloned_sender = request_sender.clone();
        let pallettes = PalletteCollection {
            background_pallette: Pallette::new(PalletteName::Background),
            sprite_pallette_01: Pallette::new(PalletteName::Sprite01),
            sprite_pallette_02: Pallette::new(PalletteName::Sprite02),
        };
        GPU {
            mode: GPUMode::HBlank,
            mode_clock: 0,
            line: 0,
            tileset: [Tile::new(); 384],
            vram: [0; 0x2000],
            visible_sprites: [None; 10],
            bus: Bus { request_sender }, // map: false,
            fifo: PixelFIFO::new(cloned_sender, [None; 10], pallettes.clone()),
            pallettes,
            temp_lcd: [[[0; 4]; 160]; 144],
            lcd_sender,
            startup: false
        }
    }
    pub fn step(&mut self) -> u8 {
        // Returns relative time
        match self.mode {
            GPUMode::OAMRead => {
                // OAM read mode, scanline active
                // TODO: Low Priority; find way to 'split up' OAM read into smaller steps
                self.oam_search();
                self.fifo.clear(self.visible_sprites, self.pallettes);
                self.mode = GPUMode::PixelTransfer;
                self.mode_clock += 80;
                return 80;
            }
            GPUMode::PixelTransfer => {
                // VRAM read mode, scanline active
                // Treat end of mode 3 as end of scanline
                let line = self.fifo.step(self.temp_lcd[self.line as usize]);
                self.temp_lcd[self.line as usize] = line;
                if self.fifo.x == 160 {
                    self.mode = GPUMode::HBlank;
                }
                // TODO: Find out why mode_clock is adding w/ overflow
                if self.mode_clock > 456 {
                    println!("Warning!");
                }
                self.mode_clock += 1;
                return 1;
            }
            GPUMode::HBlank => {
                // Hblank
                // After the last hblank, push the screen data to canvas
                // if self.mode_clock >= 204 {
                //     self.mode_clock = 0;
                //     self.line += 1;

                //     if self.line == 143 {
                //         // Enter vblank
                //         self.mode = GPUMode::VBlank;
                //         // TODO: Print image to screen
                //     } else {
                //         self.mode = GPUMode::OAMRead;
                //     }
                // }

                // Checks to see if Clock is %456(?) == 0
                // Incrememts time if false, switches to new line / vblank if true

                if self.mode_clock < 456 {
                    self.mode_clock += 1;
                    return 1;
                }

                self.mode_clock = 0;

                if self.line == 143 {
                    self.line = 0;
                    self.mode = GPUMode::VBlank;
                    // TODO: Send temp_LCD to LCD
                    self.lcd_sender.send(self.temp_lcd).unwrap();
                } else {
                    if self.startup {
                        self.startup = false;
                    } else {
                        self.line += 1;
                    }
                    self.mode = GPUMode::OAMRead;
                }
                return 1;
            }
            GPUMode::VBlank => {
                // Vblank (10 lines)
                if self.mode_clock >= 456 {
                    self.mode_clock = 0;
                    self.line += 1;

                    if self.line == 10 {
                        // Restart scanning modes
                        self.mode = GPUMode::OAMRead;
                        self.line = 0;
                    }
                }
                return 1;
            }
        }
    }

    fn oam_search(&mut self) {
        // All visible sprites added to an array

        // Read every entry in OAM
        let sprite_array = self.read_oam();
        let mut i = 0;
        while self.available_sprite_room() && i < 40 {
            if sprite_array[i].x_coordinate != 0 && sprite_array[i].is_visible(self.line) {
                self.push_sprites(sprite_array[i]);
            }
            i += 1;
        }
        // For each entry:
        // If the X position != 0 AND the y_position intersects current_line AND there's an available
        // entry in visible_sprites
        // Add sprite data to visible_sprites
    }

    fn read_oam(&self) -> [Sprite; 40] {
        // requests memory access
        let data = self.bus.read_oam();
        let mut new_sprite_array = [Sprite::from_bytes(0, 0, 0, 0); 40];
        let mut i = 0;
        for sprite in data.chunks_exact(4) {
            while i < 40 {
                new_sprite_array[i] =
                    Sprite::from_bytes(sprite[0], sprite[1], sprite[2], sprite[3]);
                i += 1;
            }
        }
        new_sprite_array
    }

    fn available_sprite_room(&self) -> bool {
        // Checks to see if there is room in the visible sprite array
        match self.visible_sprites[9] {
            None => true,
            Some(_) => false,
        }
    }

    fn clear_sprites(&mut self) {
        self.visible_sprites = [None; 10];
    }

    fn push_sprites(&mut self, sprite: Sprite) {
        // Assumes the last item in visible_sprites is None
        self.visible_sprites.rotate_right(1);
        self.visible_sprites[0] = Some(sprite);
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

        self.tileset[tile_index as usize].update((byte1 as u16) << 8 | byte2 as u16, row_index)
    }

    // fn reset_tileset(&mut self) {
    //     for x in &mut self.tileset {
    //         x.reset()
    //     }
    // }

    // fn render_scan() {
    //     // TODO: Write out renderscan

    //     // Identify which BG Map to use

    //     // Plot pixel to canvas
    //     // Read another tile
    // }
}

#[derive(Debug)]
enum GPUMode {
    OAMRead,
    PixelTransfer,
    HBlank,
    VBlank,
}

// #[cfg(test)]
// use super::tile::Color;
// #[test]
// fn test_write_vram() {
//     let mut test_gpu = GPU::new();

//     test_gpu.write_vram(0x0010, 0xC6);

//     assert!(matches!(test_gpu.tileset[1].parse(0, 0), Color::DG))
// }

#[derive(Clone, Copy)]
struct Color {
    data: [u8; 4],
}

impl Color {
    pub fn new(data: u8) -> Self {
        // Constructs RGBA info from the 2-bit data, and assigns it to the
        // appropriate 2-bit 'slot'

        // Black: 0x081820
        // Dark Grey: 0x346856
        // Light Grey: 0x88c070
        // White: 0xe0f8d0

        let rgba_data: [u8; 4] = match data {
            0 => [0xe0, 0xf8, 0xd0, 0xFF],
            1 => [0x88, 0xc0, 0x70, 0xFF],
            2 => [0x34, 0x68, 0x56, 0xFF],
            3 => [0x08, 0x18, 0x20, 0xFF],
            _ => panic!("data is more complex than 2 bits"),
        };

        Color { data: rgba_data }
    }
}

#[derive(Clone, Copy)]
pub struct Pallette {
    color_11: Color,
    color_10: Color,
    color_01: Color,
    color_00: Color,
    pub name: PalletteName,
}

impl Pallette {
    pub fn new(name: PalletteName) -> Self {
        let placeholder_color = Color::new(0);
        Pallette {
            color_11: placeholder_color,
            color_10: placeholder_color,
            color_01: placeholder_color,
            color_00: placeholder_color,
            name,
        }
    }

    pub fn from_byte(name: PalletteName, data: u8) -> Self {
        let mut temp_pallette: [u8; 4] = [0; 4];
        let mut i = 7;
        let l = 0;
        while l < 4 {
            // TODO: Check that pallette byte is correctly read
            temp_pallette[l] = (((data >> i) & 1) << 1) + ((data >> i - 1) & 1);

            i += 1;
            i -= 2
        }

        Pallette {
            name,
            color_11: Color::new(temp_pallette[0]),
            color_10: Color::new(temp_pallette[1]),
            color_01: Color::new(temp_pallette[2]),
            color_00: Color::new(temp_pallette[3]),
        }
    }

    pub fn return_color(&self, data: u8) -> [u8; 4] {
        match data {
            0 => self.color_00.data,
            1 => self.color_01.data,
            2 => self.color_10.data,
            3 => self.color_11.data,
            _ => panic!("Data is not a 4-bit int"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum PalletteName {
    Background,
    Sprite01,
    Sprite02,
}

#[derive(Clone, Copy)]
pub struct PalletteCollection {
    pub background_pallette: Pallette,
    pub sprite_pallette_01: Pallette,
    pub sprite_pallette_02: Pallette,
}
