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

    pub fn step(&mut self, line: [[u8; 4]; 160]) -> [[u8; 4]; 160] {
        // TODO: Outline Step Function
        let mut new_line = line;
        // Push
        if self.t % 2 == 0 && self.t <= 4 {
            // Conditional fetch steps
            self.fetcher.step();
        }

        if self.t == 7 {
            self.t = 0;
        } else {
            self.t += 1;
        }

        new_line
    }

    fn push(&mut self) -> Option<[u8; 4]> {
        // !!!
        // FIFO must contain more than 8 pixels to be able to push something out
        // !!!
        if let None = self.fifo[8] {
            return None;
        };
        match self.fifo[0] {
            None => {
                // None
                todo!()
            }
            Some(pixel_data) => {
                // TODO: convert pixel data & pallette info into actual RGBA
                self.fifo[0] = None;
                self.fifo.rotate_left(1);
                todo!()

            }
        }
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

    fn sprite_overlay(&mut self, sprite_pixels: [PixelData; 8]) {
        for i in 0..9 {
            match self.fifo[i] {
                None => panic!("Attempting to overlay a sprite onto empty pixels"),
                Some(base_pixel) => match base_pixel.pallette {
                    Pallette::Background => {
                        if sprite_pixels[i].data != 0 {
                            self.fifo[i] = Some(sprite_pixels[i]);
                        }
                    }
                    _ => {}
                },
            }
        }
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

    pub fn step(&mut self) {
        match self.tile_addr {
            None => {
                // TODO: Fetch tile_addr
            }
            Some(tile_addr) => match self.data_0 {
                None => {
                    // TODO: Fetch data_0
                }
                Some(data_0) => match self.data_1 {
                    None => {
                        // TODO: Fetch data_1
                    }
                    Some(data_1) => {
                        // TOPDO: Returns Pixel Data
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
