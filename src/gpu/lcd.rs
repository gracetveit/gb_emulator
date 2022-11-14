use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::hello_world_pixels::HWLetter;
// use super::pixel::Pixel;

const WIDTH: u8 = 160;
const HEIGHT: u8 = 144;

pub struct LCD {
    pixels: Pixels,
    // i: usize,
}

impl LCD {
    pub fn new(window: &Window) -> LCD {
        let size = window.inner_size();

        let surface_texture = SurfaceTexture::new(size.width, size.height, window);

        let pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

        LCD { pixels, /*i: 0*/ }
    }

    // pub fn push(&mut self, data: Pixel) {
    //     let frame = self.pixels.get_frame_mut();
    //     for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
    //         if i == self.i {
    //             pixel.copy_from_slice(&data.export());
    //         }
    //     }
    //     self.iterate();
    // }

    // fn iterate(&mut self) {
    //     if self.i + 1 == 23040 {
    //         self.i = 0;
    //     } else {
    //         self.i += 1;
    //     }
    // }

    pub fn render(&self) {
        self.pixels.render().unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.pixels.resize_surface(size.width, size.height);
        self.render();
    }

    pub fn hello_world(&mut self) {
        let test_string = HWLetter::from_string("hello world");

        let frame = self.pixels.get_frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let char_id = (i % 160) / 8;
            let relative_y = i / 160;
            let relative_x = i % 8;
            let black: [u8; 4] = [0x08, 0x18, 0x20, 0xFF];
            let white: [u8; 4] = [0xE0, 0xF8, 0xD0, 0xFF];

            if (char_id < 11) && (relative_y < 8) {
                // Find data for the character/pixel
                let data = &test_string[char_id].data[relative_y][relative_x];

                pixel.copy_from_slice(match data {
                    1 => &black,
                    _ => &white
                });
            }

            else {
                pixel.copy_from_slice(&white);
            }

        }
    }
}
