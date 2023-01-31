use std::sync::mpsc::Sender;

use super::sprite::Sprite;
struct PixelFIFO {
    fifo: [Option<PixelData>; 16],
    lcd_sender: Sender<PixelData>,
    t: u8,
    fetcher: Fetcher,
    visible_sprites: [Option<Sprite>; 10],
    x: u8,
}

impl PixelFIFO {
    pub fn new(
        lcd_sender: Sender<PixelData>,
        memory_sender: Sender<Sender<u8>>,
        visible_sprites: [Option<Sprite>; 10],
    ) -> Self {
        // Constructed every new line
        PixelFIFO {
            fifo: [None; 16],
            lcd_sender,
            t: 0,
            fetcher: Fetcher::new(FetchMode::Background, memory_sender),
            visible_sprites,
            x: 0,
        }
    }

    pub fn step(&mut self) {
        // TODO: Outline Step Function
    }

    fn sprite_check(&mut self) -> Option<&Sprite> {
        // Checks to see if the nexr 8 pixels are within a sprite's coordinates

        // Sprite cooordinates start 0x08 pixels to the right, and 0x10 pixels down
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

                    active_sprite = Some(&sprite);
                    self.visible_sprites[i] = None;
                    break;
                }
            }
        }
        active_sprite
    }
}

struct Fetcher {
    tile_addr: Option<u8>,
    data_0: Option<u8>,
    data_1: Option<u8>,
    mode: FetchMode,
    sender: Sender<Sender<u8>>,
}

impl Fetcher {
    pub fn new(mode: FetchMode, sender: Sender<Sender<u8>>) -> Self {
        // Constructed every new fetch
        Fetcher {
            tile_addr: None,
            data_0: None,
            data_1: None,
            mode,
            sender,
        }
    }

    pub fn clear(&mut self, mode: FetchMode) {
        self.tile_addr = None;
        self.data_0 = None;
        self.data_1 = None;
        self.mode = mode
    }

    pub fn step(&mut self) -> Option<[PixelData; 8]> {
        match self.tile_addr {
            None => {
                // TODO: Fetch tile_addr
                None
            }
            Some(tile_addr) => match self.data_0 {
                None => {
                    // TODO: Fetch data_0
                    None
                }
                Some(data_0) => match self.data_1 {
                    None => {
                        // TODO: Fetch data_1
                        None
                    }
                    Some(data_1) => {
                        // TOPDO: Returns Pixel Data
                        Some(self.construct_pixel_data(data_0, data_1))
                    }
                },
            },
        }
    }

    fn construct_pixel_data(&self, data_0: u8, data_1: u8) -> [PixelData; 8] {
        // TODO: write out construct pixel_data method
        todo!()
    }
}

enum FetchMode {
    Background,
    Sprite,
}

#[derive(Clone, Copy)]
struct PixelData {
    data: u8,
    pallette: Pallette,
}

#[derive(Clone, Copy)]
enum Pallette {
    Background,
    Sprite01,
    Sprite02,
}
