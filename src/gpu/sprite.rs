#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub y_coordinate: u8,
    pub x_coordinate: u8,
    pub tile_number: u8,
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette: bool,
}

impl Sprite {
    pub fn from_bytes(y_coordinate: u8, x_coordinate: u8, tile_number: u8, flags: u8) -> Sprite {
        let priority = flags >> 7 & 1 == 1;
        let y_flip = flags >> 6 & 1 == 1;
        let x_flip = flags >> 5 & 1 == 1;
        let palette = flags >> 4 & 1 == 1;
        Sprite {
            y_coordinate,
            x_coordinate,
            tile_number,
            priority,
            y_flip,
            x_flip,
            palette,
        }
    }

    pub fn is_visible(&self, current_line: u8) -> bool {
        current_line + 16 >= self.y_coordinate && current_line + 16 < self.y_coordinate + 8
        // todo!()
    }
}

#[test]
fn test_sprite_from_bytes() {
    let y_coordinate = 0;
    let x_coordinate = 0;
    let tile_number = 0;
    let flags = 0b10100000;

    let sprite = Sprite::from_bytes(y_coordinate, x_coordinate, tile_number, flags);

    assert_eq!(sprite.priority, true);
    assert_eq!(sprite.y_flip, false);
    assert_eq!(sprite.x_flip, true);
    assert_eq!(sprite.palette, false)
}
