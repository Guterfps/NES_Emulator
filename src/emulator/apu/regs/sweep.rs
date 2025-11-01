use super::Reg;

pub struct Sweep {
    data: u8,
}

pub const ENABLE_MASK: u8 = 0b1000_0000;
pub const PERIOD_MASK: u8 = 0b0111_0000;
pub const NEGATE_MAKS: u8 = 0b0000_1000;
pub const SHIFT_MASK: u8 = 0b0000_0111;

impl Reg for Sweep {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
