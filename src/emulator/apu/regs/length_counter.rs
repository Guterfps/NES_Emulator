use super::Reg;

pub struct LengthCounter {
    data: u8,
}

pub const LENGTH_LOAD_MASK: u8 = 0b1111_1000;
pub const TIMER_HIGH_MASK: u8 = 0b0000_0111;

impl LengthCounter {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

impl Reg for LengthCounter {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
