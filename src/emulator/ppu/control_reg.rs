pub struct ControlReg {
    flags: u8,
}

pub const NAME_TABLE_FLAGS: u8 = 0b0000_0011;
pub const VRAM_ADDR_INC_FLAG: u8 = 0b0000_0100;
pub const SPRITE_PATTERN_ADDR: u8 = 0b0000_1000;
pub const BACKGROUND_PATTERN_ADDR_FLAG: u8 = 0b0001_0000;
pub const SPRITE_SIZE_FLAG: u8 = 0b0010_0000;
pub const MASTER_SLAVE_FLAG: u8 = 0b0100_0000;
pub const GENERATE_NMI_FLAG: u8 = 0b1000_0000;

#[repr(u8)]
pub enum AddressInc {
    GoingAcross = 1,
    GoingDown = 32,
}

impl ControlReg {
    pub fn new() -> Self {
        ControlReg { flags: 0 }
    }

    pub fn vram_addr_increment(&self) -> AddressInc {
        if (self.flags & VRAM_ADDR_INC_FLAG) != 0 {
            AddressInc::GoingDown
        } else {
            AddressInc::GoingAcross
        }
    }

    pub fn update(&mut self, data: u8) {
        self.flags = data;
    }
}
