pub struct StatusReg {
    flags: u8,
}

pub const PPU_OPEN_BUS: u8 = 0b0001_1111;
pub const SPRITE_OVERFLOW_FLAG: u8 = 0b0010_0000;
pub const SPRITE_0_HIT_FLAG: u8 = 0b0100_0000;
pub const VBLANK_FLAG: u8 = 0b1000_0000;

impl StatusReg {
    pub fn new() -> Self {
        StatusReg { flags: 0 }
    }

    pub fn get(&mut self) -> u8 {
        let curr_stat = self.flags;
        self.reset_vblank();
        curr_stat
    }

    pub fn update(&mut self, value: u8) {
        self.flags = value;
    }

    pub fn set_vblank(&mut self) {
        self.flags |= VBLANK_FLAG;
    }

    pub fn reset_vblank(&mut self) {
        self.flags &= !VBLANK_FLAG;
    }

    pub fn unset_sprite_zero_hit(&mut self) {
        self.flags &= !SPRITE_0_HIT_FLAG;
    }

    pub fn set_sprite_zero_hit(&mut self) {
        self.flags |= SPRITE_0_HIT_FLAG;
    }

    pub fn is_in_vblank(&self) -> bool {
        (self.flags & VBLANK_FLAG) != 0
    }

    pub fn is_sprite_0_hit(&self) -> bool {
        (self.flags & SPRITE_0_HIT_FLAG) != 0
    }
}
