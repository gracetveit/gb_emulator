use std::sync::mpsc::Sender;

use crate::request_response::{Bus, Request};

use super::gpu::{Pallette, PalletteCollection, PalletteName};
use super::sprite::Sprite;
pub struct PixelFIFO {
    fifo: [Option<PixelData>; 16],
    t: u8,
    fetcher: Fetcher,
    visible_sprites: [Option<Sprite>; 10],
    pub x: u8,
    pub y: u8,
    // background_pallette: Pallette,
    // sprite_pallette_01: Pallette,
    // sprite_pallette_02: Pallette,
    pallettes: PalletteCollection,
    bg_tile_map_addr: u16,
    window_tile_map_addr: u16,
    scroll: (u8, u8),
    window_pos: (u8, u8),
    window_enable: bool,
    window_mode: bool,
    window_bg_tile_data_area_addr: u16,
}

// TODO: Get fifo to work w/ new pallette object

impl PixelFIFO {
    pub fn new(
        // lcd_sender: Sender<PixelData>,
        request_sender: Sender<Request>,
        // background_pallette: Pallette,
        // sprite_pallette_01: Pallette,
        // sprite_pallette_02: Pallette,
    ) -> Self {
        let pallettes = PalletteCollection {
            background_pallette: Pallette::new(PalletteName::Background),
            sprite_pallette_01: Pallette::new(PalletteName::Sprite01),
            sprite_pallette_02: Pallette::new(PalletteName::Sprite02),
        };
        PixelFIFO {
            fifo: [None; 16],
            // lcd_sender,
            t: 0,
            fetcher: Fetcher::new(
                pallettes.background_pallette.clone(),
                Bus { request_sender },
                0,
            ),
            visible_sprites: [None; 10],
            x: 0,
            y: 0,
            pallettes,
            bg_tile_map_addr: 0,
            window_tile_map_addr: 0,
            window_bg_tile_data_area_addr: 0,
            scroll: (0, 0),
            window_pos: (0, 0),
            window_enable: false,
            window_mode: false,
        }
    }

    pub fn step(&mut self, line: [[u8; 4]; 160]) -> [[u8; 4]; 160] {
        // TODO: if x = 0, discard the first scrollx pixels

        // Check to see if just entered window mode
        if self.check_window_switch() {
            self.window_mode = true;
            // Clear data in FIFO
            self.fifo = [None; 16];
            // reset fetch w/ window map
            self.fetcher.clear();
            self.fetcher.set_map_addr(self.window_tile_map_addr)
        }

        let mut new_line = line;

        new_line = self.push(new_line);

        if self.t % 2 == 0 && self.t <= 4 {
            // Conditional fetch steps on cycle 0, 2, and 4
            self.fetcher.step();
        }

        if self.t == 7 {
            // On last cycle, enqueue data from fetcher
            self.enqueue();
            self.t = 0;
        } else {
            self.t += 1;
        }

        new_line
    }

    fn push(&mut self, line: [[u8; 4]; 160]) -> [[u8; 4]; 160] {
        // !!!
        // FIFO must contain more than 8 pixels to be able to push something out
        // !!!
        if let None = self.fifo[8] {
            return line;
        };
        match self.fifo[0] {
            None => {
                return line;
            }
            Some(pixel_data) => {
                let mut new_line = line;
                new_line[self.x as usize] = pixel_data.to_rgba();
                self.fifo[0] = None;
                self.fifo.rotate_left(1);
                self.x += 1;
                return new_line;
            }
        }
    }

    fn sprite_check(&mut self) -> Option<Sprite> {
        // Checks to see if the next 8 pixels are within a sprite's coordinates

        // Sprite cooordinates start 0x08 pixels to the right, and 0x10 pixels down

        // Returns first sprite and sets it to None in the array
        let mut active_sprite = None;
        let mut i = 0;
        while i < 10 {
            let sprite = self.visible_sprites[i];
            match sprite {
                None => {
                    i += 1;
                }
                Some(sprite) => {
                    if sprite.x_coordinate - 0x08 != self.x {
                        i += 1;
                        continue;
                    }

                    active_sprite = Some(sprite);
                    self.visible_sprites[i] = None;
                    break;
                }
            }
        }
        active_sprite
    }

