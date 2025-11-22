use super::Reg;

pub struct LinearFeedback {
    data: u8,
}

pub const NOISE_MODE_MASK: u8 = 0b1000_0000;
pub const NOISE_PERIOD_MASK: u8 = 0b0000_1111;

impl LinearFeedback {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

impl Reg for LinearFeedback {
    fn data(&self) -> u8 {
        self.data
    }

    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
}
