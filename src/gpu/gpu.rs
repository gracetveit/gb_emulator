use std::sync::mpsc::{channel, Sender, SyncSender};

use super::{sprite::Sprite, tile::Tile};

#[derive(Debug)]
pub struct GPU {
    mode: GPUMode,
    mode_clock: u16,
    pub line: u8,
    tileset: [Tile; 384],
    vram: [u8; 0x2000],
    visible_sprites: [Option<Sprite>; 10],
    oam_sender: SyncSender<Sender<[u8; 160]>>, // map: bool,
}

impl GPU {
    pub fn new(oam_sender: SyncSender<Sender<[u8; 160]>>) -> GPU {
        GPU {
            mode: GPUMode::HBlank,
            mode_clock: 0,
            line: 0,
            tileset: [Tile::new(); 384],
            vram: [0; 0x2000],
            visible_sprites: [None; 10],
            oam_sender, // map: false,
        }
    }
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

    fn oam_search(&mut self) {
        // All visible sprites added to an array

        // Read every entry in OAM
        let sprite_array = self.read_oam();
        let mut i = 0;
        while self.available_sprite_room() && i < 40 {
            if sprite_array[i].x_coordinate != 0 && sprite_array[0].is_visible(self.line) {
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
        // TODO: Write out memory-side receiver/sender logic
        // requests memory access
        let (oam_sender, oam_receiver) = channel::<[u8; 160]>();
        self.oam_sender.send(oam_sender);
        match oam_receiver.recv() {
            Ok(data) => {
                let mut new_sprite_array = [Sprite::from_bytes(0, 0, 0, 0); 40];
                let mut i = 0;
                for sprite in data.chunks_exact(4) {
                    new_sprite_array[i] =
                        Sprite::from_bytes(sprite[0], sprite[1], sprite[2], sprite[3]);
                    i += 1;
                }
                new_sprite_array
            }
            Err(error) => panic!("{error:?}"),
        }
    }

    fn available_sprite_room(&self) -> bool {
        // Checks to see if there is room in the visible sprite array
        match self.visible_sprites[19] {
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
    VRAMRead,
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

struct ColorPallette {
    color_11: Color,
    color_10: Color,
    color_01: Color,
    color_00: Color,
    name: PalletteName,
}

impl ColorPallette {
    pub fn new(name: PalletteName) -> Self {
        let placeholder_color = Color::new(0);
        ColorPallette {
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

        ColorPallette {
            name,
            color_11: Color::new(temp_pallette[0]),
            color_10: Color::new(temp_pallette[1]),
            color_01: Color::new(temp_pallette[2]),
            color_00: Color::new(temp_pallette[3])
        }
    }
}

enum PalletteName {
    Background,
    Sprite01,
    Sprite02,
}
