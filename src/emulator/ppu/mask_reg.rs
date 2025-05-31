pub struct MaskReg {
    flags: u8,
}

pub const GREY_SCALE_FLAG: u8 = 0b0000_0001;
pub const SHOW_BACKGROUND_IN_LEFT_8_PIXELS_FLAG: u8 = 0b0000_0010;
pub const SHOW_SPRITES_IN_LEFT_8_PIXELS_FLAG: u8 = 0b0000_0100;
pub const ENABLE_BACKGROUND_RENDERING_FLAG: u8 = 0b0000_1000;
pub const ENABLE_SPRITE_RENDERING_FLAG: u8 = 0b0001_0000;
pub const EMPHASIZE_RED_FLAG: u8 = 0b0010_0000;
pub const EMPHASIZE_GREEN_FLAG: u8 = 0b0100_0000;
pub const EMPHASIZE_BKUE_FLAG: u8 = 0b1000_0000;

impl MaskReg {
    pub fn new() -> Self {
        MaskReg { flags: 0 }
    }

    pub fn get(&self) -> u8 {
        self.flags
    }

    pub fn update(&mut self, value: u8) {
        self.flags = value;
    }

    pub fn show_sprites(&self) -> bool {
        (self.flags & ENABLE_SPRITE_RENDERING_FLAG) != 0
    }
}