    fn sprite_overlay(&mut self, sprite_pixels: [PixelData; 8]) {
        for i in 0..9 {
            match self.fifo[i] {
                None => panic!("Attempting to overlay a sprite onto empty pixels"),
                Some(base_pixel) => match base_pixel.pallette.name {
                    PalletteName::Background => {
                        if sprite_pixels[i].data != 0 {
                            self.fifo[i] = Some(sprite_pixels[i]);
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    fn enqueue(&mut self) {
        // Checks to see if the data from self.fifo[0] or self.fifo[8] is None
        let mut i = match (self.fifo[0], self.fifo[8]) {
            (Some(_), Some(_)) => {
                // Do Nothing
                9
            }
            (None, _) => {
                // Fill in starting with i = 0
                0
            }
            (Some(_), None) => {
                // Fill in starting with i = 8
                8
            }
        };
        // In either case, fill each item with the data from the fetcher
        if i != 9 {
            let new_pixel_data = self.fetcher.push();
            let mut l = 0;
            while l < 8 {
                self.fifo[i] = new_pixel_data[l];
                i += 1;
                l += 1;
            }
        }
    }

    // pub fn clear(&mut self) {
    //     self.fifo = [None; 16];
    //     self.t = 0;
    //     self.fetcher.clear();
    // }

    pub fn set_sprites(&mut self, sprite_list: [Option<Sprite>; 10]) {
        self.visible_sprites = sprite_list;
    }

    pub fn set_pallettes(&mut self, pallettes: PalletteCollection) {
        self.pallettes = pallettes;
        self.fetcher
            .set_pallettes(self.pallettes.background_pallette);
    }

    pub fn set_bg_tile_map_addr(&mut self, addr: u16) {
        self.bg_tile_map_addr = addr;
    }

    pub fn set_window_tile_map_addr(&mut self, addr: u16) {
        self.window_tile_map_addr = addr;
    }

    pub fn set_scroll(&mut self, scroll: (u8, u8)) {
        self.scroll = scroll;
    }

    pub fn set_window_pos(&mut self, window_pos: (u8, u8)) {
        self.window_pos = window_pos;
    }

    pub fn set_window_bg_tile_data_area_addr(&mut self, addr: u16) {
        self.window_bg_tile_data_area_addr = addr;
    }

    pub fn set_window_enable(&mut self, enable: bool) {
        self.window_enable = enable;
    }

    pub fn reset_x(&mut self) {
        self.x = 0;
    }

    pub fn inc_y(&mut self) {
        self.y += 1;
    }

    pub fn reset_y(&mut self) {
        self.y = 0;
    }

    fn check_window_switch(&self) -> bool {
        if !self.window_enable {
            return false;
        }

        // If we're already in window mode, we don't need to switch to window mode
        if self.window_mode == true {
            return false;
        }

        let x_check = self.x >= self.window_pos.0;
        let y_check = self.y >= self.window_pos.1;

        if x_check && y_check {
            return true;
        } else {
            return false;
        }
    }

    fn get_current_bg_addr(&self) -> u16 {
        let current_tile_row_addr = ((self.scroll.1 as u16 + self.y as u16) / 8) * 32;

        let tile_area_addr = current_tile_row_addr + ((self.x as u16 + self.scroll.0 as u16) / 8);

        return self.bg_tile_map_addr + tile_area_addr;
    }

    fn get_current_window_addr(&self) -> u16 {
        if self.x < self.window_pos.0 || self.y < self.window_pos.1 {
            panic!("Trying to access window mode when outside window position")
        }

        let window_x = self.x - self.window_pos.0;
        let window_y = self.y - self.window_pos.1;

        // Based upon the assumption the window map is 20x18 tiles

        let i = ((window_y as u16 / 8) * 20) + (window_x as u16 / 8);
        self.window_tile_map_addr + i
    }

    fn get_current_sprite_addr(&mut self, sprite: Sprite) -> u16 {
        // Assumes sprite is visible at the x/y coordinates (checked in step)
        return sprite.tile_number as u16 + 0x8000;
    }
}

struct Fetcher {
    map_addr: u16,
    tile_num: Option<u8>,
    data_0: Option<u8>,
    data_1: Option<u8>,
    pallette: Pallette,
    bus: Bus,
    // True: 0x8000 method, False:: 0x8800 method
    data_starting_addr: u16,
}

impl Fetcher {
    pub fn new(pallette: Pallette, bus: Bus, map_addr: u16) -> Self {
        // Constructed every new fetch
        Fetcher {
            map_addr,
            tile_num: None,
            data_0: None,
            data_1: None,
            pallette,
            bus,
            data_starting_addr: 0,
        }
    }

    pub fn set_pallettes(&mut self, pallete: Pallette) {
        self.pallette = pallete;
    }

    pub fn clear(&mut self) {
        self.tile_num = None;
        self.data_0 = None;
        self.data_1 = None;
    }

    pub fn set_map_addr(&mut self, map_addr: u16) {
        self.map_addr = map_addr;
    }

    pub fn step(&mut self) {
        // match self.tile_num {
        //     None => {
        //         // TODO: Fetch tile_addr

        //         // Read Background Map based on window x,y and lcd x, y(line)
        //     }
        //     Some(tile_addr) => match self.data_0 {
        //         None => {
        //             // TODO: Fetch data_0
        //         }
        //         Some(data_0) => match self.data_1 {
        //             None => {
        //                 // TODO: Fetch data_1
        //             }
        //             Some(data_1) => {
        //                 // TOPDO: Returns Pixel Data
        //             }
        //         },
        //     },
        // }
    }

    pub fn push(&mut self) -> [Option<PixelData>; 8] {
        match (self.data_0, self.data_1) {
            (Some(data_0), Some(data_1)) => {
                let pixel_data = self.construct_pixel_data(data_0, data_1);
                self.clear();
                pixel_data
            }
            _ => [None; 8],
        }
    }

    fn construct_pixel_data(&self, data_0: u8, data_1: u8) -> [Option<PixelData>; 8] {
        let mut i = 7;
        let mut l = 0;
        let mut return_data: [Option<PixelData>; 8] = [None; 8];
        while i <= 0 {
            // TODO: Check to make sure that data_0 is the 2nd significant digit
            let data = (((data_0 >> i) & 1) << 1) + ((data_1 >> i) & 1);

            return_data[l] = Some(PixelData {
                data,
                pallette: self.pallette,
            });

            i -= 1;
            l += 1;
        }
        return_data
    }

    fn set_tile_num(&mut self, data: u8) {
        self.tile_num = Some(data);
    }

    fn set_addressing_method(&mut self, starting_address: u16) {
        match starting_address {
            0x8000 => {
                self.data_starting_addr = 0x8000;
            }
            0x8800 => {
                self.data_starting_addr = 0x8800;
            }
            _ => {
                panic!("Incorrect Starting Address, 0x{starting_address:x} is not 0x8000 or 0x8800")
            }
        }
    }

    fn get_tile_data_addr(&self) -> u16 {
        match (self.data_starting_addr, self.tile_num) {
            (0x8800, Some(tile_num)) => {
                let signed_tile_num = ((tile_num as i16) as i8) as i32;
                return (36864 + (signed_tile_num * 16)) as u16;
            }
            (0x8000, Some(tile_num)) => {
                return 0x8000 + (tile_num as u16 * 16);
            }
            _ => {
                let starting_addr = self.data_starting_addr;
                let tile_num = self.tile_num;
                panic!("Mismatch between starting_addr and tile_num\n starting_addr: {starting_addr:x}\ntile_num: {tile_num:?}")
            }
        }
    }
}

enum FetchMode {
    Background,
    Sprite,
}

#[derive(Clone, Copy)]
pub struct PixelData {
    data: u8,
    pallette: Pallette,
}

impl PixelData {
    fn to_rgba(&self) -> [u8; 4] {
        self.pallette.return_color(self.data)
    }
}

// #[derive(Clone, Copy, Debug)]
// enum TileNum {
//     Signed(i8),
//     Unsigned(u8)
// }

// #[derive(Clone, Copy)]
// enum Pallette {
//     Background,
//     Sprite01,
//     Sprite02,
// }

#[cfg(test)]
fn create_fifo() -> PixelFIFO {
    use std::sync::mpsc::channel;

    let (request_sender, _) = channel();
    PixelFIFO::new(request_sender)
}

#[cfg(test)]
fn create_fetcher() -> Fetcher {
    use std::sync::mpsc::channel;

    let (request_sender, _) = channel();
    Fetcher::new(
        Pallette::new(PalletteName::Background),
        Bus { request_sender },
        0x9800,
    )
}

#[test]
fn test_get_bg_addr() {
    // Yes, I know these aren't the actual tile_map_area addresses, I don't care
    let mut fifo = create_fifo();
    fifo.set_bg_tile_map_addr(0x8000);

    let mut addr = fifo.get_current_bg_addr();
    assert!(addr == 0x8000, "{addr:x} is not 0x8000");

    fifo.set_scroll((46, 95));
    addr = fifo.get_current_bg_addr();
    assert!(addr == 0x8165, "{addr:x} is not 0x8165");
}

#[test]
fn test_get_window_addr() {
    let mut fifo = create_fifo();
    fifo.set_window_tile_map_addr(0x9800);

    let mut addr = fifo.get_current_window_addr();
    assert!(addr == 0x9800, "{addr:x} is not 0x9800");

    fifo.x = 32;
    fifo.y = 64;

    fifo.set_window_pos((24, 16));

    addr = fifo.get_current_window_addr();
    assert!(addr == 0x9879, "0x{addr:x} is not 0x9879");
}

#[test]
fn test_get_srpite_addr() {
    let mut fifo = create_fifo();

    let sprite = Sprite::from_bytes(0x10, 0x8, 0x30, 0b10000000);

    let addr = fifo.get_current_sprite_addr(sprite);
    assert!(addr == 0x8030, "0x{addr:x} is not 0x8030");
}

// #[test]
// fn test_setting_tile_num() {
//     let mut fetcher = create_fetcher();

//     fetcher.set_addressing_method(0x8800);
//     fetcher.set_tile_num(0x9F);
//     if let Some(TileNum::Signed(ans)) = fetcher.tile_num {
//         assert!(ans == -97, "{ans} is not -97")
//     } else {
//         panic!("Type error: Tile_num")
//     };
// }

#[test]
fn test_getting_correct_data_addr_from_tile_num() {
    let mut fetcher = create_fetcher();

    fetcher.set_addressing_method(0x8800);
    fetcher.set_tile_num(0x9F);
    let ans = fetcher.get_tile_data_addr();

    assert!(ans == 0x89F0, "0x{ans:x} is not 0x89F0");

    fetcher.set_tile_num(0x28);
    let ans = fetcher.get_tile_data_addr();

    assert!(ans == 0x9280, "0x{ans:x} is not 0x9280");
}
