pub struct StatusReg {
    pub status: u8,
}

pub const CARRY_FLAG: u8 = 0b0000_0001;
pub const ZERO_FLAG: u8 = 0b0000_0010;
pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
pub const DECIMAL_MODE: u8 = 0b0000_1000;
pub const BREAK_COMMAND: u8 = 0b0001_0000;
pub const ONE_FLAG: u8 = 0b0010_0000;
pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
pub const NEGATIVE_FLAG: u8 = 0b1000_0000;

impl StatusReg {
    pub fn new() -> Self {
        StatusReg { status: 0 }
    }

    pub fn get_flag(&self, flag: u8) -> u8 {
        self.status & flag
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.status |= flag;
    }

    pub fn unset_flag(&mut self, flag: u8) {
        self.status &= !flag;
    }
}
